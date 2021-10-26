use crate::{BackdropDispatcher, Button, Icon, Variant};
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
pub struct About {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for About {
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
        let hero_style = match (self.props.hero_style.as_ref(), self.props.logo.is_empty()) {
            (Some(hero_style), _) => hero_style.into(),
            (None, false) => format!(
                "--pf-c-about-modal-box__hero--sm--BackgroundImage:url({url});",
                url = self.props.logo
            ),
            (None, true) => "".into(),
        };

        return html! {
            <div class="pf-c-about-modal-box" role="dialog" aria-modal="true" aria-labelledby="about-modal-title">
                { if !self.props.brand_src.is_empty() {html!{
                    <div class="pf-c-about-modal-box__brand">
                        <img
                          class="pf-c-about-modal-box__brand-image"
                          src=self.props.brand_src
                          alt=self.props.brand_alt
                        />
                    </div>
                }} else {html!{}}}

                <div class="pf-c-about-modal-box__close">
                  <Button
                    variant=Variant::Plain
                    aria_label="Close dialog"
                    onclick=self.link.callback(|_|Msg::Close)>
                    { Icon::Times }
                  </Button>
                </div>

                <div class="pf-c-about-modal-box__header">
                  <h1 class="pf-c-title pf-m-4xl" id="about-modal-title">{ &self.props.title }</h1>
                </div>
                <div class="pf-c-about-modal-box__hero" style=hero_style></div>

              <div class="pf-c-about-modal-box__content">
                { for self.props.children.iter() }
                <p
                  class="pf-c-about-modal-box__strapline"
                >{self.props.strapline.clone()}</p>
              </div>

            </div>
        };
    }
}
