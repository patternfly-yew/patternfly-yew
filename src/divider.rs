use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Divider;

impl Component for Divider {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        return html! {<li class="pf-c-divider" role="separator"></li>};
    }
}
