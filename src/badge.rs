use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub read: bool,
}

pub struct Badge {
    props: Props,
}

impl Component for Badge {
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
        let mut classes = Classes::from("pf-c-badge");

        if self.props.read {
            classes.push("pf-m-read");
        } else {
            classes.push("pf-m-unread");
        }

        html! {
            <span class=classes>
                { for self.props.children.iter() }
            </span>
        }
    }
}
