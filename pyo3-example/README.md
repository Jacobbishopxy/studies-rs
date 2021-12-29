# Pyo3 Example

[原文地址](https://saidvandeklundert.net/learn/2021-11-18-calling-rust-from-python-using-pyo3/)

[Pyo3 源码](https://github.com/PyO3/pyo3)

## 在 Python 中使用 Rust

Python 安装 maturin 库：

```sh
pip install maturin
```

Rust 库初始化：

```sh
maturin init
```

Rust 库打包为 python 库：

```sh
maturin develop
```
