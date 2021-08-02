use yew::prelude::*;
use yew_router::components::RouterAnchor;
use yew_router::prelude::*;

mod pages;
use pages::{home::Home, projects::Projects, users::Users};

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum Route {
    #[to = "/users"]
    Users,
    #[to = "/projects"]
    Projects,
    #[to = "/"]
    Home,
}

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        unimplemented!()
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<Route>;

        html! {
            <>
            <div>
                { "actix-graphql-rbatis / frontend-yew" }
            </div>
            <div>
                <Anchor route=Route::Users>
                    { "User list" }
                </Anchor>
                { " - " }
                <Anchor route=Route::Projects>
                    { "Project list" }
                </Anchor>
                { " - " }
                <Anchor route=Route::Home>
                    { "Home" }
                </Anchor>
            </div>
            <main>
                <Router<Route, ()>
                    render = Router::render(|switch: Route| {
                        match switch {
                            Route::Users => html!{ <Users/> },
                            Route::Projects => html!{ <Projects/> },
                            Route::Home => html!{ <Home/> },
                        }
                    })
                />
            </main>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
