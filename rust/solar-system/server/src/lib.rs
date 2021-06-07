//! 实现具体 gRPC 业务逻辑
//!
//! 1. 使用定义在 repository 中与数据库交互的函数
//! 2. 实现由 build.rs 从 proto 中自动生成的 gRPC 接口

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

pub mod conversion;
pub mod persistence;

use diesel::result::Error;
use futures::Stream;
use log::{debug, error};
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use tonic::{Request, Response, Status};

use crate::conversion::PlanetWrapper;
use crate::persistence::{Conn, PgPool};
use solar_system_rpc::solar_system_info::solar_system_info_server::SolarSystemInfo;
use solar_system_rpc::solar_system_info::{
    Planet, PlanetRequest, PlanetResponse, PlanetsListResponse,
};

embed_migrations!();

pub fn run_migrations(pool: &PgPool) {
    let conn = pool.get().expect("Can't get DB connection");
    embedded_migrations::run(&conn).expect("Failed to run database migrations");
}

pub struct SolarSystemInfoService {
    pub pool: PgPool,
}

impl SolarSystemInfoService {
    fn get_connection(&self) -> Conn {
        self.pool.get().expect("Can't get DB connection")
    }
}

#[tonic::async_trait]
impl SolarSystemInfo for SolarSystemInfoService {
    type GetPlanetsStream =
        Pin<Box<dyn Stream<Item = Result<PlanetResponse, Status>> + Send + Sync + 'static>>;

    async fn get_planets_list(
        &self,
        request: Request<()>,
    ) -> Result<Response<PlanetsListResponse>, Status> {
        debug!("Got a request: {:?}", request);

        let names_of_planets = persistence::repository::get_names(&self.get_connection())
            .expect("Can't get names of the planets");

        let reply = PlanetsListResponse {
            list: names_of_planets,
        };

        Ok(Response::new(reply))
    }

    async fn get_planets(
        &self,
        request: Request<()>,
    ) -> Result<Response<Self::GetPlanetsStream>, Status> {
        debug!("Got a request: {:?}", request);

        let (tx, rx) = mpsc::channel(4);

        let planets: Vec<Planet> = persistence::repository::get_all(&self.get_connection())
            .expect("Can't load planets")
            .into_iter()
            .map(|p| {
                PlanetWrapper {
                    planet: p.0,
                    satellites: p.1,
                }
                .into()
            })
            .collect();

        tokio::spawn(async move {
            let mut stream = tokio_stream::iter(&planets);

            while let Some(planet) = stream.next().await {
                tx.send(Ok(PlanetResponse {
                    planet: Some(planet.clone()),
                }))
                .await
                .unwrap();
            }
        });

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx),
        )))
    }

    async fn get_planet(
        &self,
        request: Request<PlanetRequest>,
    ) -> Result<Response<PlanetResponse>, Status> {
        debug!("Got a request: {:?}", request);

        let planet_name = request.into_inner().name;

        let planet = persistence::repository::get_by_name(&planet_name, &self.get_connection());

        match planet {
            Ok(planet) => {
                let planet = PlanetWrapper {
                    planet: planet.0,
                    satellites: planet.1,
                }
                .into();

                let reply = PlanetResponse {
                    planet: Some(planet),
                };

                Ok(Response::new(reply))
            }
            Err(e) => {
                error!(
                    "There was an error while getting a planet {}: {}",
                    &planet_name, e
                );
                match e {
                    Error::NotFound => Err(Status::not_found(format!(
                        "Planet with name {} not found",
                        &planet_name
                    ))),
                    _ => Err(Status::unknown(format!(
                        "There was an error while getting a planet {}: {}",
                        &planet_name, e
                    ))),
                }
            }
        }
    }
}
