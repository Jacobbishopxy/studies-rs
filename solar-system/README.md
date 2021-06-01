# Solar system

## Prerequisites

Copy `.env.template` as `.env`, and modify it with your own arguments.

```sh
cargo install diesel_cli --no-default-features --features "postgres sqlite mysql"
```

## Start up

```sh
diesel migration run
```
