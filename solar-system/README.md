# Solar system

[origin](https://romankudryashov.com/blog/2021/04/grpc-rust/)

## Prerequisites

Copy `server/.env.template` to `server/.env`, and modify it with your own arguments.

Initializing diesel tool for migration (only if you don't have it):

```sh
cargo install diesel_cli --no-default-features --features "postgres sqlite mysql"
```

## Start up

Build gRPC dependencies:

```sh
cd rpc && cargo build
```
