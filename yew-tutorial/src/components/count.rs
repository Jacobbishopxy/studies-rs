use yew::prelude::*;

pub struct Count {
    counter: i64,
    link: ComponentLink<Self>,
}

pub enum Msg {
    AddOne,
    RemoveOne,
}

impl Component for Count {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Count { counter: 0, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.counter += 1,
            Msg::RemoveOne => self.counter -= 1,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    /// view 函数仅依赖于其内部状态：消息应该保留在消息所属的地方，即更新的地方。
    /// 这样做是为了将组件的表达与其逻辑分开。
    ///
    /// 使用 `html!` 强力宏渲染组件，以 quasi-html 的方式展示。
    /// 其底层的逻辑与 React 中的 JSX 一致：
    /// 插入 HTML 元素于一个语言中（这里是Rust），使用一种 DSL。
    fn view(&self) -> Html {
        html! {
            <>
                <p>{"Counter: "} { self.counter }</p>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "Add 1" }</button>
                {" "}
                <button onclick=self.link.callback(|_| Msg::RemoveOne)>{ "Remove 1" }</button>
            </>
        }
    }
}
