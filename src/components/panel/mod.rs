//! Panel

use crate::prelude::{AsClasses, ChildrenProperties, ExtendClasses};
use yew::prelude::*;

/// Properties for [`Panel`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PanelProperties {
    pub children: Html,

    #[prop_or_default]
    pub variant: PanelVariant,
    #[prop_or_default]
    pub scrollable: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PanelVariant {
    #[default]
    Default,
    Bordered,
    Raised,
}

impl AsClasses for PanelVariant {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Default => {}
            Self::Bordered => classes.push(classes!("pf-m-bordered")),
            Self::Raised => classes.push(classes!("pf-m-raised")),
        }
    }
}

/// The Panel component.
///
/// > The **panel** component is a container that supports flexible content layouts. It can be used to house other components such as fields, forms, videos, buttons, and more. The panel should not be confused with the drawer component, which allows you to surface information via a collapsable container.
///
/// See: <https://www.patternfly.org/components/panel>
///
/// ## Example
///
/// ```rust
/// use patternfly_yew::prelude::*;
/// use yew::prelude::*;
///
/// #[function_component(Example)]
/// pub fn example() -> Html {
///   html!(
///     <Panel>
///       <PanelHeader>{"Header"}</PanelHeader>
///       <Divider/>
///       <PanelMain>
///         <PanelMainBody>
///             {"Main body"}
///         </PanelMainBody>
///       </PanelMain>
///       <PanelFooter>{"Footer"}</PanelFooter>
///     </Panel>
///   )
/// }
/// ```
#[function_component(Panel)]
pub fn panel(props: &PanelProperties) -> Html {
    let mut class = classes!("pf-v6-c-panel");

    class.extend_from(&props.variant);

    if props.scrollable {
        class.push(classes!("pf-m-scrollable"));
    }

    html!(
        <div {class}>
            { props.children.clone() }
        </div>
    )
}

#[function_component(PanelMain)]
pub fn panel_main(props: &ChildrenProperties) -> Html {
    html!(
        <div class="pf-v6-c-panel__main">
            { props.children.clone() }
        </div>
    )
}

#[function_component(PanelMainBody)]
pub fn panel_main_body(props: &ChildrenProperties) -> Html {
    html!(
        <div class="pf-v6-c-panel__main-body">
            { props.children.clone() }
        </div>
    )
}

#[function_component(PanelHeader)]
pub fn panel_header(props: &ChildrenProperties) -> Html {
    html!(
        <div class="pf-v6-c-panel__header">
            { props.children.clone() }
        </div>
    )
}

#[function_component(PanelFooter)]
pub fn panel_footer(props: &ChildrenProperties) -> Html {
    html!(
        <div class="pf-v6-c-panel__footer">
            { props.children.clone() }
        </div>
    )
}
