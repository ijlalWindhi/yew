use about::About;
use home::Home;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {<Home/>},
        Route::About => html! {<About/>},
        // Route::Dashboard => html! {<Dashboard/>},
        Route::NotFound => html! {<h1>{"404 Page Not Found"}</h1>},
    }
}
