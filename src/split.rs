use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: ChildrenWithProps<SplitItem>,
    #[prop_or_default]
    pub gutter: bool,
    #[prop_or_default]
    pub wrap: bool,
}

#[derive(Clone, PartialEq)]
pub struct Split {
    props: Props,
}

impl Component for Split {
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
        let mut classes = Classes::from("pf-l-split");

        if self.props.gutter {
            classes.push("pf-m-gutter");
        }

        if self.props.wrap {
            classes.push("pf-m-wrap");
        }

        return html! {
            <div class=classes>
            { for self.props.children.iter().map(|child|{
                html_nested!{
                    { child }
                }
            }) }
            </div>
        };
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct SplitItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub fill: bool,
}

pub struct SplitItem {
    props: SplitItemProps,
}

impl Component for SplitItem {
    type Message = ();
    type Properties = SplitItemProps;

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
        let mut classes = Classes::from("pf-l-split__item");

        if self.props.fill {
            classes.push("pf-m-fill");
        }

        return html! {
            <div class=classes>
                { self.props.children.clone() }
            </div>
        };
    }
}
