use crate::{BackdropDispatcher};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub description: String,
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
pub struct Modal {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for Modal {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Close => {
                if let Some(onclose) = &self.props.onclose {
                    onclose.emit(());
                } else {
                    BackdropDispatcher::default().close();
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {

        return html! {
            <div class="pf-c-modal-box" role="dialog" aria-modal="true" aria-labelledby="modal-title" aria-describedby="modal-description">
                <button
                    class="pf-c-button pf-m-plain"
                    type="button"
                    aria-label="Close dialog"
                    onclick=self.link.callback(|_|Msg::Close)
                >
                    <i class="fas fa-times" aria-hidden="true"></i>
                </button>

                <header class="pf-c-modal-box__header">
                    <h1
                        class="pf-c-modal-box__title"
                        id="modal-title-modal-with-form"
                    >{ &self.props.title }</h1>
                </header>

                <div class="pf-c-modal-box__body">
                    <p>{ &self.props.description }</p>
                </div>

                { for self.props.children.iter().map(|c|{
                   {html!{
                    <div class="pf-c-modal-box__body">{c}</div>
                       }}
                }) }

                { if let Some(footer) = &self.props.footer {
                  {html!{
                      <footer class="pf-c-modal-box__footer">
                      { footer.clone() }
                      </footer>
                  }}
                } else {
                    html!{}
                }}
            </div>
        };
    }
}
