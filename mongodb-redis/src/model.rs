use std::str::FromStr;

use chrono::Utc;
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::dto::{PlanetDto, SatelliteDto};

#[derive(Serialize, Deserialize, Debug)]
pub struct Planet {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub r#type: PlanetType,
    pub mean_radius: f32,
    pub satellites: Option<Vec<Satellite>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum PlanetType {
    TerrestrialPlanet,
    GasGiant,
    IceGiant,
    DwarfPlanet,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Satellite {
    pub name: String,
    pub first_spacecraft_landing_date: Option<DateTime>,
}

impl From<PlanetDto> for Planet {
    fn from(source: PlanetDto) -> Self {
        Planet {
            id: source
                .id
                .map(|id| ObjectId::from_str(id.as_str()).expect("Can't convert to ObjectId")),
            name: source.name,
            r#type: source.r#type,
            mean_radius: source.mean_radius,
            satellites: source
                .satellites
                .map(|satellites| satellites.into_iter().map(Satellite::from).collect()),
        }
    }
}

impl From<SatelliteDto> for Satellite {
    fn from(source: SatelliteDto) -> Self {
        Satellite {
            name: source.name,
            first_spacecraft_landing_date: source.first_spacecraft_landing_date.map(|d| {
                mongodb::bson::DateTime::from_millis(
                    chrono::Date::<Utc>::from_utc(d, Utc)
                        .and_hms(0, 0, 0)
                        .timestamp_millis(),
                )
            }),
        }
    }
}

impl std::fmt::Display for PlanetType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
