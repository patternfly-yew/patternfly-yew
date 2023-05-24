//! About modal

use crate::{utils::ContextWrapper, Backdropper, Button, ButtonVariant, Icon, ModalProperties, use_backdrop};
use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::Node;
use yew::prelude::*;

/// Properties for [`About`]
#[derive(Clone, PartialEq, Properties)]
pub struct AboutProperties {
    /// Id of the outermost element
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes, // TODO: Should this be Option<T>
    #[prop_or_default]
    pub brand_src: AttrValue, // TODO: Optonial?
    #[prop_or_default]
    pub brand_alt: Option<AttrValue>,
    pub title: AttrValue,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub strapline: Option<Html>, // FIXME: Change vnode in hint actions to html?
        /// A simple way to style the hero section with a logo.
        ///
        /// Will be ignored if `hero_style` is being used.
    #[prop_or_default]
    pub logo: AttrValue,  // FIXME: this is called backgroundImageSrc in pf react. Would seem a better name.
    #[prop_or_default]
    pub onclose: Option<Callback<()>>,
    /// Allows to directly set the style of the hero element.
    #[prop_or_default]
    pub hero_style: Option<String>, // FIXME: pf does not appear to allow custom stying of the hero

    /// Disable closing the modal when the escape key is pressed
    #[prop_or_default]
    pub disable_close_escape: bool,
    /// Disable closing the modal when the user clicks outside the modal
    #[prop_or_default]
    pub disable_close_click_outside: bool,
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
#[function_component(About)]  // FIXME: pf Calls this compont `AboutModal`
pub fn about(props: &AboutProperties) -> Html {
    let mut class = props.class.clone();
    class.push("pf-c-about-modal-box");

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
        (props.onclose.clone(), backdrop.clone()),
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

    html!(
        <div
            id={props.id.clone()}
            {class}
            role="dialog"
            aria-modal="true"
            aria-labelledby="about-modal-title"
        >
            if !props.brand_src.is_empty() {
                <div class="pf-c-about-modal-box__brand">
                    <img
                      class="pf-c-about-modal-box__brand-image"
                      src={props.brand_src.clone()}
                      alt={props.brand_alt.clone()}
                    />
                </div>
            }

            <div class="pf-c-about-modal-box__close">
                <Button
                    variant={ButtonVariant::Plain}
                    aria_label="Close dialog"
                    onclick={onclose.reform(|_|())}
                >
                    { Icon::Times }
                </Button>
            </div>

            <div class="pf-c-about-modal-box__header">
                <h1 class="pf-c-title pf-m-4xl" id="about-modal-title">{ props.title.clone() }</h1>
            </div>

            <div class="pf-c-about-modal-box__hero" /> // style={hero_style}></div>

            <div class="pf-c-about-modal-box__content">
                { for props.children.iter() }
                if props.strapline.is_some() {
                    <p class="pf-c-about-modal-box__strapline">{ props.strapline.clone() }</p>
                }
            </div>
        </div>
    )
}

// pub struct About {
//     backdrop: ContextWrapper<Backdropper>,
// }
//
// impl Component for About {
//     type Message = Msg;
//     type Properties = AboutProperties;
//
//     fn create(ctx: &Context<Self>) -> Self {
//         let backdrop = ContextWrapper::from((ctx, Msg::SetBackdrop));
//         Self { backdrop }
//     }
//
//     fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             Msg::Close => {
//                 if let Some(onclose) = &ctx.props().onclose {
//                     onclose.emit(());
//                 } else {
//                     if let Some(backdrop) = self.backdrop.deref() {
//                         backdrop.close();
//                     }
//                 }
//             }
//             Msg::SetBackdrop(backdropper) => {
//                 self.backdrop.set(backdropper);
//             }
//         }
//         true
//     }

    // fn view(&self, ctx: &Context<Self>) -> Html {
    //     let hero_style = match (ctx.props().hero_style.as_ref(), ctx.props().logo.is_empty()) {
    //         (Some(hero_style), _) => hero_style.into(),
    //         (None, false) => format!(
    //             "--pf-c-about-modal-box__hero--sm--BackgroundImage:url({url});",
    //             url = ctx.props().logo
    //         ),
    //         (None, true) => "".into(),
    //     };
    // }
//}
