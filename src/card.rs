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
    props: Props,
    link: ComponentLink<Self>,
    expanded: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum Msg {
    Toggle,
}

impl Component for Card {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            expanded: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Toggle => {
                self.expanded = !self.expanded;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("pf-c-card");

        if self.props.compact {
            classes.push("pf-m-compact");
        }

        if self.props.expandable && self.expanded {
            classes.push("pf-m-expanded");
        }

        if self.props.large {
            classes.push("pf-m-m-display-lg");
        }

        if self.props.flat {
            classes.push("pf-m-flat");
        }

        if self.props.hoverable {
            classes.push("pf-m-hoverable");
        }

        if self.props.selectable {
            classes.push("pf-m-selectable");
        }

        if self.props.selected {
            classes.push("pf-m-selected");
        }

        html! {
            <div
                class=classes
                onclick=&self.props.onclick
                >
                { self.header() }
                {
                    if self.expanded || !self.props.expandable {
                        self.body()
                    } else {
                        html!{}
                    }
                }
                { self.footer() }
            </div>
        }
    }
}

impl Card {
    fn body(&self) -> Html {
        html! {
            {for self.props.children.iter().map(|child|{
                html_nested!{
                    <div class="pf-c-card__body">
                        { child }
                    </div>
                }
            })}
        }
    }

    fn header(&self) -> Html {
        if self.props.expandable {
            html! {
                <div class="pf-c-card__header">
                    <div class="pf-c-card__header-toggle">
                        <button
                            class="pf-c-button pf-m-plain"
                            type="button"
                            aria-label="Details"
                            onclick=self.link.callback(|_|Msg::Toggle)
                            >
                            <span class="pf-c-card__header-toggle-icon"> { Icon::AngleRight } </span>
                        </button>
                    </div>
                    { self.title() }
                </div>
            }
        } else {
            self.title()
        }
    }

    fn title(&self) -> Html {
        match &self.props.title {
            Some(t) => html! {
                <div class="pf-c-card__title">
                    { t.clone() }
                </div>
            },
            None => html! {},
        }
    }

    fn footer(&self) -> Html {
        match &self.props.footer {
            Some(f) => html! {
                <div class="pf-c-card__footer">
                    { f.clone() }
                </div>
            },
            None => html! {},
        }
    }
}
