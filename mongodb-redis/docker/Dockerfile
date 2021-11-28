FROM rust:1.56

ENV CARGO_TERM_COLOR always

# 创建空项目用于缓存依赖
RUN USER=root cargo new --bin /mongodb-redis/docker-build
WORKDIR /mongodb-redis/docker-build
COPY /Cargo.lock ./
COPY /mongodb-redis/Cargo.toml ./
# 缓存依赖
RUN cargo install --path . --locked
COPY /mongodb-redis/ ./
RUN touch ./src/main.rs
RUN cargo install --path . --locked

FROM debian:buster-slim
COPY --from=0 /usr/local/cargo/bin/mongodb-redis /usr/local/bin/mongodb-redis
CMD ["mongodb-redis"]
