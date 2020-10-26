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
}

#[derive(Clone, PartialEq)]
pub struct Card {
    props: Props,
}

impl Component for Card {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
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
            <div class=classes>
                { self.title() }
                { for self.props.children.iter().map(|child|{
                    html_nested!{
                        <div class="pf-c-card__body">
                            { child }
                        </div>
                    }
                }) }
                { self.footer() }
            </div>
        }
    }
}

impl Card {
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
