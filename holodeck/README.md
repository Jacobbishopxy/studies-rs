# Holodeck

原文地址：[Engaging Warp: REST API with Rust Series' Articles](https://dev.to/rogertorres/series/12179)。

本 demo 展示的是原文中最后的一步，以及添加了少量个人注解。

Rust 有 actix，rocket，tide，warp 等等 web 框架，它们各有[优劣](https://www.lpalmieri.com/posts/2020-07-04-choosing-a-rust-web-framework-2020-edition/)。本项目采用 warp 作为框架，展示如何搭建一个 REST 服务。

## 依赖

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }      // warp 的依赖，项目中需要使用其类型、测试宏等
warp = "0.3"                                        // web 框架
serde = { version = "1", features = ["derive"]}     // 序列化与反序列化
serde_json = "1.0"                                  // 解析 JSON string
```

## 设计

- [models.rs](./src/models.rs)：数据结构
- [handlers.rs](./src/handlers.rs)：处理过程
- [filters.rs](./src/filters.rs)：接口
