use yew::prelude::*;

use crate::components::{Count, Goal, Thing};

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
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
            </div>
        }
    }
}
