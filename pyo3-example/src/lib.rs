mod async_tc;
mod sync_tc;

use async_tc::*;
use pyo3::prelude::*;
use sync_tc::*;

// 注意函数名 pyo3_starter 必须与 Cargo.toml 中的 [lib.name] 一致
#[pymodule]
fn pyo3_starter(_py: Python, m: &PyModule) -> PyResult<()> {
    // sync
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    m.add_function(wrap_pyfunction!(get_fibonacci, m)?)?;
    m.add_function(wrap_pyfunction!(list_sum, m)?)?;
    m.add_function(wrap_pyfunction!(dict_printer, m)?)?;
    m.add_function(wrap_pyfunction!(word_printer, m)?)?;
    m.add_class::<RustStruct>()?;
    m.add_function(wrap_pyfunction!(human_says_hi, m)?)?;

    // async
    m.add_class::<MyCoroutine>()?;
    m.add_function(wrap_pyfunction!(my_coroutine, m)?).unwrap();

    Ok(())
}
