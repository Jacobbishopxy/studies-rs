
# Tonic Web React

## Reference

1. [Article 1](https://betterprogramming.pub/building-a-website-using-rust-grpc-web-react-7412f1596a17)
1. [Article 2](https://www.koyeb.com/tutorials/build-and-deploy-a-grpc-web-app-using-rust-tonic-and-react)
1. [Official tonic-web](https://github.com/hyperium/tonic/tree/master/examples/src/grpc-web)

## Tool

- grpcurl: [repo](https://github.com/fullstorydev/grpcurl)

## Test

```sh
grpcurl -d '{"name": "jacob"}' -plaintext -import-path proto -proto helloworld.proto 127.0.0.1:3000 helloworld.Greeter/SayHello

grpcurl -d '{"name": "home"}' -plaintext -import-path proto -proto page.proto 127.0.0.1:3000 page.Page/GetPage
```

## Web

1. Create frontend project: `yarn create react-app react-web --template typescript && cd react-web`

1. Add protoc js plugin `yarn add --dev protoc-gen-grpc-web`

1. Add dependencies: `yarn add google-protobuf grpc-web`

1. generate code (notice here we used `--plugin=protoc-gen-grpc-web` with specified location):

    ```sh
    protoc -I=../proto/ page.proto \\
     --grpc-web_out=import_style=typescript,mode=grpcweb:./src \\
     --js_out=import_style=commonjs,binary:./src \\
     --plugin=protoc-gen-grpc-web=./node_modules/.bin/protoc-gen-grpc-web
    ```

## Todo

- test case in Rust

- React web
