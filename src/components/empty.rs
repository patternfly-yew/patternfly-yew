//! Empty state
use yew::prelude::*;

use crate::prelude::{Action, Button, ButtonVariant, ExtendClasses, Icon, Size, Title};

/// Properties for [`EmptyState`]
#[derive(Clone, PartialEq, Properties)]
pub struct EmptyStateProperties {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub icon: Option<Icon>,
    #[prop_or_default]
    pub primary: Option<Action>,
    #[prop_or_default]
    pub secondaries: Vec<Action>,
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub full_height: bool,
}

impl EmptyStateProperties {
    fn title_size(&self) -> Size {
        match self.size {
            Some(Size::XLarge)
            | Some(Size::XXLarge)
            | Some(Size::XXXLarge)
            | Some(Size::XXXXLarge) => Size::XXXXLarge,
            _ => Size::Large,
        }
    }
}

/// Empty State component
///
/// > An **empty state** is a screen that is not yet populated with data or information. Empty states typically contain a short message and next steps for the user.
///
/// See: <https://www.patternfly.org/components/empty-state>
///
/// ## Properties
///
/// Define by [`EmptyStateProperties`].
#[function_component(EmptyState)]
pub fn empty_state(props: &EmptyStateProperties) -> Html {
    let mut classes = Classes::from("pf-v6-c-empty-state");

    if props.full_height {
        classes.push("pf-m-full-height");
    }

    classes.extend_from(&props.size);

    html! (
        <div class={classes}>
            <div class="pf-v6-c-empty-state__content">
                <div class="pf-v6-c-empty-state__header">
                    if let Some(icon) = &props.icon {
                        <div class="pf-v6-c-empty-state__icon">
                            { *icon }
                        </div>
                    }
                    <Title size={ props.title_size() }>{ props.title.clone() }</Title>
                </div>
                <div class="pf-v6-c-empty-state__body">
                    { props.children.clone() }
                </div>
                if props.primary.is_some() || !props.secondaries.is_empty() {
                    <div class="pf-v6-c-empty-state__footer">
                        if let Some(action) = &props.primary {
                            <div class="pf-v6-c-empty-state__actions">
                                <Button label={action.label.clone()} variant={ButtonVariant::Primary} onclick={action.callback.reform(|_|{})}/>
                            </div>
                        }

                        if !props.secondaries.is_empty() {
                            <div class="pf-v6-c-empty-state__actions">
                                { for props.secondaries.iter().map(|action|{
                                    html!{
                                        <Button label={action.label.clone()} variant={ButtonVariant::Link} onclick={action.callback.reform(|_|{})}/>
                                    }
                                }) }
                            </div>
                        }
                    </div>
                }
            </div>
        </div>
    )
}
