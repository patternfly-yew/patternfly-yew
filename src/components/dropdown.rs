use crate::prelude::*;
use popper_rs::prelude::{State as PopperState, *};
use yew::{html::ChildrenRenderer, prelude::*};
use yew_hooks::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct DropdownProperties {
    #[prop_or_default]
    pub children: ChildrenRenderer<MenuChildVariant>,

    #[prop_or_default]
    pub text: Option<String>,
    #[prop_or_default]
    pub icon: Option<Html>,

    #[prop_or_default]
    pub aria_label: AttrValue,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub full_height: bool,

    #[prop_or_default]
    pub full_width: bool,

    #[prop_or_default]
    pub variant: MenuToggleVariant,

    #[prop_or_default]
    pub position: Position,
}

/// Dropdown menu component
///
/// ## Properties
///
/// Define by [`DropdownProperties`].
///
/// ## Contexts
///
/// Provides the following contexts to its children:
///
/// * [`CloseMenuContext`]
#[function_component(Dropdown)]
pub fn drop_down(props: &DropdownProperties) -> Html {
    let expanded = use_state_eq(|| false);
    let ontoggle = use_callback(expanded.clone(), move |_, expanded| {
        expanded.set(!**expanded)
    });

    // this defines what is "inside"
    let inside_ref = use_node_ref();
    let target_ref = use_node_ref();
    let menu_ref = use_node_ref();

    {
        // click away unless it was on the inside, which covers the toggle as well as
        // the menu content. As long as we use inline/absolute popover modes and not use
        // a portal.
        let expanded = expanded.clone();
        use_click_away(inside_ref.clone(), move |_: Event| {
            expanded.set(false);
        });
    }

    let state = use_state_eq(PopperState::default);
    let onstatechange = use_callback(state.clone(), |new_state, state| state.set(new_state));

    let placement = match props.position {
        Position::Left => Placement::BottomStart,
        Position::Right => Placement::BottomEnd,
        Position::Top => Placement::TopStart,
    };

    let onclose = use_callback(expanded.clone(), |(), expanded| expanded.set(false));
    let context = CloseMenuContext::new(onclose);

    html!(
        <>
            <div style="display: inline;" ref={inside_ref}>
                <InlinePopper
                    target={target_ref.clone()}
                    content={menu_ref.clone()}
                    visible={*expanded}
                    {onstatechange}
                    {placement}
                >
                    <ContextProvider<CloseMenuContext>
                        {context}
                    >
                        <Menu
                            r#ref={menu_ref}
                            style={&state.styles.popper.extend_with("z-index", "1000")}
                        >
                            { props.children.clone() }
                        </Menu>
                    </ContextProvider<CloseMenuContext>>
                </InlinePopper>
                <MenuToggle
                    r#ref={target_ref}
                    text={props.text.clone()}
                    icon={props.icon.clone()}
                    disabled={props.disabled}
                    full_height={props.full_height}
                    full_width={props.full_width}
                    aria_label={&props.aria_label}
                    variant={props.variant}
                    expanded={*expanded}
                    {ontoggle}
                />
            </div>
        </>
    )
}
