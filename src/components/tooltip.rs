//! Tooltip
use crate::prelude::{ExtendClasses, Orientation};
use popper_rs::{prelude::*, yew::component::PortalPopper};
use yew::prelude::*;

/// Properties for [`Tooltip`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TooltipProperties {
    pub children: Html,
    pub text: String,
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
    let active = use_state_eq(|| false);
    let state = use_state_eq(State::default);

    let onmouseenter = use_callback(active.clone(), |_, active| active.set(true));
    let onmouseleave = use_callback(active.clone(), |_, active| active.set(false));

    let content_ref = use_node_ref();
    let target_ref = use_node_ref();

    let onstatechange = use_callback(state.clone(), |new_state, state| state.set(new_state));

    html! (
        <>
            <span {onmouseenter} {onmouseleave} ref={target_ref.clone()}>
                { props.children.clone() }
            </span>
            <PortalPopper
                visible={*active}
                content={content_ref.clone()}
                target={target_ref}
                {onstatechange}
                placement={Placement::Right}
                modifiers={vec![
                    Modifier::Offset(Offset {
                        skidding: 0,
                        distance: 11,
                    }),
                    Modifier::PreventOverflow(PreventOverflow { padding: 0 }),
                ]}
            >
                <TooltipPopupContent
                    state={(*state).clone()}
                    text={props.text.clone()}
                    r#ref={content_ref}
                />
            </PortalPopper>
        </>
    )
}

#[derive(PartialEq, Properties)]
struct TooltipPopupContentProperties {
    text: String,
    state: State,
    r#ref: NodeRef,
}

#[function_component(TooltipPopupContent)]
fn tooltip_popup_content(props: &TooltipPopupContentProperties) -> Html {
    let orientation = Orientation::from_popper_data(&props.state.attributes.popper);

    html! {
        <TooltipPopup
            r#ref={props.r#ref.clone()}
            style={&props.state.styles.popper.extend_with("z-index", "1000")}
            {orientation}
            text={props.text.clone()}
        />
    }
}

/// Properties for [`TooltipPopup`]
#[derive(Clone, PartialEq, Properties)]
pub struct TooltipPopupProperties {
    pub text: String,
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
        <div ref={&props.r#ref} style={&props.style} {class} role="tooltip">
            <div class="pf-v6-c-tooltip__arrow"></div>
            <div class="pf-v6-c-tooltip__content">
                { &props.text }
            </div>
        </div>
    }
}
