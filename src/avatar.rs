use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub src: String,
    #[prop_or("Avatar image".into())]
    pub alt: String,
}

#[function_component(Avatar)]
pub fn avatar(props: &Props) -> Html {
    html! {
        <img
            class="pf-c-avatar"
            src={props.src.clone()}
            alt={props.alt.clone()}
            />
    }
}
