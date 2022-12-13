use yew::prelude::*;

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Copy, Debug)]
pub enum TextVariant {
    Paragraph,
    Link,
    Small,
    Blockquote,
    Preformated,
}

impl Default for TextVariant {
    fn default() -> Self {
        TextVariant::Paragraph
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub component: TextVariant,
    #[prop_or_default]
    pub visited: bool,
    #[prop_or_default]
    pub link: Option<String>,
}

#[function_component(Text)]
pub fn text(props: &Props) -> Html {
    let mut classes = Classes::from("pf-c-content");

    if props.visited {
        classes.push("pf-m-visited");
    }

    let component = match props.component {
        TextVariant::Paragraph => html! { <p>{ for props.children.iter() }</p>},
        TextVariant::Link => {
            html! { <a href={props.link.clone().unwrap_or(String::from("#"))}>{ for props.children.iter() }</a>}
        }
        TextVariant::Small => {
            html! { <small>{ for props.children.iter() }</small>}
        }
        TextVariant::Blockquote => {
            html! { <blockquote>{ for props.children.iter() }</blockquote>}
        }
        TextVariant::Preformated => html! { <pre>{ for props.children.iter() }</pre>},
    };
    html! { <div class={classes}>{component}</div> }
}
