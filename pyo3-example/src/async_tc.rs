//! pyo3 async
//!
//! original link: https://github.com/ChillFish8/Async-PyO3-Examples

use pyo3::iter::IterNextOutput;
use pyo3::prelude::*;
use pyo3::{PyAsyncProtocol, PyIterProtocol};

// Python 类，用于处理 python 的协程
#[pyclass]
pub struct MyCoroutine {}

// 添加 async 协议，使其 awaitable
#[pyproto]
impl PyAsyncProtocol for MyCoroutine {
    fn __await__(slf: PyRef<Self>) -> PyRef<Self> {
        // 协程可迭代的部分
        slf
    }
}

#[pyproto]
impl PyIterProtocol for MyCoroutine {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        // 可选的函数，如果不想实现返回一个现存迭代器的方法，则不需要该函数
        slf
    }

    // 也有其它的返回类型可提供，IterNextOutput 则是现今最有用的创建协程的办法
    fn __next__(_slf: PyRefMut<Self>) -> IterNextOutput<Option<PyObject>, &'static str> {
        IterNextOutput::Return("Ended")
    }
}

#[pyfunction]
pub fn my_coroutine() -> MyCoroutine {
    MyCoroutine {}
}

#[pymodule]
pub fn starter_async(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MyCoroutine>()?;
    m.add_function(wrap_pyfunction!(my_coroutine, m)?).unwrap();
    Ok(())
}
