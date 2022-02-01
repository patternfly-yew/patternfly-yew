use crate::BackdropDispatcher;
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
pub struct Props {
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
}

#[derive(Clone)]
pub struct Modal {}

impl Component for Modal {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Close => {
                if let Some(onclose) = &ctx.props().onclose {
                    onclose.emit(());
                } else {
                    BackdropDispatcher::default().close();
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = ctx.props().variant.as_classes();
        classes.push("pf-c-modal-box");

        return html! {
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
        };
    }
}
