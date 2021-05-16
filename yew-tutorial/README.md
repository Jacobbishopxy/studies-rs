# Yew Tutorial

[原文地址](https://github.com/davidedelpapa/yew-tutorial)

## 预备

```sh
cargo install wasm-pack
npm i -g rollup
```

## 要点

### Tutorial #1

在完成所有代码编写后的构建与运行：

```sh
wasm-pack build --target web
```

第一次构建所需的时间一般都会长一些。在看到以下信息后，说明构建完毕：

```null
[INFO]: :-) Your wasm pkg is ready to publish at ./pkg.
```

如果我们检查内容将会看到 _.js_ 和 _.wasm_ 文件可被服务启动了。由于我们需要把它们捆绑到一起，我们需要使用*rollup*工具：

```sh
rollup ./main.js --format iife --file ./pkg/bundle.js
```

完成后可以使用 python 的 _http.server_ 来启动服务。

### Tutorial #2

`run.sh`中把执行行为分为三个函数用于在文件最后执行。模块化很重要，因为只有这样我们可以进行独立的修改。

- *build*函数调用`wasm-pack`进行`WASM`的构建
- *pack*函数调用`rollup`进行捆绑 JS 和 WASM
- *run*函数即唤起了服务，同时使其在后台运行（在命令后方添加`&`）。也正因于此我们保持进程的 PID 于一个隐藏文件*.serverpid*中。

`stop.sh`获取*.serverpid*中的 PID，杀死进程。
