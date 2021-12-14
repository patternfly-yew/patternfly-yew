use crate::Popper;
use crate::{Orientation, PopperContent};

use yew::prelude::*;

use crate::integration::popperjs;

// tooltip

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TooltipProps {
    pub children: Children,
    pub text: String,
}

pub struct Tooltip {
    node: NodeRef,
    active: bool,
}

#[derive(Clone, Debug)]
pub enum TooltipMsg {
    Enter,
    Leave,
}

impl Component for Tooltip {
    type Message = TooltipMsg;
    type Properties = TooltipProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            node: NodeRef::default(),
            active: false,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        log::debug!("Update: {:?}", msg);

        match msg {
            TooltipMsg::Enter => {
                self.active = true;
                true
            }
            TooltipMsg::Leave => {
                self.active = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let enter = ctx.link().callback(|_| TooltipMsg::Enter);
        let leave = ctx.link().callback(|_| TooltipMsg::Leave);

        return html! {
            <>
                <Popper<Tooltip> active={self.active} content={ctx.props().clone()}>
                    <span onmouseenter={enter.clone()} onmouseleave={leave.clone()} ref={self.node.clone()}>
                        { for ctx.props().children.iter() }
                    </span>
                </Popper<Tooltip>>
            </>
        };
    }
}

impl PopperContent for Tooltip {
    fn view(
        props: &TooltipProps,
        _onclose: Callback<()>,
        r#ref: NodeRef,
        state: Option<popperjs::State>,
    ) -> Html {
        let styles = state
            .as_ref()
            .map(|s| s.styles.to_string())
            .unwrap_or_default();
        let orientation = state
            .as_ref()
            .map(|s| s.orientation)
            .unwrap_or(Orientation::Bottom);

        html! {
        <TooltipPopup
            ref={r#ref}
            styles={styles}
            hidden={state.is_none()}
            orientation={orientation}
            text={props.text.clone()}
        />}
    }
}

// tooltip popup

#[derive(Clone, PartialEq, Properties)]
pub struct TooltipPopupProps {
    pub text: String,
    pub orientation: Orientation,
    #[prop_or_default]
    pub hidden: bool,
    #[prop_or_default]
    pub styles: String,
}

#[function_component(TooltipPopup)]
pub fn tooltip_popup(props: &TooltipPopupProps) -> Html {
    let mut classes = Classes::from("pf-c-tooltip");

    classes.extend(props.orientation.as_classes());

    let style = if props.hidden {
        "display: none;"
    } else {
        &props.styles
    }
    .to_string();

    html! {
        <div style={style} class={classes} role="tooltip">
            <div class="pf-c-tooltip__arrow"></div>
            <div class="pf-c-tooltip__content">
                { &props.text }
            </div>
        </div>
    }
}
