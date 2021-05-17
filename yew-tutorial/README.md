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
