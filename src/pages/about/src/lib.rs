use navbar::Navbar;
use yew::prelude::*;

pub struct About {}

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        About {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Navbar/>
                <article id="home" class="home">
                    <h1 class="play fadeInLeft">
                        {"About"}
                    </h1>
                </article>
            </>
        }
    }
}
