use serde_json::json;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

use yew::prelude::*;

const LOG_TARGET: &'static str = "tooltip";

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Orientation {
    Left,
    Right,
    Top,
    Bottom,
}

impl Orientation {
    pub fn as_classes(&self) -> Vec<&str> {
        match self {
            Orientation::Left => vec!["pf-m-left"],
            Orientation::Right => vec!["pf-m-right"],
            Orientation::Top => vec!["pf-m-top"],
            Orientation::Bottom => vec!["pf-m-bottom"],
        }
    }
}

// tooltip

#[derive(Clone, PartialEq, Properties)]
pub struct TooltipProps {
    pub children: Children,
    pub text: String,
}

pub struct Tooltip {
    props: TooltipProps,
    link: ComponentLink<Self>,
    node: NodeRef,
    tooltip: NodeRef,
    popper: Option<JsValue>,
    orientation: Orientation,
    styles: String,
}

#[derive(Clone, Debug)]
pub enum TooltipMsg {
    Enter,
    Leave,
    Position {
        orientation: Orientation,
        styles: String,
    },
}

impl Component for Tooltip {
    type Message = TooltipMsg;
    type Properties = TooltipProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            node: NodeRef::default(),
            tooltip: NodeRef::default(),
            popper: None,
            orientation: Orientation::Top,
            styles: Default::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TooltipMsg::Enter => {
                self.enter().unwrap();
                true
            }
            TooltipMsg::Leave => {
                self.leave().unwrap();
                true
            }
            TooltipMsg::Position {
                orientation,
                styles,
            } => {
                let mut changed = false;
                if self.orientation != orientation {
                    self.orientation = orientation;
                    changed = true;
                }
                if self.styles != styles {
                    self.styles = styles;
                    changed = true;
                }
                changed
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

        self.check_update().ok();

        return html! {
            <>
                <TooltipPopup hidden=self.popper.is_none() styles=&self.styles ref=self.tooltip.clone() orientation=self.orientation text=&self.props.text/>
                <span onmouseenter=enter.clone() onmouseleave=leave.clone() ref=self.node.clone()>
                    { for self.props.children.iter() }
                </span>
            </>
        };
    }
}

impl Tooltip {
    fn enter(&mut self) -> Result<(), JsValue> {
        if self.popper.is_some() {
            return Ok(());
        }
        let target = self.node.get().unwrap();
        let tooltip = self.tooltip.get().unwrap();

        let update = self.link.callback(|msg| msg);

        let apply = Closure::wrap(Box::new(move |this: &JsValue| {
            // web_sys::console::debug_2(&JsValue::from("apply: "), this);
            let msg = Self::from_popper(this).unwrap();
            // log::info!("Msg: {:?}", msg);
            update.emit(msg);
        }) as Box<dyn FnMut(&JsValue)>);

        let m1 = js_sys::Object::new();
        js_sys::Reflect::set(&m1, &JsValue::from("name"), &JsValue::from("applyStyles"))?;
        js_sys::Reflect::set(&m1, &JsValue::from("phase"), &JsValue::from("write"))?;
        js_sys::Reflect::set(&m1, &JsValue::from("fn"), &apply.into_js_value())?;

        let m2 = js_sys::Object::new();
        js_sys::Reflect::set(&m2, &JsValue::from("name"), &JsValue::from("offset"))?;
        js_sys::Reflect::set(
            &m2,
            &JsValue::from("options"),
            &JsValue::from_serde(&json!({
                "offset": [0, 11],
            }))
            .unwrap(),
        )?;

        let m3 = js_sys::Object::new();
        js_sys::Reflect::set(
            &m3,
            &JsValue::from("name"),
            &JsValue::from("preventOverflow"),
        )?;
        js_sys::Reflect::set(
            &m3,
            &JsValue::from("options"),
            &JsValue::from_serde(&json!({
                "padding": 0,
            }))
            .unwrap(),
        )?;

        let mods = js_sys::Array::of3(&m1, &m2, &m3);
        let opts = js_sys::Object::new();
        js_sys::Reflect::set(&opts, &JsValue::from("modifiers"), &mods)?;
        js_sys::Reflect::set(&opts, &JsValue::from("strategy"), &JsValue::from("fixed"))?;
        js_sys::Reflect::set(&opts, &JsValue::from("placement"), &JsValue::from("auto"))?;

        //web_sys::console::debug_1(&opts);

        let popper = crate::utils::createPopper(target, tooltip, &opts);

        // web_sys::console::debug_1(&popper);
        self.popper = Some(popper);

        Ok(())
    }

    fn leave(&mut self) -> Result<(), JsValue> {
        if let Some(popper) = self.popper.take() {
            // call popper.destroy()
            let destroy_fn = js_sys::Reflect::get(&popper, &JsValue::from("destroy"))?;
            let destroy_fn = destroy_fn.dyn_ref::<js_sys::Function>();
            if let Some(f) = destroy_fn {
                let r = f.call0(&popper);
                log::debug!(target: LOG_TARGET, "Destroyed: {:?}", r);
                r?;
            }
        }
        Ok(())
    }

    fn check_update(&self) -> Result<(), JsValue> {
        if let Some(popper) = &self.popper {
            let update_fn = js_sys::Reflect::get(&popper, &JsValue::from("update"))?;
            let update_fn = update_fn.dyn_ref::<js_sys::Function>();
            if let Some(f) = update_fn {
                let r = f.call0(&popper);
                log::debug!(target: LOG_TARGET, "Updated: {:?}", r);
                r?;
            }
        }
        Ok(())
    }

    fn from_popper(popper: &JsValue) -> Result<TooltipMsg, JsValue> {
        let state = js_sys::Reflect::get(popper, &JsValue::from("state"))?;
        let attributes = js_sys::Reflect::get(&state, &JsValue::from_str("attributes"))?;
        let popper = js_sys::Reflect::get(&attributes, &JsValue::from("popper"))?;
        let placement = js_sys::Reflect::get(&popper, &JsValue::from("data-popper-placement"))?;

        let orientation = match placement.as_string() {
            Some(p) if p == "bottom" => Orientation::Bottom,
            Some(p) if p == "top" => Orientation::Top,
            Some(p) if p == "left" => Orientation::Left,
            Some(p) if p == "right" => Orientation::Right,
            _ => Orientation::Bottom,
        };

        log::debug!(
            target: LOG_TARGET,
            "Orientation - original: {:?}, outcome: {:?}",
            placement.as_string(),
            orientation
        );

        let styles = js_sys::Reflect::get(&state, &JsValue::from_str("styles"))?;
        let popper = js_sys::Reflect::get(&styles, &JsValue::from("popper"))?;

        let popper = js_sys::Object::from(popper);

        let mut styles: String = js_sys::Object::entries(&popper)
            .to_vec()
            .iter()
            .map(|field| js_sys::Array::from(&field))
            .map(|field| {
                let key = js_sys::Array::get(&field, 0);
                let value = js_sys::Array::get(&field, 1);
                (
                    key.as_string().unwrap_or_default(),
                    value.as_string().unwrap_or_default(),
                )
            })
            .map(|(key, value)| format!("{}: {};", key, value))
            .collect::<Vec<String>>()
            .join(" ");

        styles.push_str("; z-index: 1000;");

        Ok(TooltipMsg::Position {
            orientation,
            styles,
        })
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
