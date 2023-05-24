//! Modal
use crate::use_backdrop;
use wasm_bindgen::JsCast;
use web_sys::Node;
use yew::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ModalVariant {
    None,
    Small,
    Medium,
    Large,
}

impl ModalVariant {
    pub fn as_classes(&self) -> Vec<&'static str> {
        match self {
            ModalVariant::None => vec![],
            ModalVariant::Small => vec!["pf-m-sm"],
            ModalVariant::Medium => vec!["pf-m-md"],
            ModalVariant::Large => vec!["pf-m-lg"],
        }
    }
}

impl Default for ModalVariant {
    fn default() -> Self {
        ModalVariant::None
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

    /// Disable closing the modal when the escape key is pressed
    #[prop_or_default]
    pub disable_close_escape: bool,
    /// Disable closing the modal when the user clicks outside the modal
    #[prop_or_default]
    pub disable_close_click_outside: bool,
}

/// Modal component
///
/// > A **modal** displays important information to a user without requiring them to navigate to a new page.
///
/// See: <https://www.patternfly.org/v4/components/modal>
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
    let mut classes = props.variant.as_classes();
    classes.push("pf-c-modal-box");

    let backdrop = use_backdrop();

    let onclose = use_memo(
        |(onclose, backdrop)| {
            let onclose = onclose.clone();
            let backdrop = backdrop.clone();
            Callback::from(move |()| {
                if let Some(onclose) = &onclose {
                    onclose.emit(());
                } else if let Some(backdrop) = &backdrop {
                    backdrop.close();
                }
            })
        },
        (props.onclose.clone(), backdrop),
    );

    // escape key

    use_effect_with_deps(
        |(disabled, onclose)| {
            let listener = match *disabled {
                true => None,
                false => {
                    let onclose = onclose.clone();
                    Some(gloo_events::EventListener::new(
                        &gloo_utils::body(),
                        "keyup",
                        move |evt| {
                            if let Some(evt) = evt.dyn_ref::<KeyboardEvent>() {
                                if evt.key() == "Escape" {
                                    onclose.emit(());
                                }
                            }
                        },
                    ))
                }
            };
            move || {
                drop(listener);
            }
        },
        (props.disable_close_escape, onclose.clone()),
    );

    // outside click

    let node_ref = use_node_ref();

    {
        let node_ref = node_ref.clone();
        use_effect_with_deps(
            move |(disabled, onclose)| {
                let mut listeners = vec![];
                if !*disabled {
                    let mut register = |name: &'static str, node_ref: NodeRef| {
                        let onclose = onclose.clone();
                        listeners.push(gloo_events::EventListener::new(
                            &gloo_utils::body(),
                            name,
                            move |evt| {
                                if let Some(node) = node_ref.get() {
                                    if let Some(target_node) = evt.target_dyn_into::<Node>() {
                                        if !node.contains(Some(&target_node)) {
                                            onclose.emit(());
                                        }
                                    }
                                }
                            },
                        ));
                    };

                    register("mousedown", node_ref.clone());
                    register("touchstart", node_ref);
                }
                move || {
                    drop(listeners);
                }
            },
            (props.disable_close_click_outside, onclose.clone()),
        );
    }

    html! (
        <div
            class={classes}
            role="dialog"
            aria-modal="true"
            aria-labelledby="modal-title"
            aria-describedby="modal-description"
            ref={node_ref}
        >
            <button
                class="pf-c-button pf-m-plain"
                type="button"
                aria-label="Close dialog"
                onclick={onclose.reform(|_|())}
            >
                <i class="fas fa-times" aria-hidden="true"></i>
            </button>

            <header class="pf-c-modal-box__header">
                <h1
                    class="pf-c-modal-box__title"
                    id="modal-title-modal-with-form"
                >{ &props.title }</h1>
            </header>


            if !&props.description.is_empty() {
                <div class="pf-c-modal-box__body">
                    <p>{ &props.description }</p>
                </div>
            }

            { for props.children.iter().map(|c|{
               { html! (
                <div class="pf-c-modal-box__body">{c}</div>
               ) }
            }) }

            if let Some(footer) = &props.footer {
              <footer class="pf-c-modal-box__footer">
                  { footer.clone() }
              </footer>
            }
        </div>
    )
}
