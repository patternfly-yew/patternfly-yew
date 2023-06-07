//! Popover
use crate::prelude::{Button, ButtonVariant, Icon, Orientation, Popper, PopperContent};
use yew::prelude::*;
use yew::virtual_dom::VChild;

use crate::integration::popperjs;

// tooltip

/// Properties for [`Popover`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PopoverProperties {
    /// The target, rendered by the component, to which the popover will be aligned to.
    #[prop_or_default]
    pub target: Children,

    /// The body content of the popover.
    pub body: VChild<PopoverBody>,

    /// Binds the onclick handler of the target to toggle visibility.
    #[prop_or_default]
    pub toggle_by_onclick: bool,
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
pub struct Popover {
    node: NodeRef,
    active: bool,
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub enum PopoverMsg {
    Open,
    Close,
}

impl Component for Popover {
    type Message = PopoverMsg;
    type Properties = PopoverProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node: NodeRef::default(),
            active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PopoverMsg::Open => {
                if !self.active {
                    self.active = true;
                    true
                } else {
                    false
                }
            }
            PopoverMsg::Close => {
                if self.active {
                    self.active = false;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (onclick, onclose) = match ctx.props().toggle_by_onclick {
            true => (
                ctx.link().callback(|_| PopoverMsg::Open),
                ctx.link().callback(|_| PopoverMsg::Close),
            ),
            false => Default::default(),
        };

        let style = match self.active {
            true => "pointer-events: none;",
            false => "",
        };

        html! (
            <>
                <Popper<Popover>
                    active={self.active}
                    content={ctx.props().clone()}
                    onclose={onclose}
                    >
                    <span style={style} onclick={onclick} ref={self.node.clone()}>
                        { ctx.props().target.clone() }
                    </span>
                </Popper<Popover>>
            </>
        )
    }
}

impl PopperContent for Popover {
    fn view(
        props: &PopoverProperties,
        onclose: Callback<()>,
        r#ref: NodeRef,
        state: Option<popperjs::State>,
    ) -> Html {
        let styles = match &state {
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
                styles={styles}
                orientation={orientation}
                onclose={onclose}
                body={props.body.clone()}
            />
        )
    }
}

// popover popup

#[derive(Clone, PartialEq, Properties)]
pub struct PopoverPopupProperties {
    pub body: VChild<PopoverBody>,

    pub orientation: Orientation,
    #[prop_or_default]
    pub hidden: bool,
    #[prop_or_default]
    pub styles: String,

    /// called when the close button is clicked
    #[prop_or_default]
    pub onclose: Callback<()>,

    #[prop_or_default]
    pub r#ref: NodeRef,
}

/// The actual popover content component.
#[function_component(PopoverPopup)]
pub fn popover_popup(props: &PopoverPopupProperties) -> Html {
    let mut classes = classes!("pf-v5-c-popover");

    classes.extend(props.orientation.as_classes());

    let style = if props.hidden {
        "display: none;".to_string()
    } else {
        props.styles.to_string()
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
            style={style}
            class={classes}
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
