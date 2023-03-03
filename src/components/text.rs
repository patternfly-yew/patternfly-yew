use yew::prelude::*;

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Copy, Debug)]
pub enum TextVariant {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    P,
    A,
    Small,
    Blockquote,
    Pre,
}

impl Default for TextVariant {
    fn default() -> Self {
        TextVariant::P
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TextProperties {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub component: TextVariant,
    #[prop_or_default]
    pub visited: bool,
}

#[function_component(Text)]
pub fn text(props: &TextProperties) -> Html {
    let mut classes = Classes::from("pf-c-content");

    if props.visited {
        classes.push("pf-m-visited");
    }

    let component = match props.component {
        TextVariant::H1 => html! { <h1>{ for props.children.iter() }</h1>},
        TextVariant::H2 => html! { <h2>{ for props.children.iter() }</h2>},
        TextVariant::H3 => html! { <h3>{ for props.children.iter() }</h3>},
        TextVariant::H4 => html! { <h4>{ for props.children.iter() }</h4>},
        TextVariant::H5 => html! { <h5>{ for props.children.iter() }</h5>},
        TextVariant::H6 => html! { <h6>{ for props.children.iter() }</h6>},
        TextVariant::P => html! { <p>{ for props.children.iter() }</p>},
        TextVariant::A => html! { <a href="#">{ for props.children.iter() }</a>},
        TextVariant::Small => {
            html! { <small>{ for props.children.iter() }</small>}
        }
        TextVariant::Blockquote => {
            html! { <blockquote>{ for props.children.iter() }</blockquote>}
        }
        TextVariant::Pre => html! { <pre>{ for props.children.iter() }</pre>},
    };
    html! { <div class={classes}>{component}</div> }
}
