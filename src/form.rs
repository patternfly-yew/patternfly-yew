use yew::html::ChildrenRenderer;
use yew::Properties;
use yew::{html, Component, ComponentLink, Html};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: ChildrenRenderer<Html>,
    // pub weak_link: WeakComponentLink<Page>,
}

pub struct Form {
    props: Props,
}

impl Component for Form {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        // props.weak_link.borrow_mut().replace(Some(link));
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
            <form novalidate=true class="pf-c-form">
                { for self.props.children.iter().map(|child|{
                        child
                }) }
            </form>
        }
    }
}
