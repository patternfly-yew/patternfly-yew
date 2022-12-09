use crate::{Button, Icon, Orientation, Popper, PopperContent, Variant};

use yew::prelude::*;

use crate::integration::popperjs;

// tooltip

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PopoverProps {
    /// The target, rendered by the component, to which the popover will be aligned to.
    #[prop_or_default]
    pub target: Html,

    /// The header content of the popover.
    #[prop_or_default]
    pub header: Option<Html>,
    /// The content which will be show in the popover.
    pub children: Children,
    /// The footer content of the popover.
    #[prop_or_default]
    pub footer: Option<Html>,

    /// Binds the onclick handler of the target to toggle visibility.
    #[prop_or_default]
    pub toggle_by_onclick: bool,
}

pub struct Popover {
    node: NodeRef,
    active: bool,
}

#[derive(Clone, Debug)]
pub enum PopoverMsg {
    Open,
    Close,
}

impl Component for Popover {
    type Message = PopoverMsg;
    type Properties = PopoverProps;

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
        props: &PopoverProps,
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
                header={props.header.clone()}
                footer={props.footer.clone()}
                children={props.children.clone()}
                onclose={onclose}
            />
        )
    }
}

// popover popup

#[derive(Clone, PartialEq, Properties)]
pub struct PopoverPopupProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub header: Option<Html>,
    #[prop_or_default]
    pub footer: Option<Html>,
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

#[derive(Clone)]
pub struct PopoverPopup {}

#[derive(Copy, Debug, Clone)]
pub enum PopoverPopupMsg {
    Close,
}

impl Component for PopoverPopup {
    type Message = PopoverPopupMsg;
    type Properties = PopoverPopupProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PopoverPopupMsg::Close => {
                ctx.props().onclose.emit(());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-c-popover");

        classes.extend(ctx.props().orientation.as_classes());

        let style = if ctx.props().hidden {
            "display: none;".to_string()
        } else {
            ctx.props().styles.to_string()
        };

        let onclose = ctx.link().callback(|_| PopoverPopupMsg::Close);

        return html! {
            <div
                ref={&ctx.props().r#ref}
                style={style}
                class={classes}
                role="dialog"
                aria-model="true"
            >
                <div class="pf-c-popover__arrow"></div>
                <div class="pf-c-popover__content">

                    <Button
                        variant={Variant::Plain}
                        icon={Icon::Times}
                        aria_label="Close"
                        onclick={onclose}
                    />

                    if let Some(header) = &ctx.props().header {
                        <h1 class="pf-c-title pf-m-md">
                            { header.clone() }
                        </h1>
                    }

                    <div class="pf-c-popover__body">
                        { for ctx.props().children.iter() }
                    </div>

                    if let Some(footer) = &ctx.props().footer {
                        <footer class="pf-c-popover__footer">
                            { footer.clone() }
                        </footer>
                    }

                </div>
            </div>
        };
    }
}
