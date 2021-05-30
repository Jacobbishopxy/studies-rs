# Actix-web + Async-graphql + Rbatis

原文地址：[基于 actix-web + async-graphql + rbatis + postgresql / mysql 构建异步 Rust GraphQL 服务](https://blog.budshome.com/budshome/ji-yu-actix-web-+-async-graphql-+-rbatis-+-postgresql---mysql-gou-jian-yi-bu-rust-graphql-fu-wu---qi-bu-ji-crate-xuan-ze)。

## 依赖

- actix-web：web 框架
- dotenv：配置文件
- lazy_static：Rust 静态变量存储
- async-graphql：gql 服务
- async-graphql-actix-web：actix-web 的 gql 支持
- rbatis：ORM
- serde：（反）序列化

## 设计

- [dbs](./src/dbs/mod.rs)：管理数据库连接
- [gql](./src/gql/mod.rs)：GraphQL 的接口
- [users](./src/users/mod.rs)：业务数据结构以及处理过程
- [util](./src/util/mod.rs)：配置等

## 备注

- 第一次执行项目需要新建 `.env` 配置文件，具体细节见 `.env.template`。
