mod child;
mod group;
mod item;

pub use child::*;
pub use group::*;
pub use item::*;

use crate::AsClasses;
use yew::{html::ChildrenRenderer, prelude::*};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ToolbarElementModifier {
    Hidden,
    Visible,
    Left,
    Right,
}

impl AsClasses for ToolbarElementModifier {
    fn extend(&self, classes: &mut Classes) {
        classes.push(match self {
            Self::Hidden => "pf-m-hidden",
            Self::Visible => "pf-m-visible",
            Self::Left => "pf-m-align-left",
            Self::Right => "pf-m-align-right",
        });
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarProps {
    #[prop_or_default]
    pub children: ChildrenRenderer<ToolbarChildVariant>,
    #[prop_or_default]
    pub id: String,
}

#[function_component(Toolbar)]
pub fn toolbar(props: &ToolbarProps) -> Html {
    html! {
        <div id={props.id.clone()} class="pf-c-toolbar">
            <div class="pf-c-toolbar__content">
                <div class="pf-c-toolbar__content-section">
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}
