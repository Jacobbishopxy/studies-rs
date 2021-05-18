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

### Tutorial #3

不同于教程给出的：

```rs
use yew::services::{ConsoleService, DialogService};

pub struct App {
    items: Vec<i64>,
    link: ComponentLink<Self>,
    console: ConsoleService,
    dialog: DialogService,
}
```

因为`yew`升级，不再使用函数调用的方式，如：

```rs
self.console.error("No more elements to remove!");
self.dialog.alert("I kenw it!");
```

而是使用关联函数的方式：

```rs
ConsoleService::error("No more elements to remove!");
DialogService::alert("I knew it!");
```

因此，`App`结构体中不再需要`console`和`dialog`成员，即保持 Tutorial #2 中的样式：

```rs
pub struct App {
    items: Vec<i64>,
    link: ComponentLink<Self>,
}
```

### Tutorial #4

持久化：

```rs
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::Area;
use yew::services::StorageService;

const KEY: &'static str = "yew.tut.database";
```

- 使用 serde 的 _serialize_ 来格式化数据， _deserialize_ 反之
- 使用 wasm_bindgen 获得对栈的操作，这里需要硬编码，之后会细讲
- 从 Yew 导入 _Json_ 使得可以用 JSON 创建和卸出数据
- Yew 的 prelude
- 导入 Yew 关于 storage area 的模块，使得我们可以存储数据（以 local 或是 session 的方式），并且存储 service 本身

更新 _Msg_ 和 _App_ ：

```rs
pub enum Msg {
    AddTask,
    RemoveTask(usize),
    SetTitle(String),
    SetDescription(String),
}

pub struct App {
    link: ComponentLink<Self>,
    storage: StorageService,
    database: Database,
    temp_task: Task,
}
```

更新 `create` 函数：

```rs
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let Json(database) = storage.restore(KEY);
        let database = database.unwrap_or_else(|_| Database::new());
        App {
            link,
            storage,
            database,
            temp_task: Task::new(),
        }
    }
```

- 首先创建了 `storage` 对象，同时使用了 local 方式而不是 session 方式。
- 通过 `restore` 函数，使用 JSON 解构 database。即恢复数据库，将其作为 JSON 从连接到 KEY 的 `storage` 中加载
- 第一次使用 `restore` 会返回错误，因此为了数据库初始化需要使用 `unwrap_or_else`

更新 `update` 函数：

```rs
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddTask => {
                if self.temp_task.is_filledin() {
                    self.database.tasks.push(self.temp_task.clone());
                    self.storage.store(KEY, Json(&self.database));
                    self.temp_task = Task::new();
                    refreshform("taskform");
                }
            }
            Msg::RemoveTask(pos) => {
                let _ = self.database.tasks.remove(pos);
                self.storage.store(KEY, Json(&self.database));
            }
            Msg::SetTitle(title) => {
                self.temp_task.title = title;
            }
            Msg::SetDescription(description) => {
                self.temp_task.description = description;
            }
        }
        true
    }
```

在 `Msg::AddTask` 中调用了神秘函数 `refreshform("taskform");`。该函数通过 `wasm-bindgen` 创建，其目的在于：当用户在页面中填写表单并点击提交时，页面通常会将信息发送到另一个页面，如果设置表单的 `action` 字段，则加载页面，或者使用更新的信息刷新同一页面。

当我们触发 _Add Task_ 按钮，信息将被发送到我们构建的 WASM app。该 app 需要用一种方式刷新表单，如果我们希望重置 _title_ 和 _description_ 输入。迄今为止 Yew 还没有一个直截了当的方式进行这项工作。

我们可以使用 `#[wasm_bindgen]` 来绑定一个由 JavaScript 所创建的函数给 Rust。这是一个 FFI 行为。 _wasm_bindgen_ 允许两种方式：编写一个 `.js` 文件将其绑定给 Rust 函数接口，或者直接编写一个内联的 JavaScript 函数。

```rs
#[wasm_bindgen(
    inline_js = "export function refreshform(form) { document.getElementById(form).reset(); }"
)]
extern "C" {
    fn refreshform(form: &str);
}
```

### Tutorial #5

更新 `index.html`，引入 JS 库 uuid 并移除 css：

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <title>Yew Tutorial</title>

    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />

    <script src="/pkg/bundle.js" defer></script>
    <script src="https://unpkg.com/uuid@latest/dist/umd/uuidv4.min.js"></script>
  </head>

  <body></body>
</html>
```

新建 `src/external.rs`：

```rs
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    fn uuidv4() -> JsValue;
}
```

- 我们要是用一个外部 JS 库，因此我们需要通过 FFI 接口引入所有所需的对象和函数，记住每项都需要使用 `#[wasm_bindgen]` 进行绑定。
- 使用 uuid 版本 4，我们仅需函数（`uuidv4`）的接口，可以跳过其它 JS 模块的接口
- 从 JavaScript 而来的所有值的类型皆为 `JsValue`。我们用它作为 JS 函数的返回类型，然后尝试将其转为 Rust 的类型。

```rs
#[derive(Default)]
pub struct ExternalService();

impl ExternalService {
    pub fn new() -> Self {
        ExternalService()
    }

    pub fn uuidv4(&mut self) ->String {
        let v = uuidv4();
        v.as_string()
        .unwrap_or_else(|| {
            console::log_1(&JsValue::from_str("Can't get a uuid"));
            "".to_string()
        })
    }
}
```

这其中的重点是 `uuidv4` 函数，它管理了 `uuid` 库的连接。我们定义了 _external_ 的 `uuidv4` 返回类型 `JsValue`；现在我们调用它并转换其结果成为更有用的 `String`

- `let v = uuidv4()`：分配 JS 的 `uuidv4()` 函数结果给变量 `v`：该变量现在是一个 `JsValue`。
- `v.as_string()`：尝试转换 `JsValue` 为 Rust 的 `String`。
- `.unwrap_or_else`：因为 `as_string()` 可能会失败，我们需要 unwrap 其返回类型；使用 `unwrap_or_else` 处理失败，并在闭包中获取失败时的返回值。
- `console::log_1(&JsValue::from_str("Can't get a uuid"))`：闭包开始于打印 Can't get a uuid，然而为了达到这个目的我们需要传递一个 JsValue 的引用给 `console::log_1`，即 `web_sys` 的控制台。为什么不使用 Yew 自带的控制台呢？首先 Yew 的控制台服务是用于 Yew 的，而在 _src/external.rs_ 中的代码更为常规以及底层的。其次我们可以实例化 Yew 的服务，在其中创建 `ServiceConsole`；这里我们在更为底层的环境中更好是用一些合适的底层工具。这里更想展示的是如何使用独立与 Yew 的控制台。
- 在打印之后我们返回一个空字符串。为什么这么做呢？因为在 _src/app.rs_ 将会更明显，通常来说如果一个前端控制返回一个错误，不显示红色的错误以及清单给用户更为美观。如果用户点击按钮无事发生，用户会认为接口坏了接着跳过。然而如果用户开始看见错误代码，信息等等。一个更好的解决方案便是系统捕获错误，展示用户一个简单的信息讲明是什么出错了；重点是不用担心用户看到调试信息，也就是说这都是给开发者看的。

通过 API 来增强我们的 app：

```rs
#[derive(Deserialize, Debug, Clone)]
pub struct ServerResponse {
    pub country_name: String,
    pub country_code: String,
    pub city: String,
    pub ip: String,
}

#[derive(Default)]
pub struct HostIpService {}

impl HostIpService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_response(&mut self, callback: Callback<Result<ServerResponse, Error>>) -> FetchTask {
        let url = format!("http://api.hostip.info/get_json.php");
        let handler = move |response: Response<Json<Result<ServerResponse, Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            if meta.status.is_success() {
                callback.emit(data)
            } else {
                callback.emit(Err(anyhow!(
                    "{}: error getting ip from http://api.hostip.info/get_json.php",
                    meta.status
                )))
            }
        };
        let request = Request::get(url.as_str()).body(Nothing).unwrap();
        FetchService::fetch(request, handler.into()).unwrap()
    }
}
```

与文章区别：`FetchService` 不再需要实例化后再调用其 `get` 等方法，现在可直接使用其关联函数。

这里的关键是 `get_response` 函数：

- 首先我们注意到的：使用返回 `Result` 的 `Callback` 作为入参。返回中使用 `anyhow` 的错误处理。
- 函数返回一个 `FetchTask`，其会被存储于 App 中。
- 首先存储 Web API URL 进 _url_ 变量中，接着创建一个命名的闭包，当 API 的响应达到页面时，它会被 fetch 调用。
- 我们需要移动（`move`）服务所接受到的响应（`response`）进闭包。
- 解构 `response` 成为 _meta_ （headers）以及 _data_ （响应的主体）。用 `Json` 包裹 _data_。
- 如通常基于 HHTP 的服务一样，如果成功 header 则包含 200 代码，否则错误代码。因此我们 _状态_ 成功（200），失败（其它代码）。`meta.status.is_success()` 在这里就很有用了。
- 成功状态下我们需要返回结果。问题是没有平常的函数带着一个 `return` 关键字。这种情况下我们需要 `emit` 我们的 `callback`，也就是说我们会发送 API 服务的返回给 callback，此时 JSON 包裹了 _data_。我们需要这样间接的返回响应（通过副作用的方式），因为我们将返回一个 `TaskService`，其用作为处理 fetching 过程。
- 失败情况下，我们通过`anyhow!` 宏来 `emit` 包含自定义信息的错误，这里面也包含了 `status`。
- 现在最肉的部分来了：我们为服务准备了 _Request_，作为 `GET` 请求使用 `Request::get`；传入 API 服务的 url 以及 `Nothing` 作为 body。真实世界并不是“nothing”，而是一个格式化的 body，被视为一个 no-body（因为不需要）提供给服务。
- 最后返回 `fetch` 任务（现在可被发送了）。这个 task 我们传入了准备好的请求，以及服务器返回响应时所要调用的闭包。
