use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(true)]
    pub open: bool,
}

#[derive(Clone, PartialEq)]
pub struct PageSidebar {
    props: Props,
}

impl Component for PageSidebar {
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
        let collapsed = match self.props.open {
            true => vec![],
            false => vec!["pf-m-collapsed"],
        };

        html! {
            <div class=("pf-c-page__sidebar",collapsed)>
                <div class="pf-c-page__sidebar-body">
                    { for self.props.children.iter() }
                </div>
            </div>
        }
    }
}
