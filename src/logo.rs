use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub src: String,
    pub alt: String,
}

#[function_component(Logo)]
pub fn logo(props: &Props) -> Html {
    html! {
        <img class="pf-c-brand" src={props.src.clone()} alt={props.alt.clone()} />
    }
}
