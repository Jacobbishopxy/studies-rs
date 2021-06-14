# Fintech Banking App

[原文地址](https://dev.to/duomly/series/6782)

## 工具

- migrate: migration cli

  ```sh
  go install -tags 'postgres' github.com/golang-migrate/migrate/v4/cmd/migrate@latest
  ```

- sqlc: sql generate cli

  ```sh
  go get github.com/kyleconroy/sqlc/cmd/sqlc
  ```

## 依赖

- pq: postgres driver

  ```sh
  go get -u github.com/lib/pq
  ```

- testify: test

  ```sh
  go get github.com/stretchr/testify
  ```
