mod child;
mod item;
mod variant;

use crate::ouia;
use crate::prelude::{Ouia, OuiaComponentType, OuiaSafe};
pub use child::*;
pub use item::*;
pub use variant::*;
use yew::html::ChildrenRenderer;
use yew::prelude::*;

const OUIA: Ouia = ouia!("ProgressStepper");

/// Properties for [`ProgressStepper`]
#[derive(PartialEq, Properties)]
pub struct ProgressStepperProperties {
    #[prop_or_default]
    pub centered: bool,
    #[prop_or_default]
    pub vertical: bool,
    #[prop_or_default]
    pub responsive: bool,
    #[prop_or_default]
    pub compact: bool,
    #[prop_or_default]
    pub children: ChildrenRenderer<ProgressStepperChildVariant>,

    /// OUIA Component id
    #[prop_or_default]
    pub ouia_id: Option<String>,
    /// OUIA Component Type
    #[prop_or(OUIA.component_type())]
    pub ouia_type: OuiaComponentType,
    /// OUIA Component Safe
    #[prop_or(OuiaSafe::TRUE)]
    pub ouia_safe: OuiaSafe,
}

#[component]
pub fn ProgressStepper(props: &ProgressStepperProperties) -> Html {
    let ouia_id = use_memo(props.ouia_id.clone(), |id| {
        id.clone().unwrap_or(OUIA.generated_id())
    });

    let mut class = classes!("pf-v6-c-progress-stepper");

    if props.centered {
        class.push(classes!("pf-m-center"));
    }
    if props.vertical {
        class.push(classes!("pf-m-vertical"));
    }
    if props.responsive {
        class.push(classes!("pf-m-vertical-on-lg", "pf-m-horizontal-on-2xl"));
    }
    if props.compact {
        class.push(classes!("pf-m-compact"));
    }

    html!(
        <ol
            {class}
            data-ouia-component-id={(*ouia_id).clone()}
            data-ouia-component-type={props.ouia_type}
            data-ouia-safe={props.ouia_safe}
        >
            { for props.children.iter() }
        </ol>
    )
}

#[derive(Clone, Copy, PartialEq)]
pub enum ProgressStepperStepStatus {
    Default,
    Success,
    Info,
    Pending,
    Warning,
    Danger,
}
