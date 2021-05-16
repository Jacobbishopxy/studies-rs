// 定义一个 *app* 模块
mod app;

// 为了使用`yew::start_app`
use wasm_bindgen::prelude::*;

// 通过`#[wasm_bindgen]`标注`WASM`的入口点
// *main.js* 文件将会指向这里
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    // *run_app()* 返回一个被包裹进 Option 的 **JsValue**
    // 函数需要一个 Result 的返回类型
    yew::start_app::<app::App>();

    Ok(())
}
