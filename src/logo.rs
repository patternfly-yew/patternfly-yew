use yew::Properties;
use yew::{html, Component, ComponentLink, Html};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub src: String,
    pub alt: String,
}

pub struct Logo {
    props: Props,
}

impl Component for Logo {
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
        html! {
            <img class="pf-c-brand" src={self.props.src.clone()} alt={self.props.alt.clone()} />
        }
    }
}
