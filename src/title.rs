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
    } else {
        match props.level {
            Level::H1 => classes.push(Size::XXLarge.as_class()),
            Level::H2 => classes.push(Size::XLarge.as_class()),
            Level::H3 => classes.push(Size::Large.as_class()),
            Level::H4 => classes.push(Size::Medium.as_class()),
            Level::H5 => classes.push(Size::Medium.as_class()),
            Level::H6 => classes.push(Size::Medium.as_class()),
        }
    }

    let heading = match props.level {
        Level::H1 => html! {<h1>{ for props.children.iter() }</h1>},
        Level::H2 => html! {<h2>{ for props.children.iter() }</h2>},
        Level::H3 => html! {<h3>{ for props.children.iter() }</h3>},
        Level::H4 => html! {<h4>{ for props.children.iter() }</h4>},
        Level::H5 => html! {<h5>{ for props.children.iter() }</h5>},
        Level::H6 => html! {<h6>{ for props.children.iter() }</h6>},
    };
    html! { <div class={classes}>{heading}</div> }
}
