//! About modal

use crate::{utils::ContextWrapper, Button, ButtonVariant, Icon, ModalProperties, use_backdrop};
use yew::prelude::*;
use yew_hooks::{use_click_away, use_event_with_window};

/// Properties for [`About`]
#[derive(Clone, PartialEq, Properties)]
pub struct AboutProperties {
    /// Id of the outermost element
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub brand_src: AttrValue,
    #[prop_or_default]
    pub brand_alt: Option<AttrValue>,
    pub title: AttrValue,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub strapline: Option<Html>,
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

    /// Disable closing the modal when the escape key is pressed
    #[prop_or_default]
    pub disable_close_escape: bool,
    /// Disable closing the modal when the user clicks outside the modal
    #[prop_or_default]
    pub disable_close_click_outside: bool,

}

/// About modal component
///
/// > An **about modal** displays information about an application like product version number(s), as well as any appropriate legal text.
///
/// See: <https://www.patternfly.org/v4/components/about-modal>
///
/// For a complete example, see the PatternFly Yew quickstart.
///
/// ## Properties
///
/// Defined by [`AboutProperties`].
///
/// ## Contexts
///
/// If the modal dialog is wrapped by a [`crate::prelude::BackdropViewer`] component and no
/// `onclose` callback is set, then it will automatically close the backdrop when the modal dialog
/// gets closed.
///
#[function_component(About)]
pub fn about(props: &AboutProperties) -> Html {
    // TODO: Focus is not trapped this should be considdered when using backdrop
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
    {
        let disabled = props.disable_close_escape.clone();
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
        let disabled = props.disable_close_click_outside.clone();
        let onclose = onclose.clone();
        use_click_away(node_ref.clone(), move |_: Event| {
            if !disabled {
                onclose.emit(());
            }
        });
    }

    html!(
        <div
            id={props.id.clone()}
            {class}
            role="dialog"
            aria-modal="true"
            aria-labelledby="about-modal-title"
            ref={node_ref}
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
