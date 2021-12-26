# Yew Getting Started

## 准备工作

- 安装 WebAssembly target

  ```sh
  rustup target add wasm32-unknown-unknown
  ```

- 安装 trunk，wasm-bindgen-cli，wasm-pack

  ```sh
  cargo install trunk
  cargo install wasm-bindgen-cli
  cargo install wasm-pack
  ```

## trunk 使用

```sh
trunk serve             # 启动 dev 服务
trunk build --release   # 编译 release 版本
```

## wasm-pack 使用

需要在 toml 文件中加入（注意：需要 `src/lib.rs` 存在时，rust-analyzer 才可以识别 toml 文件）：

```toml
[lib]
crate-type = ["rlib", "cdylib"]
```

### Build

该命令将会创建一个 bundle 于 `./pkg` 路径下，该路径也包含了 app 被打包好的 wasm 文件以及对其进行包装的 JavaScript 文件，它们都可被用于 app 中。

```sh
wasm-pack build --target web
```

### Bundle

更多细节请访问该[手册](https://rollupjs.org/guide/en/#quick-start)。

```sh
rollup ./main.js --format life --file ./pkg/bundle.js
```

如果是使用类似 rollup.js 的 bundle 则可以省略 `--target web`。

### Serve

可以使用任意启动服务。这里我们使用一个简单的 Python 服务来启动构建后的 app。

```sh
python -m http.server 8000
```

如果没有安装 Python，则可以使用 `simple-http-server` crate。
