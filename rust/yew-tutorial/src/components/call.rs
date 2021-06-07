use anyhow::Error;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew::services::ConsoleService;

use crate::api::{HostIpService, ServerResponse};
use crate::external::ExternalService;

pub struct Call {
    link: ComponentLink<Self>,
    // 用于调用 JS 的外部 uuidv4 库
    service: ExternalService,
    uuidv4: String,
    // 用于调用 fetch 函数抓取外部数据
    ip_service: HostIpService,
    callback: Callback<Result<ServerResponse, Error>>,
    r: Option<ServerResponse>,
    _task: Option<FetchTask>,
    ip: String,
}

pub enum Msg {
    PollService,
    // 用于poll api.rs
    GetIpResponse,
    // 当从服务收到响应后被触发
    IpResponseReady(Result<ServerResponse, Error>),
}

impl Component for Call {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Call {
            link: link.clone(),
            service: ExternalService::new(),
            uuidv4: "".to_string(),
            ip_service: HostIpService::new(),
            callback: link.callback(Msg::IpResponseReady),
            _task: None,
            r: None,
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
                self._task = Some(task);
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
            <div>
                <p>{ format!("{}", self.ip ) }</p>
                <button onclick=self.link.callback(|_| Msg::PollService)>{ "Get a UUID" }</button>
                {" "}
                <button onclick=self.link.callback(|_| Msg::GetIpResponse)>{ "Get my IP" }</button>
                <p>{ format!("{}", &self.uuidv4) }</p>
            </div>
        }
    }
}
