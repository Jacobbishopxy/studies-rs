# Demo Rust

- [holodeck](./holodeck/README.md)
- [actix_web-async_graphql_rbatis](./actix_web-async_graphql_rbatis/README.md)
- [actix_web-dissel-sqlite](./actix_web-dissel-sqlite/README.md)
- [simple-auth-server](./simple-auth-server/README.md)
- [yew-tutorial](./yew-tutorial/README.md)
- [news](./news/README.md)
- [tonic-example](./tonic-example/README.md)
- [solar-system](./solar-system/README.md)
- [docker-simple](./docker-simple/README.md)
- [actix-example](./actix-example/README.md)
- [rocket-tutorial](./rocket-tutorial/README.md)
- [file-upload-download](./file-upload-download/README.md)
- [parser-combinators](./parser-combinators/README.md)

## To test

- [web-cors](https://github.com/actix/examples/tree/master/security/web-cors)
- [kafka-fun](https://github.com/diegopacheco/rust-playground/tree/master/kafka-fun)

## To read

- [How to write Full Stack Rust code](https://www.steadylearner.com/blog/read/How-to-write-Full-Stack-Rust-code)
- [How to use NPM packages with Rust Frontend](https://www.steadylearner.com/blog/read/How-to-use-NPM-packages-with-Rust-Frontend)
- [Fullstack Rust with Yew](https://www.steadylearner.com/blog/read/Fullstack-Rust-with-Yew)
- [Rust on the front-end](https://dev.to/nfrankel/rust-on-the-front-end-hen)
- [How to start Rust Chat App](https://www.steadylearner.com/blog/read/How-to-start-Rust-Chat-App)
- [How to use gRPC with Rust Tonic and Postgres database with examples](https://dev.to/steadylearner/how-to-use-grpc-with-rust-tonic-and-postgres-database-with-examples-3dl7)
- [A Web App in Rust](https://dev.to/krowemoh/series/9410)
- [GraphQL in Rust](https://romankudryashov.com/blog/2020/12/graphql-rust/#_conclusion)
- [Web services' config management](https://blog.logrocket.com/configuration-management-in-rust-web-services/)
- [Web services' JSON input validation](https://blog.logrocket.com/json-input-validation-in-rust-web-services/)
- [The Algorithms - Rust](https://github.com/TheAlgorithms/Rust)
- [Get started making desktop apps using Rust and React](https://kent.medium.com/get-started-making-desktop-apps-using-rust-and-react-78a7e07433ce)
- [MoonZoon Dev News Series](https://dev.to/martinkavik/series/11511)

## To learn

- [async-graphql](https://github.com/async-graphql/async-graphql)
- [rocket](https://github.com/SergioBenitez/Rocket)
- [jsonwebtoken](https://github.com/Keats/jsonwebtoken)
- [rbatis](https://github.com/rbatis/rbatis)
- [rdkafka](https://github.com/fede1024/rust-rdkafka)
- [algorithms](https://github.com/EbTech/rust-algorithms)
- [yew](https://github.com/yewstack/yew)
- [yewprint](https://github.com/cecton/yewprint)

## Notes

```n
&str    -> String--| String::from(s) or s.to_string() or s.to_owned()
&str    -> &[u8]---| s.as_bytes()
&str    -> Vec<u8>-| s.as_bytes().to_vec() or s.as_bytes().to_owned()
String  -> &str----| &s if possible* else s.as_str()
String  -> &[u8]---| s.as_bytes()
String  -> Vec<u8>-| s.into_bytes()
&[u8]   -> &str----| s.to_vec() or s.to_owned()
&[u8]   -> String--| std::str::from_utf8(s).unwrap(), but don't**
&[u8]   -> Vec<u8>-| String::from_utf8(s).unwrap(), but don't**
Vec<u8> -> &str----| &s if possible* else s.as_slice()
Vec<u8> -> String--| std::str::from_utf8(&s).unwrap(), but don't**
Vec<u8> -> &[u8]---| String::from_utf8(s).unwrap(), but don't**
```
