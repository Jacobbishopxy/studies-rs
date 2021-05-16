use yew::prelude::*;

pub struct App {
    counter: i64,
    link: ComponentLink<Self>,
}

pub enum Msg {
    AddOne,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App { link, counter: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.counter += 1,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    /// view 函数仅依赖于其内部状态：消息应该保留在消息所属的地方，即更新的地方。
    /// 这样做是为了将组件的表达与其逻辑分开。
    fn view(&self) -> Html {
        // 使用 `html!` 强力宏渲染组件，以 quasi-html 的方式展示。
        // 其底层的逻辑与 React 中的 JSX 一致：
        // 插入 HTML 元素于一个语言中（这里是Rust），使用一种 DSL。
        html! {
            <div>
            <p>{"Counter: "} { self.counter }</p>
            <button onclick=self.link.callback(|_| Msg::AddOne)>{ "Add 1" }</button>
            </div>
        }
    }
}
