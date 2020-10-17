use yew::Properties;
use yew::{html, Children, Component, ComponentLink, Html};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[derive(Clone, PartialEq)]
pub struct Content {
    props: Props,
}

impl Component for Content {
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
            <div class="pf-c-content">
                { for self.props.children.iter() }
            </div>
        }
    }
}
