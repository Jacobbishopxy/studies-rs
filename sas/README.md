# Simple auth server

[原文地址](https://gill.net.in/posts/auth-microservice-rust-actix-web1.0-diesel-complete-tutorial/)

## 依赖

- actix-identity：
- actix-rt：
- actix-web：
- chrono ：
- derive_more ：
- diesel ：
- dotenv ：
- env_logger ：
- futures ：
- lazy_static ：
- r2d2 ：
- rust-argon2：
- serde ：
- serde_derive ：
- serde_json ：
- sparkpost ：
- uuid ：

## 设计

- [handlers](./src/handlers)
  - [auth_handler](./src/handlers/auth_handler.rs)：登陆验证、登出、获取现在的身份
  - [invitation_handler](./src/handlers/invitation_handler.rs)：邀请注册
  - [register_handler](./src/handlers/register_handler.rs)：注册用户
- [email_service](./src/email_service.rs)：邀请注册所需的邮件发送服务
- [errors](./src/errors.rs)：自定义错误类型
- [main](./src/main.rs)：创建数据库连接池、启动整合了所有路径的 HTTP 服务
- [models](./src/models.rs)：项目中所有的数据结构
- [schema](./src/schema.rs)：由 Diesel 自动生成的 schema
- [utils](./src/utils.rs)：常量、用于哈希密码的函数、用于验证的函数

## 测试

- 邀请测试

```sh
curl --request POST \
  --url http://localhost:3000/api/invitation \
  --header 'content-type: application/json' \
  --data '{"email":"name@domain.com"}'
```

- 注册测试

```sh
curl --request POST \
  --url http://localhost:3000/api/register/f87910d7-0e33-4ded-a8d8-2264800d1783 \
  --header 'content-type: application/json' \
  --data '{"email":"name@domain.com", "password":"password"}'
```

- 登陆测试

```sh
curl -i --request POST \
  --url http://localhost:3000/api/auth \
  --header 'content-type: application/json' \
  --data '{"email": "name@domain.com","password":"password"}'
```

```null
HTTP/1.1 200 OK
set-cookie: auth=iqsB4KUUjXUjnNRl1dVx9lKiRfH24itiNdJjTAJsU4CcaetPpaSWfrNq6IIoVR5+qKPEVTrUeg==; HttpOnly; Path=/; Domain=localhost; Max-Age=86400
content-length: 0
date: Sun, 28 Oct 2018 12:36:43 GMT
```

- 登出测试

```sh
curl -i --request DELETE \
  --url http://localhost:3000/auth
```

```null
HTTP/1.1 200 OK
set-cookie: auth=; HttpOnly; Path=/; Domain=localhost; Max-Age=0; Expires=Fri, 27 Oct 2017 13:01:52 GMT
content-length: 0
date: Sat, 27 Oct 2018 13:01:52 GMT
```

- 回传 cookie

```sh
curl -i --request GET \
  --url http://localhost:3000/auth \
  --cookie auth=HdS0iPKTBL/4MpTmoUKQ5H7wft5kP7OjP6vbyd05Ex5flLvAkKd+P2GchG1jpvV6p9GQtzPEcg==
```

```null
HTTP/1.1 200 OK
content-length: 27
content-type: application/json
date: Sun, 28 Oct 2018 19:21:04 GMT

{"email":"name@domain.com"}
```
