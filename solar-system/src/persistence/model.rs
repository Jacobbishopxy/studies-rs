//! 将数据库的 schema 转换为语言上的数据结构

use bigdecimal::BigDecimal;
use chrono::NaiveDate;

use crate::persistence::schema::{planets, satellites};

#[derive(Identifiable, Queryable)]
#[table_name = "planets"]
pub struct PlanetEntity {
    pub id: i32,
    pub name: String,
    pub type_: String,
    pub mean_radius: BigDecimal,
    pub mass: BigDecimal,
}

// 与 planets 有多对一的关联关系
#[derive(Identifiable, Queryable, Associations)]
#[table_name = "satellites"]
#[belongs_to(PlanetEntity, foreign_key = "planet_id")]
pub struct SatelliteEntity {
    pub id: i32,
    pub name: String,
    pub first_spacecraft_landing_date: Option<NaiveDate>,
    pub planet_id: i32,
}
