use crate::Icon;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
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
    pub hoverable: bool,
    #[prop_or_default]
    pub selectable: bool,
    #[prop_or_default]
    pub selected: bool,
    #[prop_or_default]
    pub onclick: Callback<yew::MouseEvent>,
    #[prop_or_default]
    pub expandable: bool,
    #[prop_or_default]
    pub large: bool,
}

pub struct Card {
    expanded: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum Msg {
    Toggle,
}

impl Component for Card {
    type Message = Msg;
    type Properties = Props;

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

        if ctx.props().hoverable {
            classes.push("pf-m-hoverable");
        }

        if ctx.props().selectable {
            classes.push("pf-m-selectable");
        }

        if ctx.props().selected {
            classes.push("pf-m-selected");
        }

        html! {
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
        }
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
