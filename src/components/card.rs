//! Card
use crate::Icon;
use yew::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum CardSelection {
    #[default]
    None,
    Disabled,
    Selectable {
        selected: bool,
    },
}

/// Properties for [`Card`]
#[derive(Clone, PartialEq, Properties)]
pub struct CardProperties {
    pub children: Children,
    #[prop_or_default]
    pub title: Option<Html>,
    #[prop_or_default]
    pub footer: Option<Html>,
    #[prop_or_default]
    pub compact: bool,
    #[prop_or_default]
    pub flat: bool,
    #[prop_or_default]
    pub onclick: Callback<yew::MouseEvent>,
    #[prop_or_default]
    pub expandable: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub full_height: bool,
    #[prop_or_default]
    pub rounded: bool,
    #[prop_or_default]
    pub selection: CardSelection,
    #[prop_or_default]
    pub class: Classes,
}

/// Card component
///
/// > A **card** is a square or rectangular container that can contain any kind of content. Cards symbolize units of information, and each one acts as an entry point for users to access more details. For example, in dashboards and catalog views, cards function as a preview of a detailed page. Cards may also be used in data displays like card views, or for positioning content on a page.
///
/// See: <https://www.patternfly.org/v4/components/card>
///
/// ## Properties
///
/// Defined by [`CardProperties`].
pub struct Card {
    expanded: bool,
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug)]
pub enum Msg {
    Toggle,
}

impl Component for Card {
    type Message = Msg;
    type Properties = CardProperties;

    fn create(_: &Context<Self>) -> Self {
        Self { expanded: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Toggle => {
                self.expanded = !self.expanded;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-c-card");

        if ctx.props().compact {
            classes.push("pf-m-compact");
        }

        if ctx.props().expandable && self.expanded {
            classes.push("pf-m-expanded");
        }

        if ctx.props().large {
            classes.push("pf-m-display-lg");
        }

        if ctx.props().flat {
            classes.push("pf-m-flat");
        }

        match ctx.props().selection {
            CardSelection::None => {}
            CardSelection::Disabled => {
                classes.push("pf-m-non-selectable-raised");
            }
            CardSelection::Selectable { selected } => {
                classes.push("pf-m-selectable-raised");
                if selected {
                    classes.push("pf-m-selected-raised");
                }
            }
        }

        if ctx.props().full_height {
            classes.push("pf-m-full-height");
        }

        if ctx.props().rounded {
            classes.push("pf-m-rounded");
        }

        classes.extend(ctx.props().class.clone());

        html! (
            <div
                class={classes}
                onclick={&ctx.props().onclick}
            >
                { self.header(ctx) }
                if self.expanded || !ctx.props().expandable {
                    { self.body(ctx) }
                }
                { self.footer(ctx) }
            </div>
        )
    }
}

impl Card {
    fn body(&self, ctx: &Context<Self>) -> Html {
        html! {
            {for ctx.props().children.iter().map(|child|{
                html_nested!{
                    <div class="pf-c-card__body">
                        { child }
                    </div>
                }
            })}
        }
    }

    fn header(&self, ctx: &Context<Self>) -> Html {
        if ctx.props().expandable {
            html! {
                <div class="pf-c-card__header">
                    <div class="pf-c-card__header-toggle">
                        <button
                            class="pf-c-button pf-m-plain"
                            type="button"
                            aria-label="Details"
                            onclick={ctx.link().callback(|_|Msg::Toggle)}
                            >
                            <span class="pf-c-card__header-toggle-icon"> { Icon::AngleRight } </span>
                        </button>
                    </div>
                    { self.title(ctx) }
                </div>
            }
        } else {
            self.title(ctx)
        }
    }

    fn title(&self, ctx: &Context<Self>) -> Html {
        match &ctx.props().title {
            Some(t) => html! {
                <div class="pf-c-card__title">
                    { t.clone() }
                </div>
            },
            None => html! {},
        }
    }

    fn footer(&self, ctx: &Context<Self>) -> Html {
        match &ctx.props().footer {
            Some(f) => html! {
                <div class="pf-c-card__footer">
                    { f.clone() }
                </div>
            },
            None => html! {},
        }
    }
}
