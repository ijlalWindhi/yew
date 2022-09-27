use navbar::Navbar;
use yew::prelude::*;

pub struct Home {}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Home {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <Navbar/>
            <article id="home" class="home">
                <div class="content">
                <h1 class="play fadeInLeft">
                    {"Discover your next place to travel in Taiwan"}
                </h1>
                <p class="play fadeInRight">
                    {"Since 2020, we've guided more than 100.000 people to enjoy their best Taiwan Experience"}
                </p>
                    <a href="#"><button class="play fadeInUp">{"Let's Explore"}</button></a>
                </div>
                <div class="image play fadeIn">
                    <img src="../../../../assets/Hero Section.svg" alt="" />
                </div>
            </article>
            </>
        }
    }
}
