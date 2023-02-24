use crate::{utils::ContextWrapper, Backdropper};
use std::ops::Deref;
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
}

pub enum Msg {
    Close,
    SetBackdrop(Backdropper),
}

/// The Modal component.
///
/// > A **modal** displays important information to a user without requiring them to navigate to a new page.
///
/// See: https://www.patternfly.org/v4/components/modal
///
/// ## Properties
///
/// Defined by [`ModalProperties`].
///
/// ## Contexts
///
/// If the modal dialog is wrapped by a [`BackdropViewer`] component and no `onclose` callback is
/// set, then it will automatically close the backdrop when the modal dialog gets closed.
///
pub struct Modal {
    backdrop: ContextWrapper<Backdropper>,
}

impl Component for Modal {
    type Message = Msg;
    type Properties = ModalProperties;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            backdrop: ContextWrapper::from((ctx, Msg::SetBackdrop)),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Close => {
                if let Some(onclose) = &ctx.props().onclose {
                    onclose.emit(());
                } else if let Some(backdrop) = self.backdrop.deref() {
                    backdrop.close();
                }
            }
            Msg::SetBackdrop(backdrop) => {
                self.backdrop.set(backdrop);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = ctx.props().variant.as_classes();
        classes.push("pf-c-modal-box");

        html! {
            <div class={classes}
                    role="dialog"
                    aria-modal="true"
                    aria-labelledby="modal-title"
                    aria-describedby="modal-description">
                <button
                    class="pf-c-button pf-m-plain"
                    type="button"
                    aria-label="Close dialog"
                    onclick={ctx.link().callback(|_|Msg::Close)}
                >
                    <i class="fas fa-times" aria-hidden="true"></i>
                </button>

                <header class="pf-c-modal-box__header">
                    <h1
                        class="pf-c-modal-box__title"
                        id="modal-title-modal-with-form"
                    >{ &ctx.props().title }</h1>
                </header>


                if !&ctx.props().description.is_empty() {
                    <div class="pf-c-modal-box__body">
                        <p>{ &ctx.props().description }</p>
                    </div>
                }

                { for ctx.props().children.iter().map(|c|{
                   {html!{
                    <div class="pf-c-modal-box__body">{c}</div>
                   }}
                }) }

                if let Some(footer) = &ctx.props().footer {
                  <footer class="pf-c-modal-box__footer">
                      { footer.clone() }
                  </footer>
                }
            </div>
        }
    }
}
