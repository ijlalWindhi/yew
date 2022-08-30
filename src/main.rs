use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(About)]
fn about() -> Html {
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.push(Route::Home));
    html! {
        <div>
            <h1>{ "About Section" }</h1>
            <button {onclick}>{ "Home" }</button>
        </div>
    }
}

#[function_component(Contact)]
fn contact() -> Html {
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.push(Route::Home));
    html! {
        <div>
            <h1>{ "Contact Section" }</h1>
            <button {onclick}>{ "Home" }</button>
        </div>
    }
}

#[function_component(Home)]
fn home() -> Html {
    let history = use_history().unwrap();
    html! {
        <div>
            <h1>{ "Home Section" }</h1>
            <button><Link<Route> to={Route::About}>{ "About" }</Link<Route>></button>
            <button><Link<Route> to={Route::Contact}>{ "Contact" }</Link<Route>></button>
        </div>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { 
            <Home />
         },
        Route::About => html! {
            <About />
        },
        Route::Contact => html! {
            <Contact />
        },
        Route::NotFound => html! { <h1>{ "404 PAGE NOT FOUND" }</h1> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<Main>();
}