use std::fmt::Debug;
use yew::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Properties)]
pub struct TableColumnProps {
    #[prop_or_default]
    pub label: Option<String>,
}

#[derive(Clone, Debug)]
pub struct TableColumn {
    props: TableColumnProps,
}

impl Component for TableColumn {
    type Message = ();
    type Properties = TableColumnProps;

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
        match &self.props.label {
            None => html! {},
            Some(label) => {
                html! {
                    <th role="columnheader">{ &label }</th>
                }
            }
        }
    }
}
