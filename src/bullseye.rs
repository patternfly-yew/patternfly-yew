use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

/// Bullseye layout.
///
/// https://www.patternfly.org/v4/layouts/bullseye
#[derive(Clone, PartialEq)]
pub struct Bullseye {
    props: Props,
}

impl Component for Bullseye {
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
        html! {
            <div class="pf-l-bullseye">
                <div class="pf-l-bullseye__item">
                    { for self.props.children.iter() }
                </div>
            </div>
        }
    }
}
