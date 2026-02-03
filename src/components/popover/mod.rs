//! Popover
use crate::prelude::{Button, ButtonVariant, ExtendClasses, Icon, Orientation};
use popper_rs::{
    prelude::{State as PopperState, *},
    yew::component::PortalPopper,
};
use yew::{prelude::*, virtual_dom::VChild};
use yew_hooks::use_click_away;

#[derive(Clone, PartialEq)]
pub struct PopoverContext {
    close: Callback<()>,
}

impl PopoverContext {
    /// Close the popover
    pub fn close(&self) {
        self.close.emit(());
    }
}

/// Properties for [`Popover`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PopoverProperties {
    /// The target, rendered by the component, to which the popover will be aligned to.
    #[prop_or_default]
    pub target: Html,

    /// The body content of the popover.
    pub body: VChild<PopoverBody>,

    #[prop_or_default]
    pub no_padding: bool,

    #[prop_or_default]
    pub no_close: bool,

    #[prop_or_default]
    pub width_auto: bool,
}

/// Popover component
///
/// > A **popover** is in-app messaging that provides more information on specific product areas. Popovers display content in a new window that overlays the current page. Unlike modals, popovers don't block the current page.
///
/// See: <https://www.patternfly.org/components/popover>
///
/// ## Properties
///
/// Defined by [`PopoverProperties`].
#[function_component(Popover)]
pub fn popover(props: &PopoverProperties) -> Html {
    let active = use_state_eq(|| false);

    let state = use_state_eq(PopperState::default);
    let onstatechange = use_callback(state.clone(), |new_state, state| state.set(new_state));

    // a reference to the target the user clicks on
    let target_ref = use_node_ref();
    // a reference to the content
    let content_ref = use_node_ref();

    let onclick = use_callback(active.clone(), |_, active| active.set(!**active));
    let onclose = use_callback(active.clone(), |_, active| active.set(false));

    {
        let active = active.clone();
        use_click_away(content_ref.clone(), move |_| {
            active.set(false);
        });
    }

    let style = match *active {
        true => "pointer-events: none;",
        false => "",
    };

    let orientation = Orientation::from_popper_data(&state.attributes.popper);

    let context = PopoverContext {
        close: onclose.clone(),
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
            <PortalPopper
                visible={*active}
                content={content_ref.clone()}
                target={target_ref}
                {onstatechange}
                placement={Placement::Right}
                modifiers={vec![
                    Modifier::Offset(Offset {
                        skidding: 0,
                        distance: 20,
                    }),
                    Modifier::PreventOverflow(PreventOverflow { padding: 0 }),
                ]}
            >
                <ContextProvider<PopoverContext> {context}>
                    <PopoverPopup
                        width_auto={props.width_auto}
                        no_padding={props.no_padding}
                        no_close={props.no_close}
                        r#ref={content_ref}
                        style={&state.styles.popper.extend_with("z-index", "1000")}
                        {orientation}
                        {onclose}
                        body={props.body.clone()}
                    />
                </ContextProvider<PopoverContext>>
            </PortalPopper>
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
    pub no_padding: bool,
    #[prop_or_default]
    pub no_close: bool,

    #[prop_or_default]
    pub width_auto: bool,

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
    let mut class = classes!("pf-v6-c-popover");

    class.extend_from(&props.orientation);

    if props.width_auto {
        class.extend(classes!("pf-m-width-auto"));
    }

    if props.no_padding {
        class.extend(classes!("pf-m-no-padding"));
    }

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
            <div class="pf-v6-c-popover__arrow"></div>
            <div class="pf-v6-c-popover__content">
                if !props.no_close {
                    <div class="pf-v6-c-popover__close">
                        <Button
                            variant={ButtonVariant::Plain}
                            icon={Icon::Times}
                            aria_label="Close"
                            onclick={onclose}
                        />
                    </div>
                }

                { props.body.clone() }

            </div>
        </div>
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PopoverBodyProperties {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub header: Option<Html>,
    #[prop_or_default]
    pub footer: Option<Html>,
}

#[function_component(PopoverBody)]
pub fn popover_body(props: &PopoverBodyProperties) -> Html {
    html!(
        <>
            if let Some(header) = &props.header {
                <header class="pf-v6-c-popover__header">
                    <div class="pf-v6-c-popover__title">
                        <h1 class="pf-v6-c-title pf-m-md">
                            { header.clone() }
                        </h1>
                    </div>
                </header>
            }

            <div class="pf-v6-c-popover__body">
                { props.children.clone() }
            </div>

            if let Some(footer) = &props.footer {
                <footer class="pf-v6-c-popover__footer">
                    { footer.clone() }
                </footer>
            }
        </>
    )
}
