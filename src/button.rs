use crate::{Icon, Variant};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub onclick: Callback<yew::MouseEvent>,
    #[prop_or_default]
    pub variant: Variant,
    #[prop_or_default]
    pub icon: Option<Icon>,
}

pub struct Button {
    props: Props,
}

impl Component for Button {
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
        let mut classes = Classes::from("pf-c-button");
        classes = classes.extend(self.props.variant.as_classes());

        html! {
            <button
                id=&self.props.id
                class=classes
                type="button"
                onclick=self.props.onclick.clone()>

                { self.icon() }
                { self.props.label.clone() }

            </button>
        }
    }
}

impl Button {
    pub fn icon(&self) -> Html {
        match self.props.icon {
            Some(i) => html! {
                <span class="pf-c-button__icon pf-m-start">
                    { i.as_html() }
                </span>
            },
            None => html! {},
        }
    }
}
