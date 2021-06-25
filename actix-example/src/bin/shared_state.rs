//! 应用程序可能有多个数据对象被多个 handlers 所共享。
//!
//! 对于全局共享状态而言，用一个 `actix_web::web::Data` 包裹它并且移动它进入工厂闭包。
//! 该闭包仅在每个线程上调用一次，我们克隆状态并且通过 `.app_data(state.clone())` 将状态与每个 `App` 的实例关联。
//!
//! 对于线程的本地状态而言，我们构建状态于工厂闭包，并且通过 `.data(state)` 将状态与 app 关联。
//!
//! 我们在 handlers 中通过一个 `state: Data<...>` 参数来获取 app 状态。
//!
//! 默认情况下， `actix-web` 在每个逻辑 cpu 核上会运行一个 `App`。
//! 运行在 <N> 核时，我们会看到在每次 endpoint 被调用时，将增加 `counter1` （通过 Mutex 而改变的全局状态）和
//! `counter3` （通过原子变量而改变的全局变量），而增加 `counter2` 仅发生在平均每 N 次时 （线程的本地状态）。
//! 这是因为工作负载在内核之间平均分配。
//!
//! 查看 [用户指引](https://actix.rs/docs/application/#state) 了解更多信息。

use std::cell::Cell;
use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};

/// 简易 handle
async fn index(
    counter1: web::Data<Mutex<usize>>,
    counter2: web::Data<Cell<u32>>,
    counter3: web::Data<AtomicUsize>,
    req: HttpRequest,
) -> HttpResponse {
    println!("{:?}", req);

    // 增加 counters
    *counter1.lock().unwrap() += 1;
    counter2.set(counter2.get() + 1);
    counter3.fetch_add(1, Ordering::SeqCst);

    let body = format!(
        "global mutex counter: {}, local counter: {}, global atomic counter: {}",
        *counter1.lock().unwrap(),
        counter2.get(),
        counter3.load(Ordering::SeqCst),
    );
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // 构建服务器前先创建一些全局状态
    #[allow(clippy::mutex_atomic)] // it's intentional.
    let counter1 = web::Data::new(Mutex::new(0usize));
    let counter3 = web::Data::new(AtomicUsize::new(0usize));

    // move 是必须的，用于提供给闭包 counter1 的所有权
    HttpServer::new(move || {
        // 创建一些线程的本地状态
        let counter2 = Cell::new(0u32);

        App::new()
            .app_data(counter1.clone()) // 添加共享状态
            .app_data(counter3.clone()) // 添加共享状态
            .data(counter2) // 添加线程的本地状态
            // 开启 logger
            .wrap(middleware::Logger::default())
            // 注册简易 handler
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
