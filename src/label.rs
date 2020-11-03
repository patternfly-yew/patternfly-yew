use crate::{Button, Icon, Variant};
use yew::prelude::*;

use strum_macros::{Display, EnumIter, EnumString};

#[derive(Copy, Clone, Display, Debug, PartialEq, Eq, EnumIter, EnumString)]
pub enum Color {
    Grey,
    Blue,
    Green,
    Orange,
    Red,
    Purple,
    Cyan,
}

impl Default for Color {
    fn default() -> Self {
        Self::Grey
    }
}

impl From<Color> for Classes {
    fn from(color: Color) -> Self {
        match color {
            Color::Grey => Classes::new(),
            Color::Blue => Classes::from("pf-m-blue"),
            Color::Green => Classes::from("pf-m-green"),
            Color::Orange => Classes::from("pf-m-orange"),
            Color::Red => Classes::from("pf-m-red"),
            Color::Purple => Classes::from("pf-m-purple"),
            Color::Cyan => Classes::from("pf-m-cyan"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub color: Color,
    #[prop_or_default]
    pub outline: bool,
    #[prop_or_default]
    pub overflow: bool,
    #[prop_or_default]
    pub icon: Option<Icon>,
    #[prop_or_default]
    pub onclose: Option<Callback<()>>,
    #[prop_or_default]
    pub href: String,
}

pub struct Label {
    props: Props,
}

impl Component for Label {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("pf-c-label");

        classes = classes.extend(Classes::from(self.props.color));

        if self.props.outline {
            classes.push("pf-m-outline");
        }

        if self.props.overflow {
            classes.push("pf-m-overflow");
        }

        let content = |content: Html| {
            if self.props.href.is_empty() {
                html! {<span class="pf-c-label__content">{content}</span>}
            } else {
                html! {<a class="pf-c-label__content" href=self.props.href.clone()>{content}</a>}
            }
        };

        return html! {
            <span class=classes>
                { content (
                    html!{
                        <>
                            { self.render_icon() }
                            { &self.props.label }
                        </>
                    }
                )}
                { self.render_close() }
            </span>
        };
    }
}

impl Label {
    fn render_icon(&self) -> Html {
        if let Some(icon) = &self.props.icon {
            html! {
                <span class="pf-c-label__icon">
                    { icon.as_html() }
                </span>
            }
        } else {
            html! {}
        }
    }

    fn render_close(&self) -> Html {
        if let Some(onclose) = &self.props.onclose {
            let onclose = onclose.reform(|_| {});
            return html! {
                <Button variant=Variant::Plain icon=Icon::Times onclick=onclose/>
            };
        } else {
            return html! {};
        }
    }
}
