use std::fmt::Debug;
use yew::prelude::*;

use super::column::TableColumn;

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct TableHeaderProps {
    #[prop_or_default]
    pub sticky: bool,
    #[prop_or_default]
    pub children: ChildrenWithProps<TableColumn>,
    #[prop_or_default]
    pub(crate) expandable: bool,
    #[prop_or_default]
    pub hide_actions: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TableHeader {
    props: TableHeaderProps,
}

impl Component for TableHeader {
    type Message = ();
    type Properties = TableHeaderProps;

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
        return html! {
            <thead>

                <tr role="row">

                    { if self.props.expandable {
                        html!{<td></td>}
                    } else {
                        html!{}
                    }}

                    { for self.props.children.iter() }

                    { if self.props.hide_actions {html!{}} else {html!{
                        <td></td>
                    }}}

                </tr>

            </thead>
        };
    }
}
