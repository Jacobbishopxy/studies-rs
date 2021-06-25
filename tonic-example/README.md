# Tonic example

[原文地址](https://github.com/hyperium/tonic/tree/master/examples)

## Route guide

### 为什么使用 gRPC

本例是一个简单的路由映射应用程序，用户可以获取路由的信息，创建一个他们路由的总结，以及交换路由信息例如服务器或者其它客户端的流量更新。

通过 gRPC 我们可以定义服务于一个 `.proto` 文件中，在任何支持 gRPC 的语言中实现客户端与服务端，最终可以运行在不同的环境中 - 所有复杂的语言间和环境间交流都由 gRPC 来处理。同样的我们获取了所有关于 protocol buffers 的好处，包括高效的序列化，简单的 IDL，以及轻松的接口更新。
