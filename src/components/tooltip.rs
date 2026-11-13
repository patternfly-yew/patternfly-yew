//! Tooltip
use crate::prelude::{ExtendClasses, Orientation};
use popper_rs::{prelude::*, yew::component::PortalPopper};
use yew::prelude::*;

pub use popper_rs::prelude::Placement as TooltipPosition;

/// Properties for [`Tooltip`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TooltipProperties {
    pub children: Html,
    #[prop_or_default]
    pub text: String,
    /// Rich content, overriding `text` when supplied.
    #[prop_or_default]
    pub content: Option<Html>,
    #[prop_or(Placement::Right)]
    pub position: TooltipPosition,
    /// Attach directly to a native child element, preserving its layout.
    #[prop_or_default]
    pub attach_to_child: bool,
}

/// Tooltip component
///
/// > A **tooltip** is in-app messaging used to identify elements on a page with short, clarifying text.
///
/// See: <https://www.patternfly.org/components/tooltip>
///
/// ## Properties
///
/// Defined by [`TooltipProperties`].
#[function_component(Tooltip)]
pub fn tooltip(props: &TooltipProperties) -> Html {
    let hovered = use_state_eq(|| false);
    let focused = use_state_eq(|| false);
    let state = use_state_eq(State::default);

    let onmouseenter = use_callback(hovered.clone(), |_, hovered| hovered.set(true));
    let onmouseleave = use_callback(hovered.clone(), |_, hovered| hovered.set(false));

    let onfocusin = use_callback(focused.clone(), |_, focused| focused.set(true));
    let onfocusout = use_callback(focused.clone(), |_, focused| focused.set(false));
    let onkeydown = use_callback(
        (hovered.clone(), focused.clone()),
        |event: KeyboardEvent, (hovered, focused)| {
            if event.key() == "Escape" {
                hovered.set(false);
                focused.set(false);
            }
        },
    );
    let content_ref = use_node_ref();
    let target_ref = use_node_ref();

    let tooltip_id = crate::hooks::id::use_id(None);

    let onstatechange = use_callback(state.clone(), |new_state, state| state.set(new_state));

    let mut trigger = props.children.clone();
    if props.attach_to_child
        && let Html::VTag(tag) = &mut trigger
    {
        let tag = std::rc::Rc::make_mut(tag);
        tag.node_ref = target_ref.clone();
        tag.add_listener(std::rc::Rc::new(yew::html::onmouseenter::Wrapper::new(
            onmouseenter,
        )));
        tag.add_listener(std::rc::Rc::new(yew::html::onmouseleave::Wrapper::new(
            onmouseleave,
        )));
        tag.add_listener(std::rc::Rc::new(yew::html::onfocusin::Wrapper::new(
            onfocusin,
        )));
        tag.add_listener(std::rc::Rc::new(yew::html::onfocusout::Wrapper::new(
            onfocusout,
        )));
        tag.add_listener(std::rc::Rc::new(yew::html::onkeydown::Wrapper::new(
            onkeydown,
        )));
        tag.add_attribute("aria-describedby", tooltip_id.clone());
    } else {
        trigger = html!(
            <span
                {onmouseenter}
                {onmouseleave}
                {onfocusin}
                {onfocusout}
                {onkeydown}
                ref={target_ref.clone()}
            >
                { trigger }
            </span>
        );
    }

    html! (
        <>
            { trigger }
            <PortalPopper
                visible={*hovered || *focused}
                content={content_ref.clone()}
                target={target_ref}
                {onstatechange}
                placement={props.position}
                modifiers={vec![
                    Modifier::Offset(Offset {
                        skidding: 0,
                        distance: 11,
                    }),
                    Modifier::PreventOverflow(PreventOverflow { padding: 0 }),
                ]}
            >
                <TooltipPopupContent
                    id={tooltip_id}
                    state={(*state).clone()}
                    text={props.content.clone().unwrap_or_else(|| props.text.clone().into())}
                    r#ref={content_ref}
                />
            </PortalPopper>
        </>
    )
}

#[derive(PartialEq, Properties)]
struct TooltipPopupContentProperties {
    id: AttrValue,
    text: Html,
    state: State,
    r#ref: NodeRef,
}

#[function_component(TooltipPopupContent)]
fn tooltip_popup_content(props: &TooltipPopupContentProperties) -> Html {
    let orientation = Orientation::from_popper_data(&props.state.attributes.popper);

    html! {
        <TooltipPopup
            id={props.id.clone()}
            r#ref={props.r#ref.clone()}
            style={&props.state.styles.popper.extend_with("z-index", "1000")}
            {orientation}
            content={props.text.clone()}
        />
    }
}

/// Properties for [`TooltipPopup`]
#[derive(Clone, PartialEq, Properties)]
pub struct TooltipPopupProperties {
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub text: String,
    /// Rich content, overriding `text` when supplied.
    #[prop_or_default]
    pub content: Option<Html>,
    pub orientation: Orientation,
    #[prop_or_default]
    pub style: AttrValue,
    #[prop_or_default]
    pub r#ref: NodeRef,
}

/// The content shown when the tooltip pops up.
///
/// ## Properties
///
/// Defined by [`TooltipPopupProperties`].
#[function_component(TooltipPopup)]
pub fn tooltip_popup(props: &TooltipPopupProperties) -> Html {
    let mut class = Classes::from("pf-v6-c-tooltip");

    class.extend_from(&props.orientation);

    html! {
        <div id={props.id.clone()} ref={&props.r#ref} style={&props.style} {class} role="tooltip">
            <div class="pf-v6-c-tooltip__arrow" />
            <div class="pf-v6-c-tooltip__content">
                { props.content.clone().unwrap_or_else(|| props.text.clone().into()) }
            </div>
        </div>
    }
}
