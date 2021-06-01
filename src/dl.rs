use yew::prelude::*;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct DescriptionListProps {
    #[prop_or_default]
    pub children: Children,
}

pub struct DescriptionList {
    props: DescriptionListProps,
}

impl Component for DescriptionList {
    type Message = ();
    type Properties = DescriptionListProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
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
        let classes = Classes::from("pf-c-description-list");

        return html! {
            <dl class=classes>
                { for self.props.children.iter() }
            </dl>
        };
    }
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct DescriptionGroupProps {
    pub term: String,
    #[prop_or_default]
    pub children: Children,
}

pub struct DescriptionGroup {
    props: DescriptionGroupProps,
}

impl Component for DescriptionGroup {
    type Message = ();
    type Properties = DescriptionGroupProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
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
        html! {
            <div class="pf-c-description-list__group">
                <dt class="pf-c-description-list__term">{&self.props.term}</dt>
                <dd class="pf-c-description-list__description">
                    <div class="pf-c-description-list__text">
                        { for self.props.children.iter() }
                    </div>
                </dd>
            </div>
        }
    }
}
