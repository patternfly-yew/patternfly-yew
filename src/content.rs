use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[function_component(Content)]
pub fn content(props: &Props) -> Html {
    html! {
        <div class="pf-c-content">
            { for props.children.iter() }
        </div>
    }
}
