use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub src: String,
    #[prop_or("Avatar image".into())]
    pub alt: String,
}

pub struct Avatar {
    props: Props,
}

impl Component for Avatar {
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
        let classes = Classes::from("pf-c-avatar");

        html! {
            <img
                class=classes
                src=self.props.src
                alt=self.props.alt
                />
        }
    }
}
