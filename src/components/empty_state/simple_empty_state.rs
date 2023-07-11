use crate::components::empty_state::*;
use std::fmt::Debug;

use crate::prelude::{Action, Button, ButtonVariant, Icon, Size};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SimpleEmptyStateProperties {
    #[prop_or_default]
    pub children: Children,

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

#[function_component(SimpleEmptyState)]
pub fn simple_empty_state(props: &SimpleEmptyStateProperties) -> Html {
    let icon = match props.icon {
        Some(icon) => vec![html!({ icon })],
        None => Vec::new(),
    };

    let variant = match props.size {
        Some(Size::XLarge) | Some(Size::XXLarge) | Some(Size::XXXLarge) | Some(Size::XXXXLarge) => {
            EmptyStateVariant::XL
        }
        _ => EmptyStateVariant::FULL,
    };

    html! (
        <EmptyState variant={variant} full_height={props.full_height}>
            <EmptyStateHeader
                title_text={html!(&props.title)}
                heading_level={EmptyStateHeadingLevel::H4}
                icon={html!(
                    <EmptyStateIcon icon={Children::new(icon)} />
                )}
            />
            <EmptyStateBody>
                { for props.children.iter() }
            </EmptyStateBody>
            <EmptyStateFooter>
                if let Some(action) = &props.primary {
                    <EmptyStateActions>
                        <Button
                            label={action.label.clone()}
                            variant={ButtonVariant::Primary}
                            onclick={action.callback.reform(|_|{})}
                        />
                    </EmptyStateActions>
                }
                if !props.secondaries.is_empty() {
                    <EmptyStateActions>
                        { for props.secondaries.iter().map(|action|{
                            html!{
                                <Button label={action.label.clone()} variant={ButtonVariant::Link} onclick={action.callback.reform(|_|{})}/>
                            }
                        }) }
                    </EmptyStateActions>
                }
            </EmptyStateFooter>
        </EmptyState>
    )
}
