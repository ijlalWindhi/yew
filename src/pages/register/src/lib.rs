use routes::AppRoute;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew::{
    format::Json,
    services::fetch::{FetchService, FetchTask, Request, Response},
    services::ConsoleService,
};
use yew_router::components::RouterAnchor;

#[derive(Clone, Debug, Serialize)]
pub struct RequestRegister {
    email: String,
    password: String,
    name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ResponseMessage {
    message: String,
}

pub struct Register {
    form_data: RequestRegister,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
}

pub enum Msg {
    UpdateEmail(String),
    UpdatePassword(String),
    UpdateName(String),
    Submit,
    RegisterResponse(Result<ResponseMessage, anyhow::Error>),
}

impl Component for Register {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let form_data = RequestRegister {
            email: "".into(),
            password: "".into(),
            name: "".into(),
        };
        Register {
            form_data,
            link,
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateEmail(email) => {
                self.form_data.email = email;
                true
            }
            Msg::UpdatePassword(password) => {
                self.form_data.password = password;
                true
            }
            Msg::UpdateName(name) => {
                self.form_data.name = name;
                true
            }
            Msg::Submit => {
                let request = Request::post(format!("http://127.0.0.1:8080/user/add",))
                    .header("Content-Type", "application/json")
                    .body(Json(&self.form_data))
                    .expect("Could not build request.");
                let callback = self.link.callback(
                    |response: Response<Json<Result<ResponseMessage, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::RegisterResponse(data)
                    },
                );
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                true
            }
            Msg::RegisterResponse(response) => {
                ConsoleService::log(&format!("response: {:?}", response));
                true
            }
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<AppRoute>;
        html! {
            <div class="uk-section uk-section-muted uk-flex uk-flex-middle uk-animation-fade " >
                <div class ="uk-height-viewport ">
                    <div class="uk-width-1-1">
                        <div class="uk-container " style=" display:flex;">
                            <div class="uk-grid-margin uk-grid uk-grid-stack">
                                <div class="uk-width-1-1@m uk-position-center">
                                    <div class="uk-margin uk-width-large uk-margin-center uk-card uk-card-default uk-card-body uk-box-shadow-large">
                                        <h3 class="uk-card-title uk-text-center">
                                            {"Welcome"}
                                        </h3>
                                        <form>
                                            <div class="uk-margin">
                                                <div class="uk-inline uk-width-1-1">
                                                    <label class="uk-form-label" for="form-stacked-text">{"Name"}</label>
                                                    <input
                                                        class="uk-input"
                                                        type="text"
                                                        placeholder="your name"
                                                        required=true
                                                        value=self.form_data.name.clone()
                                                        oninput=self.link.callback(|e: InputData| Msg::UpdateName(e.value))
                                                    />
                                                </div>
                                            </div>

                                            <div class="uk-margin">
                                                <div class="uk-inline uk-width-1-1">
                                                    <label class="uk-form-label" for="form-stacked-text">{"Email"}</label>
                                                    <input
                                                        class="uk-input"
                                                        type="email"
                                                        placeholder="yourname@example.com"
                                                        required=true
                                                        value=self.form_data.email.clone()
                                                        oninput=self.link.callback(|e: InputData| Msg::UpdateEmail(e.value))
                                                    />
                                                </div>
                                            </div>

                                            <div class="uk-margin">
                                                <div class="uk-inline uk-width-1-1">
                                                    <label class="uk-form-label" for="form-stacked-text">{"Password"}</label>
                                                    <input
                                                        class="uk-input"
                                                        type="password"
                                                        placeholder="password"
                                                        required=true
                                                        value=self.form_data.password.clone()
                                                        oninput=self.link.callback(|e: InputData| Msg::UpdatePassword(e.value))
                                                    />
                                                </div>
                                            </div>
                                        </form>
                                        <div class="uk-margin">
                                            <button
                                                onclick=self.link.callback(|_| Msg::Submit)
                                                class="uk-button uk-button-large uk-width-1-1 button"
                                            >
                                                {"Register"}
                                            </button>
                                        </div>
                                        <div class="uk-text-small uk-text-center">
                                            <Anchor
                                                route=AppRoute::Data
                                            >
                                                {"Check data"}
                                            </Anchor>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
