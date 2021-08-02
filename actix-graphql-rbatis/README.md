# Actix-web + Async-graphql + Rbatis

原文地址：[基于 actix-web + async-graphql + rbatis + postgresql / mysql 构建异步 Rust GraphQL 服务](https://blog.budshome.com/budshome/ji-yu-actix-web-+-async-graphql-+-rbatis-+-postgresql---mysql-gou-jian-yi-bu-rust-graphql-fu-wu---qi-bu-ji-crate-xuan-ze)。

## 依赖

1. backend:

   - actix-web：web 框架
   - dotenv：配置文件
   - lazy_static：Rust 静态变量存储
   - async-graphql：gql 服务
   - async-graphql-actix-web：actix-web 的 gql 支持
   - rbatis：ORM
   - serde：（反）序列化

1. frontend-yew:

   - wasm-bindgen：wasm 与 JavaScript 交互
   - yew：wasm 框架库

## 工具

1. frontend-yew:

   - trunk: 为 yew 打包
   - wasm-bindgen-cli: 脚手架

## 设计

1. backend:

   - [dbs](./src/dbs/mod.rs)：管理数据库连接
   - [gql](./src/gql/mod.rs)：GraphQL 的接口
   - [users](./src/users/mod.rs)：业务数据结构以及处理过程
   - [util](./src/util/mod.rs)：配置等

1. frontend-yew

## 备注

- 首次访问 `backend` 项目需新建 `.env` 配置文件，具体细节见 `./backend/.env.template`。
- 首次访问 `frontend-yew` 项目需添加 cargo 工具：`cargo install trunk wasm-bindgen-cli`。
- 首次使用 `trunk` 需要添加：`rustup target add wasm32-unknown-unknown`
