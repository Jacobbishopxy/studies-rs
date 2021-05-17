use rand::prelude::*;
use yew::prelude::*;
use yew::services::{ConsoleService, DialogService};

pub struct Thing {
    items: Vec<i64>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    AddOne,
    RemoveOne,
    About,
}

impl Component for Thing {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Thing {
            link,
            items: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                let added: i64 = random();
                self.items.push(added);
                ConsoleService::log(format!("Added {}", added).as_str());
            }
            Msg::RemoveOne => {
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

    fn view(&self) -> Html {
        let render_item = |item| {
            html! {
                <>
                    <tr><td>{item}</td></tr>
                </>
            }
        };

        html! {
            <div class="card">
                <header>
                    {"Items: "}
                    <button onclick=self.link.callback(|_| Msg::About)>{ "About" }</button>
                </header>

                <div class="card-body">
                    <table class="primary">{ for self.items.iter().map(render_item) }</table>
                </div>

                <footer>
                    <button onclick=self.link.callback(|_| Msg::AddOne)>{ "Add 1" }</button>
                    {" "}
                    <button onclick=self.link.callback(|_| Msg::RemoveOne)>{ "Remove 1" }</button>
                </footer>
            </div>
        }
    }
}
