use rand::prelude::*;
use yew::prelude::*;
use yew::services::{ConsoleService, DialogService};

pub struct App {
    counter: i64,
    items: Vec<i64>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    AddOne,
    RemoveOne,
    AddOneItem,
    RemoveOneItem,
    About,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            counter: 0,
            items: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.counter += 1,
            Msg::RemoveOne => self.counter -= 1,
            Msg::AddOneItem => {
                let added: i64 = random();
                self.items.push(added);
                ConsoleService::log(format!("Added {}", added).as_str());
            }
            Msg::RemoveOneItem => {
                let removed = self.items.pop();

                // console 的不同等级
                match removed {
                    Some(x) => ConsoleService::warn(format!("Removed {}", x).as_str()),
                    None => {
                        ConsoleService::error("No more elements to remove!");
                        let user_is_a_monkey = DialogService::confirm(
                            "Are you dum? There are no more elements to remove!",
                        );

                        if user_is_a_monkey {
                            DialogService::alert("I knew it!");
                        } else {
                            DialogService::alert(
                                "Maybe it was an error, there are no more elements to remove!",
                            );
                        }
                    }
                }
            }
            Msg::About => {
                DialogService::alert("Here is the about button's reaction.");
            }
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
        let render_item = |item| {
            html! {
                <>
                    <tr><td>{item}</td></tr>
                </>
            }
        };

        html! {
            <div class="main">
                <p>{"Counter: "} { self.counter }</p>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "Add 1" }</button>
                {" "}
                <button onclick=self.link.callback(|_| Msg::RemoveOne)>{ "Remove 1" }</button>
                <br/>
                <div class="card">
                    <header>{"Items: "}</header>
                    <button onclick=self.link.callback(|_| Msg::About)>{ "About" }</button>
                    <div class="card-body">
                        <table class="primary">{ for self.items.iter().map(render_item) }</table>
                    </div>
                    <footer>
                        <button onclick=self.link.callback(|_| Msg::AddOneItem)>{ "Add 1" }</button>
                        {" "}
                        <button onclick=self.link.callback(|_| Msg::RemoveOneItem)>{ "Remove 1" }</button>
                    </footer>
                </div>
            </div>
        }
    }
}
