use yew::prelude::*;

use crate::components::{Button, Call, Count, Goal, Thing};

pub struct App {
    link: ComponentLink<Self>,
    counter: i32,
}

pub enum Msg {
    AddOne,
    RemoveOne,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App { link, counter: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.counter += 1;
            }
            Msg::RemoveOne => {
                self.counter -= if self.counter == 0 { 0 } else { 1 };
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="main">
                <h2>{"Example #1"}</h2>
                <br/>
                <Count/>
                <br/>
                <h2>{"Example #2"}</h2>
                <br/>
                <Thing/>
                <br/>
                <h2>{"Example #3"}</h2>
                <br/>
                <Goal/>
                <br/>
                <h2>{"Example #4"}</h2>
                <br/>
                <Call/>
                <br/>
                <h2>{"Example #5"}</h2>
                <br/>
                <div>
                    <h1>{ "Welcome to Components" }</h1>
                    <p>{ self.counter } </p>
                    <Button onsignal=self.link.callback(|_| Msg::AddOne) title="+1" />
                    {" "}
                    <Button onsignal=self.link.callback(|_| Msg::RemoveOne) title="-1" />
                </div>
            </div>
        }
    }
}
