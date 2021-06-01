#[macro_use]
extern crate diesel;

pub mod persistence;

pub mod soloar_system_info {
    tonic::include_proto!("solar_system_info");
}
