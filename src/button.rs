use crate::{Icon, Variant};
use yew::{html, Component, ComponentLink, Html};
use yew::{Callback, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub id: String,
    pub label: String,
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
        /*
        match msg {
            _ => {}
        }
         */

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
        html! {
            <button
                id=self.props.id.clone()
                class=("pf-c-button", self.props.variant.as_class())
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
                    <i class=(i.as_class())/>
                </span>
            },
            None => html! {},
        }
    }
}
