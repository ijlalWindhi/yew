use routes::AppRoute;
use serde::Deserialize;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};
use yew_router::components::RouterAnchor;

#[derive(Deserialize, Debug, Clone)]
pub struct ISSPosition {
    latitude: String,
    longitude: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ISS {
    message: String,
    timestamp: i32,
    iss_position: ISSPosition,
}

#[derive(Debug)]
pub enum Msg {
    GetLocation,
    ReceiveResponse(Result<ISS, anyhow::Error>),
}

#[derive(Debug)]
pub struct Data {
    fetch_task: Option<FetchTask>,
    iss: Option<ISS>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl Data {
    fn view_iss_location(&self) -> Html {
        type Anchor = RouterAnchor<AppRoute>;
        match self.iss {
            Some(ref space_station) => {
                html! {
                    <>
                        <p>{ "The ISS is at:" }</p>
                        <p>{ format!("Message: {}", space_station.message) }</p>
                        <p>{ format!("Timestamp: {}", space_station.timestamp) }</p>
                        <p>{ format!("Latitude: {}", space_station.iss_position.latitude) }</p>
                        <p>{ format!("Longitude: {}", space_station.iss_position.longitude) }</p>
                        <Anchor
                            route=AppRoute::Register
                        >
                            {"Register"}
                        </Anchor>
                    </>
                }
            }
            None => {
                html! {
                    <>
                        <button onclick=self.link.callback(|_| Msg::GetLocation)>
                            { "Where is the ISS?" }
                        </button>
                        <Anchor
                            route=AppRoute::Register
                        >
                            {"Register"}
                        </Anchor>
                    </>
                }
            }
        }
    }
}
impl Component for Data {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            fetch_task: None,
            iss: None,
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
            GetLocation => {
                let request = Request::get("http://api.open-notify.org/iss-now.json")
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<ISS, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Msg::ReceiveResponse(data)
                        });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                true
            }
            ReceiveResponse(response) => {
                match response {
                    Ok(location) => {
                        self.iss = Some(location);
                    }
                    Err(error) => self.error = Some(error.to_string()),
                }
                self.fetch_task = None;
                true
            }
        }
    }
    fn view(&self) -> Html {
        html! {
            <>
                { self.view_iss_location() }
            </>
        }
    }
}
