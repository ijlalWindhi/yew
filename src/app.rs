use data::Data;
use register::Register;
use routes::AppRoute;
use yew::{prelude::*, virtual_dom::VNode};
use yew_router::prelude::*;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> VNode {
        html! {
            <Router<AppRoute>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Register => html! {<Register/>},
                        AppRoute::Data => html! {<Data/>},
                    }
                })
            />
        }
    }
}
