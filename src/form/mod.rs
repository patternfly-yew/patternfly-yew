mod area;
mod group;
mod input;
mod select;
mod validation;

pub use area::*;
pub use group::*;
pub use input::*;
pub use select::*;
pub use validation::*;

use crate::{Button, WithBreakpoints};
use std::fmt::{Display, Formatter};
use yew::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormHorizontal;

impl Display for FormHorizontal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("pf-m-horizontal")
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub horizontal: WithBreakpoints<FormHorizontal>,

    #[prop_or_default]
    pub limit_width: bool,

    #[prop_or_default]
    pub children: Children,
}

#[function_component(Form)]
pub fn form(props: &Props) -> Html {
    let mut classes = Classes::from("pf-c-form");

    classes.extend(props.horizontal.clone());

    if props.limit_width {
        classes.push("pf-m-limit-width");
    }

    html! {
        <form novalidate=true class={classes}>
            { for props.children.iter().map(|child|{
                    child
            }) }
        </form>
    }
}

//
// Action group
//

#[derive(Clone, PartialEq, Properties)]
pub struct ActionGroupProps {
    pub children: ChildrenWithProps<Button>,
}

#[function_component(ActionGroup)]
pub fn action_group(props: &ActionGroupProps) -> Html {
    html! {
        <div class="pf-c-form__group pf-m-action">
            <div class="pf-c-form__group-control">
                <div class="pf-c-form__actions">
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}

//
// Input group
//

#[derive(Clone, PartialEq, Properties)]
pub struct InputGroupProps {
    pub children: Children,
}

#[function_component(InputGroup)]
pub fn input_group(props: &InputGroupProps) -> Html {
    html! {
        <div class="pf-c-input-group">
            { for props.children.iter() }
        </div>
    }
}
