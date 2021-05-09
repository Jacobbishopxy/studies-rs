use crate::models;
use std::convert::Infallible;
use warp::http::StatusCode;

pub async fn handle_list_sims(
    opt: Option<u64>,
    db: models::Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut result = db.lock().await.clone();

    if let Some(param) = opt {
        // `retain` 仅保留 HashSet 中符合闭包条件的元素
        result.retain(|k| k.id == param);
    }

    Ok(warp::reply::json(&result))
}

/// 由 `filter` 的 `and_then` 触发
/// 具体的业务逻辑
pub async fn handle_create_sim(
    sim: models::Simulation,
    db: models::Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut map = db.lock().await;

    if let Some(result) = models::get_simulation(&*map, sim.id) {
        return Ok(warp::reply::with_status(
            format!(
                "Simulation #{} already exists under the name {}. \n",
                result.id, result.name
            ),
            StatusCode::BAD_REQUEST,
        ));
    }

    map.insert(sim.clone());
    Ok(warp::reply::with_status(
        format!("Simulation #{} created. \n", sim.id),
        StatusCode::CREATED,
    ))
}

pub async fn handle_update_sim(
    id: u64,
    new: models::NewName,
    db: models::Db,
) -> Result<impl warp::Reply, Infallible> {
    // Replaced entry
    if let Some(_) = db
        .lock()
        .await
        .replace(models::Simulation { id, name: new.name })
    {
        return Ok(warp::reply::with_status(
            format!("Simulation #{} was updated. \n", id),
            StatusCode::OK,
        ));
    };

    // Create entry
    Ok(warp::reply::with_status(
        format!("Simulation #{} was inserted. \n", id),
        StatusCode::CREATED,
    ))
}

pub async fn handle_delete_sim(id: u64, db: models::Db) -> Result<impl warp::Reply, Infallible> {
    if db.lock().await.remove(&models::Simulation {
        id,
        name: String::new(),
    }) {
        return Ok(warp::reply::with_status(
            format!("Simulation #{} was deleted. \n", id),
            StatusCode::OK,
        ));
    }

    Ok(warp::reply::with_status(
        format!("No data was deleted. \n"),
        StatusCode::OK,
    ))
}
