//! Modal
use crate::ouia;
use crate::prelude::use_backdrop;
use crate::prelude::wrap::wrapper_div_with_attributes;
use crate::utils::{Ouia, OuiaComponentType, OuiaSafe};
use yew::prelude::*;
use yew::virtual_dom::AttributeOrProperty;
use yew_hooks::{use_click_away, use_event_with_window};

const OUIA: Ouia = ouia!("ModalContent");

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum ModalVariant {
    #[default]
    None,
    Small,
    Medium,
    Large,
}

impl ModalVariant {
    pub fn as_classes(&self) -> Classes {
        match self {
            ModalVariant::None => classes!(),
            ModalVariant::Small => classes!("pf-m-sm"),
            ModalVariant::Medium => classes!("pf-m-md"),
            ModalVariant::Large => classes!("pf-m-lg"),
        }
    }
}

/// Properties for [`Modal`]
#[derive(Clone, PartialEq, Properties)]
pub struct ModalProperties {
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub description: String,
    #[prop_or_default]
    pub variant: ModalVariant,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub footer: Option<Html>,

    #[prop_or_default]
    pub onclose: Option<Callback<()>>,

    /// Disable close button
    #[prop_or(true)]
    pub show_close: bool,

    /// Disable closing the modal when the escape key is pressed
    #[prop_or_default]
    pub disable_close_escape: bool,
    /// Disable closing the modal when the user clicks outside the modal
    #[prop_or_default]
    pub disable_close_click_outside: bool,

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

/// Modal component
///
/// > A **modal** displays important information to a user without requiring them to navigate to a new page.
///
/// See: <https://www.patternfly.org/components/modal>
///
/// ## Properties
///
/// Defined by [`ModalProperties`].
///
/// ## Contexts
///
/// If the modal dialog is wrapped by a [`crate::prelude::BackdropViewer`] component and no
/// `onclose` callback is set, then it will automatically close the backdrop when the modal dialog
/// gets closed.
///
#[function_component(Modal)]
pub fn modal(props: &ModalProperties) -> Html {
    let ouia_id = use_memo(props.ouia_id.clone(), |id| {
        id.clone().unwrap_or(OUIA.generated_id())
    });
    let mut classes = props.variant.as_classes();
    classes.push("pf-v6-c-modal-box");

    let backdrop = use_backdrop();

    let onclose = use_memo((props.onclose.clone(), backdrop), |(onclose, backdrop)| {
        let onclose = onclose.clone();
        let backdrop = backdrop.clone();
        Callback::from(move |()| {
            if let Some(onclose) = &onclose {
                onclose.emit(());
            } else if let Some(backdrop) = &backdrop {
                backdrop.close();
            }
        })
    });

    // escape key
    {
        let disabled = props.disable_close_escape;
        let onclose = onclose.clone();
        use_event_with_window("keydown", move |e: KeyboardEvent| {
            if !disabled && e.key() == "Escape" {
                onclose.emit(());
            }
        });
    }

    // outside click

    let node_ref = use_node_ref();

    {
        let disabled = props.disable_close_click_outside;
        let onclose = onclose.clone();
        use_click_away(node_ref.clone(), move |_: Event| {
            if !disabled {
                onclose.emit(());
            }
        });
    }

    html! (
        <div
            class={classes}
            role="dialog"
            aria-modal="true"
            aria-labelledby="modal-title"
            aria-describedby="modal-description"
            ref={node_ref}
            data-ouia-component-id={(*ouia_id).clone()}
            data-ouia-component-type={props.ouia_type}
            data-ouia-safe={props.ouia_safe}
        >
            if props.show_close {
                <div class="pf-v6-c-modal-box__close">
                    <button
                        class="pf-v6-c-button pf-m-plain"
                        type="button"
                        aria-label="Close dialog"
                        onclick={onclose.reform(|_|())}
                    >
                        <i class="fas fa-times" aria-hidden="true"></i>
                    </button>
                </div>
            }

            <header class="pf-v6-c-modal-box__header">
                <h1
                    class="pf-v6-c-modal-box__title"
                    id="modal-title-modal-with-form"
                >{ &props.title }</h1>
            </header>


            if !&props.description.is_empty() {
                <div class="pf-v6-c-modal-box__body">
                    <p>{ &props.description }</p>
                </div>
            }

            { for props.children.iter().map(|c|{
                wrapper_div_with_attributes(c,
                    &[
                        ("class", AttributeOrProperty::Static("pf-v6-c-modal-box__body")),
                        ("id", AttributeOrProperty::Static("modal-description"))
                    ])
            }) }

            if let Some(footer) = &props.footer {
              <footer class="pf-v6-c-modal-box__footer">
                  { footer.clone() }
              </footer>
            }
        </div>
    )
}
