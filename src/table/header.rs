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
pub struct TableHeader {}

impl Component for TableHeader {
    type Message = ();
    type Properties = TableHeaderProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        return html! {
            <thead>

                <tr role="row">

                    { if ctx.props().expandable {
                        html!{<td></td>}
                    } else {
                        html!{}
                    }}

                    { for ctx.props().children.iter() }

                    { if ctx.props().hide_actions {html!{}} else {html!{
                        <td></td>
                    }}}

                </tr>

            </thead>
        };
    }
}
