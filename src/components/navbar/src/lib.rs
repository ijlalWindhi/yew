// use route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct Navbar {}

impl Component for Navbar {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Navbar {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <header>
                <nav class="uk-navbar-container uk-margin" uk-navbar="" style="padding:0 10%; background-color: #ea4100;">
                    <div class="uk-navbar-left">
                        <ul class="uk-navbar-nav">
                            <li class="uk-active"><a href="#"><img src="../../../../assets/Logo.svg" alt="logo"/></a></li>
                        </ul>
                    </div>
                    <div class="uk-navbar-right">
                        <ul class="uk-navbar-nav">
                            <li>
                                // <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                                <a href="#">{"Home"}</a>
                            </li>
                            <li>
                                // <Link<Route> to={Route::About}>{ "About" }</Link<Route>>
                                <a href="#">{"Home"}</a>
                            </li>
                        </ul>
                    </div>
                </nav>
            </header>
        }
    }
}
