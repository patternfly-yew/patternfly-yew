use crate::{integration::popperjs::Options, prelude::*, utils::popper::*};
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
    pub variant: MenuToggleVariant,

    #[prop_or_default]
    pub position: Position,
}

#[function_component(Dropdown)]
pub fn drop_down(props: &DropdownProperties) -> Html {
    let expanded = use_state_eq(|| false);
    let ontoggle = {
        let expanded = expanded.clone();
        Callback::from(move |()| {
            expanded.set(!*expanded);
        })
    };

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

    let popper = use_state_eq(|| Option::<PopperState>::None);

    let onstatechange = {
        let popper = popper.clone();
        use_memo(
            move |()| {
                let popper = popper.clone();
                Callback::from(move |state| {
                    popper.set(Some(state));
                })
            },
            (),
        )
    };

    let content = html!(
        <Menu
            r#ref={menu_ref.clone()}
            style={popper.as_ref().map(|state|state.styles.clone()).unwrap_or_default()}
        >
            { for props.children.iter() }
        </Menu>
    );

    html!(
        <>
            <div ref={inside_ref}>
                <Popper
                    target_ref={target_ref.clone()}
                    content_ref={menu_ref}
                    {content}
                    mode={PopperMode::Inline}
                    visible={*expanded}
                    onstatechange={(*onstatechange).clone()}
                    options={
                        Options {
                            placement: match props.position {
                                Position::Left => PopperPlacement::BottomStart,
                                Position::Right => PopperPlacement::BottomEnd,
                                Position::Top => PopperPlacement::TopStart,
                            },
                            strategy: PopperStrategy::Absolute,
                            ..Default::default()
                        }
                    }
                >
                </Popper>
                <MenuToggle
                    r#ref={target_ref}
                    text={props.text.clone()}
                    icon={props.icon.clone()}
                    disabled={props.disabled}
                    full_height={props.full_height}
                    aria_label={&props.aria_label}
                    variant={props.variant}
                    expanded={*expanded}
                    {ontoggle}
                />
            </div>
        </>
    )
}
