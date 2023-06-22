//! Popover
use crate::{
    prelude::{Button, ButtonVariant, ExtendClasses, Icon, Orientation},
    utils::popper::*,
};
use yew::{prelude::*, virtual_dom::VChild};
use yew_hooks::use_click_away;

// tooltip

/// Properties for [`Popover`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PopoverProperties {
    /// The target, rendered by the component, to which the popover will be aligned to.
    #[prop_or_default]
    pub target: Children,

    /// The body content of the popover.
    pub body: VChild<PopoverBody>,
}

/// Popover component
///
/// > A **popover** is in-app messaging that provides more information on specific product areas. Popovers display content in a new window that overlays the current page. Unlike modals, popovers don't block the current page.
///
/// See: <https://www.patternfly.org/v4/components/popover>
///
/// ## Properties
///
/// Defined by [`PopoverProperties`].
#[function_component(Popover)]
pub fn popover(props: &PopoverProperties) -> Html {
    let active = use_state_eq(|| false);
    let state = use_state_eq(|| Option::<PopperState>::None);

    // a reference to the target the user clicks on
    let target_ref = use_node_ref();
    // a reference to the content
    let content_ref = use_node_ref();

    let onclick = {
        let active = active.clone();
        Callback::from(move |_| {
            active.set(!*active);
        })
    };
    let onclose = {
        let active = active.clone();
        Callback::from(move |_| {
            active.set(false);
        })
    };

    {
        let active = active.clone();
        use_click_away(content_ref.clone(), move |_| {
            active.set(false);
        });
    }

    let content = use_memo(
        |(r#ref, state, body)| {
            let style = match &state {
                Some(state) => &state.styles,
                None => "display: none;",
            }
            .to_string();

            let orientation = state
                .as_ref()
                .map(|s| s.orientation)
                .unwrap_or(Orientation::Bottom);

            html! (
                <PopoverPopup
                    r#ref={r#ref}
                    {style}
                    {orientation}
                    {onclose}
                    body={(*body).clone()}
                />
            )
        },
        (content_ref.clone(), (*state).clone(), props.body.clone()),
    );

    let onstatechange = {
        let state = state.clone();
        use_memo(
            move |()| {
                let state = state.clone();
                Callback::from(move |new_state| {
                    state.set(Some(new_state));
                })
            },
            (),
        )
    };

    let options = PopperOptions {
        strategy: PopperStrategy::Fixed,
        placement: PopperPlacement::Right,
        modifiers: vec![
            Modifier::Offset(Offset {
                skidding: 0,
                distance: 11,
            }),
            Modifier::PreventOverflow(PreventOverflow { padding: 0 }),
        ],
    };

    let style = match *active {
        true => "pointer-events: none;",
        false => "",
    };

    html!(
        <>
            <span
                {onclick}
                {style}
                ref={target_ref.clone()}
            >
                { props.target.clone() }
            </span>
            <Popper
                visible={*active}
                content={(*content).clone()}
                {content_ref}
                {target_ref}
                mode={PopperMode::Portal}
                onstatechange={(*onstatechange).clone()}
                {options}
            />
        </>
    )
}

// popover popup

/// The popover content component.
#[derive(Clone, PartialEq, Properties)]
pub struct PopoverPopupProperties {
    pub body: VChild<PopoverBody>,

    pub orientation: Orientation,
    #[prop_or_default]
    pub hidden: bool,
    #[prop_or_default]
    pub style: AttrValue,

    /// called when the close button is clicked
    #[prop_or_default]
    pub onclose: Callback<()>,

    #[prop_or_default]
    pub r#ref: NodeRef,
}

/// The actual popover content component.
#[function_component(PopoverPopup)]
pub fn popover_popup(props: &PopoverPopupProperties) -> Html {
    let mut class = classes!("pf-v5-c-popover");

    class.extend_from(&props.orientation);

    let style = if props.hidden {
        "display: none;".to_string()
    } else {
        props.style.to_string()
    };

    let onclose = {
        let onclose = props.onclose.clone();
        Callback::from(move |_| {
            onclose.emit(());
        })
    };

    html! (
        <div
            ref={&props.r#ref}
            {style}
            {class}
            role="dialog"
            aria-model="true"
        >
            <div class="pf-v5-c-popover__arrow"></div>
            <div class="pf-v5-c-popover__content">
                <div class="pf-v5-c-popover__close">
                    <Button
                        variant={ButtonVariant::Plain}
                        icon={Icon::Times}
                        aria_label="Close"
                        onclick={onclose}
                    />
                </div>

                { props.body.clone() }

            </div>
        </div>
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PopoverBodyProperties {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub header: Children,
    #[prop_or_default]
    pub footer: Children,
}

#[function_component(PopoverBody)]
pub fn popover_body(props: &PopoverBodyProperties) -> Html {
    html!(
        <>
            if !props.header.is_empty() {
                <header class="pf-v5-c-popover__header">
                    <div class="pf-v5-c-popover__title">
                        <h1 class="pf-v5-c-title pf-m-md">
                            { for props.header.iter() }
                        </h1>
                    </div>
                </header>
            }

            <div class="pf-v5-c-popover__body">
                { for props.children.iter() }
            </div>

            if !props.footer.is_empty() {
                <footer class="pf-v5-c-popover__footer">
                    { for props.footer.iter() }
                </footer>
            }
        </>
    )
}
