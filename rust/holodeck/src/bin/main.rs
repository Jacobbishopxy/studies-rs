use holodeck::{filters, models};
use warp::Filter;

#[tokio::main]
async fn main() {
    let db = models::new_db();

    // 通过 `.or` 整合所有的 filter。
    // `db.clone()` 并没有创建一个新的 Copy，这是因为克隆的是一个 `Arc`。
    // `db.clone()` 这个动作意为创建一个 Mutex 的新的引用。
    let routes = filters::list_sims(db.clone())
        .or(filters::post_sim(db.clone()))
        .or(filters::update_sim(db.clone()))
        .or(filters::delete_sim(db));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
