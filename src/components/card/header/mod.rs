use yew::prelude::*;

use super::CardContext;
use crate::prelude::*;

mod main;
mod selectable;

pub use main::*;
pub use selectable::*;

/// Actions that can be performed by the card.
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CardHeaderActionsObject {
    /// The actual actions.
    #[prop_or_default]
    pub actions: Html,
    /// Remove the offset of the position of the actions to the header content.
    /// This looks better if using large card titles or tall header images, for example.
    #[prop_or_default]
    pub has_no_offset: bool,
    /// Additional classes to the actions object.
    #[prop_or_default]
    pub class: Classes,
}

/// Properties of the CardHeader object.
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CardHeaderProperties {
    /// Content inside the header.
    #[prop_or_default]
    pub children: OptionalHtml,
    /// Additional classes to the object.
    #[prop_or_default]
    pub class: Classes,
    /// Actions that can be performed by the card.
    #[prop_or_default]
    pub actions: Option<CardHeaderActionsObject>,
    /// Actions that select the card or occur when clicking the card.
    #[prop_or_default]
    pub selectable_actions: Option<CardSelectableActionsObjectProperties>,
    /// Html id
    #[prop_or_default]
    pub id: String,
    /// Callback to run if a user clicks the expand toggle.
    #[prop_or_default]
    pub onexpand: Option<Callback<AttrValue>>,
    /// Sets whether the expand toggle should be on the right or the left of the card.
    #[prop_or_default]
    pub toggle_right_aligned: bool,
}

#[function_component(CardHeader)]
pub fn header(props: &CardHeaderProperties) -> Html {
    let context = use_context::<CardContext>().expect("Couldn't find card context");

    let is_clickable_xor_selectable = context.clickable != context.selectable;
    let has_actions = props.actions.as_ref().is_some();
    if has_actions && is_clickable_xor_selectable {
        log::warn!(
            "{} only cards should not contain any other actions. If you wish to include additional actions, use a clickable and selectable card",
            if context.clickable {
                "Clickable"
            } else {
                "Selectable"
            }
        );
    }

    let mut class = classes!(props.class.clone());
    if props.toggle_right_aligned {
        class.push("pf-m-toggle-right");
    }
    class.push("pf-v6-c-card__header");

    let selectable_actions = if context.clickable || context.selectable {
        props.selectable_actions.as_ref()
    } else {
        None
    };
    html! {
        <div {class} id={props.id.clone()}>
            if !props.toggle_right_aligned {
                <ExpandToggle onexpand={props.onexpand.clone()} />
            }
            if props.actions.is_some() || selectable_actions.is_some() {
                <CardActions
                    class={props.actions.as_ref().map(|a| a.class.clone()).unwrap_or_default()}
                    has_no_offset={props.actions.as_ref().map(|a| a.has_no_offset).unwrap_or_default()}
                >
                    if let Some(actions) = &props.actions {
                        {actions.actions.clone()}
                    }
                    if let Some(selectable_actions) = selectable_actions {
                        <CardSelectableActions class={selectable_actions.base.class.clone()}>
                            <CardSelectableActionsObject ..selectable_actions.clone() />
                        </CardSelectableActions>
                    }
                </CardActions>
            }
            if let Some(children) = props.children.as_ref() {
                <CardHeaderMain>{children.clone()}</CardHeaderMain>
            }
            if props.toggle_right_aligned {
                <ExpandToggle onexpand={props.onexpand.clone()} />
            }
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct ExpandToggleProperties {
    onexpand: Option<Callback<AttrValue>>,
}

#[function_component(ExpandToggle)]
fn expand_toggle(props: &ExpandToggleProperties) -> Html {
    let CardContext { card_id, .. } = use_context().expect("Couldn't get card context");
    let onclick = use_callback(
        (card_id.clone(), props.onexpand.clone()),
        |_, (id, onexpand)| {
            if let Some(f) = onexpand {
                f.emit(id.clone())
            }
        },
    );
    if props.onexpand.is_none() {
        return html!();
    }
    html! {
        <div class="pf-v6-c-card__header-toggle">
            <Button
                variant={ ButtonVariant::Plain }
                r#type={ ButtonType::Button }
                aria_label="Details"
                { onclick }
            >
                <span class="pf-v6-c-card__header-toggle-icon">
                    { Icon::AngleRight }
                </span>
            </Button>
        </div>
    }
}
