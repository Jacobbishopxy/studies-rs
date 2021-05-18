use yew::prelude::*;

use crate::api::{HostIpService, ServerResponse};
use crate::components::{Count, Goal, Thing};
use crate::external::ExternalService;
use anyhow::Error;
use yew::format::Json;
use yew::services::fetch::FetchTask;
use yew::services::ConsoleService;

pub struct App {
    link: ComponentLink<Self>,
    service: ExternalService,
    uuidv4: String,
    ip_service: HostIpService,
    r: Option<ServerResponse>,
    callback: Callback<Result<ServerResponse, Error>>,
    task: Option<FetchTask>,
    ip: String,
}

pub enum Msg {
    PollService,
    // 用于poll api.rs
    GetIpResponse,
    // 当从服务收到响应后被触发
    IpResponseReady(Result<ServerResponse, Error>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link: link.clone(),
            service: ExternalService::new(),
            uuidv4: "".to_string(),
            ip_service: HostIpService::new(),
            callback: link.callback(Msg::IpResponseReady),
            r: None,
            task: None,
            ip: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PollService => {
                self.uuidv4 = self.service.uuidv4();
            }
            Msg::GetIpResponse => {
                let task = self.ip_service.get_response(self.callback.clone());
                self.task = Some(task);
            }
            Msg::IpResponseReady(Ok(r)) => {
                self.r = Some(r);
                ConsoleService::log(format!("Reponse: {:?}", Json(self.r.clone())).as_str());
                self.ip = self.r.as_mut().unwrap().ip.clone();
            }
            Msg::IpResponseReady(Err(e)) => {
                ConsoleService::log(format!("Error: {:?}", e).as_str());
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
                <div>
                    <h1>{ format!("{}", self.ip ) }</h1>
                    <button onclick=self.link.callback(|_| Msg::PollService)>{ "Get a UUID" }</button>
                    {" "}
                    <button onclick=self.link.callback(|_| Msg::GetIpResponse)>{ "Get my IP" }</button>
                    <p>{ format!("{}", &self.uuidv4) }</p>
                </div>
            </div>
        }
    }
}
