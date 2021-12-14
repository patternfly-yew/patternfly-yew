use yew::prelude::*;

use crate::Size;

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Copy, Debug)]
pub enum Level {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl Default for Level {
    fn default() -> Self {
        Level::H1
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub level: Level,
    #[prop_or_default]
    pub size: Option<Size>,
}

#[function_component(Title)]
pub fn title(props: &Props) -> Html {
    let mut classes = Classes::from("pf-c-title");

    if let Some(size) = props.size {
        classes.push(size.as_class());
    }

    match props.level {
        Level::H1 => html! {<h1 class={classes}>{ for props.children.iter() }</h1>},
        Level::H2 => html! {<h2 class={classes}>{ for props.children.iter() }</h2>},
        Level::H3 => html! {<h3 class={classes}>{ for props.children.iter() }</h3>},
        Level::H4 => html! {<h4 class={classes}>{ for props.children.iter() }</h4>},
        Level::H5 => html! {<h5 class={classes}>{ for props.children.iter() }</h5>},
        Level::H6 => html! {<h6 class={classes}>{ for props.children.iter() }</h6>},
    }
}
