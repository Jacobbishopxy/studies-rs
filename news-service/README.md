# Building a Microservice with Rust

[原文地址](https://medium.com/@ilegra/building-a-microservice-with-rust-ef9641cf2331)

## 结构

- news-contract

  SAO contract 部分，拥有 News 的结构

- news-dao

  响应式的持久化代码，使用 tokio-postgres 以及针对 News 资源所有的 CURD 操作

- news-migrations

  使用 barrel 以及自定义逻辑来创建表与添加测试用的数据。

- news-service

  项目的 endpoint，服务的实现以及使用了 actix-web 框架的 main class

### Migrations

原文使用 postgres 库作为数据库连接，而本项目用 sqlx + async-std 作为替代。注意原文的 postgres 是同步的代码，而 sqlx 是异步，所以需要使用异步运行时（async-std）。

### Contract

定义一个名为 News 的结构体，并为其实现 Display 用于打印。

### Service

定义了 HttpServer actix，以及若干句柄：index，list_news，get_news_by_id，和 delete_news_by_id。服务将运行在本地的 8080 端口。通过 log 和 env_logger 库，所有的信息将被打印进日志。

`endpoints.rs` 中使用了 REST 操作（PUT，DELETE 和 GET 等）的宏。每个函数皆为公有，其中皆调用 service 并返回服务器的结果并序列化为 json。

此处是服务的实现，没有任何 REST 或 actix 的依赖在此。用于实现验证，业务逻辑，以及委派给 dao。所有的 CRUD 函数皆是异步的。

### Dao

原文使用 tokio 以及 tokio_postgres 作为异步的 DAO，本项目用的是 diesel （同步代码）作为替代。
