use crate::{utils::ContextWrapper, Backdropper, Button, ButtonVariant, Icon};
use std::ops::Deref;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub brand_src: String,
    #[prop_or_default]
    pub brand_alt: String,
    pub title: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub strapline: Html,
    #[prop_or_default]
    pub logo: String,
    #[prop_or_default]
    pub onclose: Option<Callback<()>>,
    #[prop_or_default]
    pub hero_style: Option<String>,
}

pub enum Msg {
    Close,
    SetBackdrop(Backdropper),
}

pub struct About {
    backdrop: ContextWrapper<Backdropper>,
}

impl Component for About {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let backdrop = ContextWrapper::from((ctx, Msg::SetBackdrop));
        Self { backdrop }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Close => {
                if let Some(onclose) = &ctx.props().onclose {
                    onclose.emit(());
                } else {
                    if let Some(backdrop) = self.backdrop.deref() {
                        backdrop.close();
                    }
                }
            }
            Msg::SetBackdrop(backdropper) => {
                self.backdrop.set(backdropper);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let hero_style = match (ctx.props().hero_style.as_ref(), ctx.props().logo.is_empty()) {
            (Some(hero_style), _) => hero_style.into(),
            (None, false) => format!(
                "--pf-c-about-modal-box__hero--sm--BackgroundImage:url({url});",
                url = ctx.props().logo
            ),
            (None, true) => "".into(),
        };

        html!(
            <div class="pf-c-about-modal-box" role="dialog" aria-modal="true" aria-labelledby="about-modal-title">
                { if !ctx.props().brand_src.is_empty() {html!{
                    <div class="pf-c-about-modal-box__brand">
                        <img
                          class="pf-c-about-modal-box__brand-image"
                          src={ctx.props().brand_src.clone()}
                          alt={ctx.props().brand_alt.clone()}
                        />
                    </div>
                }} else {html!{}}}

                <div class="pf-c-about-modal-box__close">
                  <Button
                    variant={ButtonVariant::Plain}
                    aria_label="Close dialog"
                    onclick={ctx.link().callback(|_|Msg::Close)}>
                    { Icon::Times }
                  </Button>
                </div>

                <div class="pf-c-about-modal-box__header">
                  <h1 class="pf-c-title pf-m-4xl" id="about-modal-title">{ &ctx.props().title }</h1>
                </div>
                <div class="pf-c-about-modal-box__hero" style={hero_style}></div>

              <div class="pf-c-about-modal-box__content">
                { for ctx.props().children.iter() }
                <p
                  class="pf-c-about-modal-box__strapline"
                >{ctx.props().strapline.clone()}</p>
              </div>

            </div>
        )
    }
}
