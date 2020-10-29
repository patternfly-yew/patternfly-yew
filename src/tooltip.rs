use crate::Popper;
use crate::{Orientation, PopperContent};

use yew::prelude::*;

use crate::integration::popperjs;

// tooltip

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TooltipProps {
    pub children: Children,
    pub text: String,
    #[prop_or_default]
    pub active: bool,
}

pub struct Tooltip {
    props: TooltipProps,
    link: ComponentLink<Self>,
    node: NodeRef,
}

#[derive(Clone, Debug)]
pub enum TooltipMsg {
    Enter,
    Leave,
}

impl Component for Tooltip {
    type Message = TooltipMsg;
    type Properties = TooltipProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            node: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::debug!("Update: {:?}", msg);

        match msg {
            TooltipMsg::Enter => {
                self.props.active = true;
                true
            }
            TooltipMsg::Leave => {
                self.props.active = false;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let enter = self.link.callback(|_| TooltipMsg::Enter);
        let leave = self.link.callback(|_| TooltipMsg::Leave);

        return html! {
            <>
                <Popper<Tooltip> active=self.props.active content=self.props.clone()>
                    <span onmouseenter=enter.clone() onmouseleave=leave.clone() ref=self.node.clone()>
                        { for self.props.children.iter() }
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

        html! {<TooltipPopup ref=r#ref styles=styles hidden=state.is_none() orientation=orientation text=&props.text/>}
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

#[derive(Clone, PartialEq)]
pub struct TooltipPopup {
    props: TooltipPopupProps,
}

impl Component for TooltipPopup {
    type Message = ();
    type Properties = TooltipPopupProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("pf-c-tooltip");

        classes = classes.extend(self.props.orientation.as_classes());

        let style = if self.props.hidden {
            "display: none;"
        } else {
            &self.props.styles
        };

        return html! {
            <div style=style class=classes role="tooltip">
                <div class="pf-c-tooltip__arrow"></div>
                <div class="pf-c-tooltip__content">
                    { &self.props.text }
                </div>
            </div>
        };
    }
}
