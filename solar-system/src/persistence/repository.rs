//! 数据库的请求/返回的业务逻辑

use diesel::prelude::*;

use crate::persistence::connection::Conn;
use crate::persistence::model::{PlanetEntity, SatelliteEntity};
use crate::persistence::schema::planets;

// 仅获取行星名称
pub fn get_names(conn: &Conn) -> QueryResult<Vec<String>> {
    let names = planets::table.select(planets::name).load(conn)?;
    Ok(names)
}

// 获取所有行星以及关联卫星
pub fn get_all(conn: &Conn) -> QueryResult<Vec<(PlanetEntity, Vec<SatelliteEntity>)>> {
    let planets: Vec<PlanetEntity> = planets::table.load(conn)?;
    let satellites = SatelliteEntity::belonging_to(&planets)
        .load(conn)?
        .grouped_by(&planets);

    let result = planets.into_iter().zip(satellites).collect::<Vec<_>>();

    Ok(result)
}

// 由名称获取行星及其关联卫星
pub fn get_by_name(name: &str, conn: &Conn) -> QueryResult<(PlanetEntity, Vec<SatelliteEntity>)> {
    let planet: PlanetEntity = planets::table
        .filter(planets::name.ilike(name))
        .first(conn)?;
    let satellites = SatelliteEntity::belonging_to(&planet).load(conn)?;

    Ok((planet, satellites))
}
