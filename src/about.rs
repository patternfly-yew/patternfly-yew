use crate::{BackdropDispatcher, Icon, button::{Button, Variant}};
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
}

#[derive(Clone)]
pub struct About {}

impl Component for About {
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
        let hero_style = match (ctx.props().hero_style.as_ref(), ctx.props().logo.is_empty()) {
            (Some(hero_style), _) => hero_style.into(),
            (None, false) => format!(
                "--pf-c-about-modal-box__hero--sm--BackgroundImage:url({url});",
                url = ctx.props().logo
            ),
            (None, true) => "".into(),
        };

        return html! {
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
                    variant={Variant::Plain}
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
        };
    }
}
