use routes::AppRoute;
use serde::Deserialize;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        ConsoleService,
    },
};
use yew_router::components::RouterAnchor;

#[derive(Deserialize, Debug, Clone)]
pub struct FetchData {
    email: String,
    password: String,
    name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FetchResult {
    user: Vec<FetchData>,
}

#[derive(Debug)]
pub enum Msg {
    GetData,
    ReceiveResponse(Result<FetchResult, anyhow::Error>),
}

#[derive(Debug)]
pub struct Data {
    fetch_task: Option<FetchTask>,
    result: Option<FetchResult>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl Data {
    fn view(&self) -> Html {
        self.result
            .iter()
            .map(|data| {
                data.user
                    .iter()
                    .map(|data| {
                        html! {
                            <div>
                                <p>{data.email.clone()}</p>
                                <p>{data.password.clone()}</p>
                                <p>{data.name.clone()}</p>
                            </div>
                        }
                    })
                    .collect::<Html>()
            })
            .collect::<Html>()
    }
}

impl Component for Data {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            fetch_task: None,
            result: None,
            link,
            error: None,
        }
    }
    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        use Msg::*;

        match msg {
            GetData => {
                let request = Request::get("http://localhost:8080/user")
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = self.link.callback(
                    |response: Response<Json<Result<FetchResult, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::ReceiveResponse(data)
                    },
                );
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                true
            }
            ReceiveResponse(response) => {
                match response {
                    Ok(data) => {
                        ConsoleService::log(&format!("{:?}", data.user));
                        self.result = Some(data);
                    }
                    Err(error) => self.error = Some(error.to_string()),
                }
                self.fetch_task = None;
                true
            }
        }
    }
    fn rendered(&mut self, _first_render: bool) {
        if _first_render {
            self.link.send_message(Msg::GetData);
        }
    }
    fn view(&self) -> Html {
        html! {
            <div>
                <RouterAnchor<AppRoute> route=AppRoute::Register>
                    { "Register" }
                </RouterAnchor<AppRoute>>
                { self.view() }
            </div>
        }
    }
}
