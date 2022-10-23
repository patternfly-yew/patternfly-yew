pub struct Spinner;

//use yew::{Classes, Component, Context, Html};
use yew::prelude::*;

impl Component for Spinner {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let classes = Classes::from("pf-c-spinner");
        html! {
            <svg
                class={classes}
                role="progressbar"
                viewBox="0 0 100 100"
                aria-label="Loading..."
            >
                <circle class="pf-c-spinner__path" cx="50" cy="50" r="45" fill="none" />
            </svg>
        }
    }
}
