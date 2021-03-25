use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub gutter: bool,
}

pub struct Stack {
    props: Props,
}

impl Component for Stack {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
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
        let mut classes = Classes::from("pf-l-stack");

        if self.props.gutter {
            classes.push("pf-m-gutter");
        }

        return html! {
            <div class=classes>
                { for self.props.children.iter() }
            </div>
        };
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct StackItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub fill: bool,
}

pub struct StackItem {
    props: StackItemProps,
}

impl Component for StackItem {
    type Message = ();
    type Properties = StackItemProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
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
        let mut classes = Classes::from("pf-l-stack__item");

        if self.props.fill {
            classes.push("pf-m-fill");
        }

        return html! {
            <div class=classes>
                { for self.props.children.iter() }
            </div>
        };
    }
}
