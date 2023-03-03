//! About modal

use crate::{utils::ContextWrapper, Backdropper, Button, ButtonVariant, Icon};
use std::ops::Deref;
use yew::prelude::*;

/// Properties for [`About`]
#[derive(Clone, PartialEq, Properties)]
pub struct AboutProperties {
    /// Id of the outermost element
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub brand_src: AttrValue,
    #[prop_or_default]
    pub brand_alt: AttrValue,
    pub title: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub strapline: Html,
    /// A simple way to style the hero section with a logo.
    ///
    /// Will be ignored if `hero_style` is being used.
    #[prop_or_default]
    pub logo: AttrValue,
    #[prop_or_default]
    pub onclose: Option<Callback<()>>,
    /// Allows to directly set the style of the hero element.
    #[prop_or_default]
    pub hero_style: Option<String>,
}

#[doc(hidden)]
pub enum Msg {
    Close,
    SetBackdrop(Backdropper),
}

/// About modal component
///
/// > An **about modal** displays information about an application like product version number(s), as well as any appropriate legal text.
///
/// See: <https://www.patternfly.org/v4/components/about-modal>
///
/// The about modal just renders the dialog and can interact with a wrapping
/// [`Backdropper`](crate::prelude::Backdropper) context. It is intended to be spawned using a
/// [`Backdropper`]:
///
/// For a complete example, see the PatternFly Yew quickstart.
///
/// ## Properties
///
/// Defined by [`AboutProperties`].
///
///
pub struct About {
    backdrop: ContextWrapper<Backdropper>,
}

impl Component for About {
    type Message = Msg;
    type Properties = AboutProperties;

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
            <div
                id={&ctx.props().id}
                class="pf-c-about-modal-box" role="dialog" aria-modal="true" aria-labelledby="about-modal-title"
            >
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
