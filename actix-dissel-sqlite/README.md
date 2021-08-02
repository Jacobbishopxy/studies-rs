# Actix-web + Diesel + Sqlite3

[原文地址](https://github.com/actix/examples/tree/master/database_interactions/diesel)

## 依赖

- actix-web
- diesel
- dotenv
- env_logger
- failure
- futures
- r2d2
- serde
- serde_json
- uuid

## 设计

- [controllers.rs](./src/controller.rs)：业务处理
- [models.rs](./src/models.rs)：数据结构
- [resolvers.rs](./src/resolvers.rs)：API 接口
- [schema.rs](./src/schema.rs)：由 `diesel migration run` 自动生成

## 备注

初始化 SQLite 数据库前需要确保安装：

```sh
# on OpenSUSE
sudo zypper install sqlite3-devel libsqlite3-0 sqlite3

# on Ubuntu
sudo apt-get install libsqlite3-dev sqlite3

# on Fedora
sudo dnf install libsqlite3x-devel sqlite3x

# on macOS (using homebrew)
brew install sqlite3
```

初次运行时需要初始化 SQLite 数据库：

```sh
cd ads
# 安装 diesel_cli 用于 migration
cargo install diesel_cli --no-default-features --features sqlite

echo "DATABASE_URL=test.db" > .env
# 前提是 migrations 文件夹下的 *_create_users 存在 down.sql 与 up.sql
diesel migration run
```
