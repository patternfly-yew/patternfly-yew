use yew::prelude::*;

use crate::{Action, Button, ExtendClasses, Icon, Size, Title, Variant};

#[derive(Clone, PartialEq, Properties)]
pub struct EmptyStateProps {
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

impl EmptyStateProps {
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

#[function_component(EmptyState)]
pub fn empty_state(props: &EmptyStateProps) -> Html {
    let mut classes = Classes::from("pf-c-empty-state");

    if props.full_height {
        classes.push("pf-m-full-height");
    }

    classes.extend_from(&props.size);

    html! (
        <div class={classes}>
            <div class="pf-c-empty-state__content">
                if let Some(icon) = &props.icon {
                    { icon.with_classes(Classes::from("pf-c-empty-state__icon")) }
                }
                <Title size={ props.title_size() }>{&props.title}</Title>
                <div class="pf-c-empty-state__body">
                    { for props.children.iter() }
                </div>

                if let Some(action) = &props.primary {
                    <Button label={action.label.clone()} variant={Variant::Primary} onclick={action.callback.reform(|_|{})}/>
                }

                if !props.secondaries.is_empty() {
                    <div class="pf-c-empty-state__secondary">
                        { for props.secondaries.iter().map(|action|{
                            html!{
                                <Button label={action.label.clone()} variant={Variant::Link} onclick={action.callback.reform(|_|{})}/>
                            }
                        }) }
                    </div>
                }
            </div>
        </div>
    )
}
