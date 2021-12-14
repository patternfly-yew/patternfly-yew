use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub read: bool,
}

#[function_component(Badge)]
pub fn badge(props: &Props) -> Html {
    let mut classes = Classes::from("pf-c-badge");

    if props.read {
        classes.push("pf-m-read");
    } else {
        classes.push("pf-m-unread");
    }

    html! {
        <span class={classes}>
            { for props.children.iter() }
        </span>
    }
}
