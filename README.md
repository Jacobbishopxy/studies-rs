# Studies RS

Since `rust-analyzer` needs full scan each time opening the project, it takes a long time and disk space for saving `target` file. Hence, for VsCode user please manually uncomment the sub-project name (willing to be tested), and this setting is in "rust-analyzer.linkedProjects" field which locates in `.vscode/settings.json` file.

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
- [mongo-example](./mongo-example/README.md)
- [redis-example](./redis-example/README.md)
- [nom-example](./nom-example/README.md)
- [telegram-bot](./telegram-bot/README.md)
- [mongodb-redis](./mongodb-redis/README.md)
- [yew-getting-started](./yew-getting-started/README.md): new version 0.19, official tutorial
- [pyo3-example](./pyo3-example/README.md)
- [procedural-macro-101](./procedural-macro-101/README.md)
- [web-wasm-demo](./web-wasm-demo/README.md)
- [actix-upload-download](./actix-upload-download/README.md)
- [tonic-web-react](./tonic-web-react/README.md)

## To test

- [ooxml-rs](https://github.com/zitsen/ooxml-rs)
- [kafka-fun](https://github.com/diegopacheco/rust-playground/tree/master/kafka-fun)

## To read

- [Procedural macro in rust](https://dev.to/naufraghi/procedural-macro-in-rust-101-k3f#modern-rust-raw-combinations-endraw-)
- [Desktop app using Tauri and Yew](https://dev.to/stevepryde/create-a-desktop-app-in-rust-using-tauri-and-yew-2bhe)
- [How Bevy uses Rust traits for labeling](https://deterministic.space/bevy-labels.html)
- [Actors with Tokio](https://ryhl.io/blog/actors-with-tokio/)
- [Writing a prometheus exporter in rust from idea to grafana chart](https://mateusfreira.github.io/@mateusfreira-writing-a-prometheus-exporter-in-rust-from-idea-to-grafana-chart/)
- [First steps with Rust declarative macros!](https://dev.to/rogertorres/first-steps-with-rust-declarative-macros-1f8m)
- [Macros in Rust: A tutorial with examples](https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/)
- [Procedural Macros: A simple derive macro](https://blog.turbo.fish/proc-macro-simple-derive/)
- [15k inserts/s with Rust and SQLite](https://kerkour.com/blog/high-performance-rust-with-sqlite/)
- [How to deploy Rust on Heroku (with Docker)](https://kerkour.com/blog/deploy-rust-on-heroku-with-docker/)
- [Build a Rust + WebAssembly frontend web app with Yew](https://blog.logrocket.com/rust-webassembly-frontend-web-app-yew/)
- [Machine learning in Rust using Linfa](https://blog.logrocket.com/machine-learning-in-rust-using-linfa/)
- [Untapped potential in Rust's type system](https://www.jakobmeier.ch/blogging/Untapped-Rust.html)
- [How to write Full Stack Rust code](https://www.steadylearner.com/blog/read/How-to-write-Full-Stack-Rust-code)
- [How to use NPM packages with Rust Frontend](https://www.steadylearner.com/blog/read/How-to-use-NPM-packages-with-Rust-Frontend)
- [Fullstack Rust with Yew](https://www.steadylearner.com/blog/read/Fullstack-Rust-with-Yew)
- [Rust on the front-end](https://dev.to/nfrankel/rust-on-the-front-end-hen)
- [How to start Rust Chat App](https://www.steadylearner.com/blog/read/How-to-start-Rust-Chat-App)
- [How to use gRPC with Rust Tonic and Postgres database with examples](https://dev.to/steadylearner/how-to-use-grpc-with-rust-tonic-and-postgres-database-with-examples-3dl7)
- [A Web App in Rust](https://dev.to/krowemoh/series/9410)
- [GraphQL in Rust](https://romankudryashov.com/blog/2020/12/graphql-rust/#_conclusion)
- [Configuration management in Rust web services](https://blog.logrocket.com/configuration-management-in-rust-web-services/)
- [Web services' JSON input validation](https://blog.logrocket.com/json-input-validation-in-rust-web-services/)
- [The Algorithms - Rust](https://github.com/TheAlgorithms/Rust)
- [Get started making desktop apps using Rust and React](https://kent.medium.com/get-started-making-desktop-apps-using-rust-and-react-78a7e07433ce)
- [MoonZoon Dev News Series](https://dev.to/martinkavik/series/11511)
- [Calling Rust from Python using PyO3](https://saidvandeklundert.net/learn/2021-11-18-calling-rust-from-python-using-pyo3/)

## To learn

- [nom](https://github.com/Geal/nom)
- [async-graphql](https://github.com/async-graphql/async-graphql)
- [jsonwebtoken](https://github.com/Keats/jsonwebtoken)
- [rbatis](https://github.com/rbatis/rbatis)
- [rdkafka](https://github.com/fede1024/rust-rdkafka)
- [algorithms](https://github.com/EbTech/rust-algorithms)
- [yew](https://github.com/yewstack/yew)
- [yewprint](https://github.com/cecton/yewprint)
- [redis-rs](https://github.com/mitsuhiko/redis-rs)
- [bson-rust](https://github.com/mongodb/bson-rust)
- [mongo-rust-driver](https://github.com/mongodb/mongo-rust-driver)

## Little Notes

String conversion:

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
