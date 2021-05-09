use crate::handlers;
use crate::models;
use warp::Filter;

/// 私有函数
/// 用于限定最大长度，提取一个已解码的 JSON 的 `future`，返回 `filter`
fn json_body() -> impl Filter<Extract = (models::Simulation,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn json_body_put() -> impl Filter<Extract = (models::NewName,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

/// 列举所有的 Simulation
pub fn list_sims(
    db: models::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let db_map = warp::any().map(move || db.clone());

    // 代表可以被发送的可选参数（从 URL 中获得）
    // 如果（URL 中）没有提供该参数，则由 `or_else` 返回 None
    // `or_else` 中的闭包需要用到 `async` 是因为 `or_else` 返回的是一个 TryFuture
    let opt = warp::path::param::<u64>()
        .map(Some)
        .or_else(|_| async { Ok::<(Option<u64>,), std::convert::Infallible>((None,)) });

    // `path!` 宏里面的 `/ ..` 意思是不要添加 `end()`，使得 `opt` 可以添加进 filter
    // 在添加完 `opt` 之后手动添加 `end()`
    warp::path!("holodeck" / ..)
        .and(opt)
        .and(warp::path::end())
        .and(warp::get())
        .and(db_map)
        .and_then(handlers::handle_list_sims)
}

/// 用于接收 JSON，参数 db。打包好发送至 `handle_create_sim` 处理具体业务。
pub fn post_sim(
    db: models::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // `warp::any` 意为捕获所有内容，即一个 filter 没有过滤。
    let db_map = warp::any().map(move || db.clone());

    // 确保 Db 被包装在 filter 内
    // `handle_create_sim` 的两个参数 `sim` 和 `db`，由两个 `.and` 接收后 `and_then` 传入
    warp::path!("holodeck")
        .and(warp::post())
        .and(json_body())
        .and(db_map)
        .and_then(handlers::handle_create_sim)
}

pub fn update_sim(
    db: models::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let db_map = warp::any().map(move || db.clone());

    warp::path!("holodeck" / u64)
        .and(warp::put())
        .and(json_body_put())
        .and(db_map)
        .and_then(handlers::handle_update_sim)
}

pub fn delete_sim(
    db: models::Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let db_map = warp::any().map(move || db.clone());

    warp::path!("holodeck" / u64)
        .and(warp::delete())
        .and(db_map)
        .and_then(handlers::handle_delete_sim)
}
