//! Slider control
use gloo_events::{EventListener, EventListenerOptions};
use gloo_utils::document;
use std::fmt::{Display, Formatter};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::html::IntoPropValue;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Step {
    pub value: f64,
    pub label: Option<String>,
}

impl From<f64> for Step {
    fn from(value: f64) -> Self {
        Step { value, label: None }
    }
}

impl IntoPropValue<Step> for f64 {
    fn into_prop_value(self) -> Step {
        self.into()
    }
}

impl<S> IntoPropValue<Step> for (f64, S)
where
    S: AsRef<str>,
{
    fn into_prop_value(self) -> Step {
        Step {
            value: self.0,
            label: Some(self.1.as_ref().into()),
        }
    }
}

impl Display for Step {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.label {
            Some(label) => f.write_str(label),
            None => write!(f, "{}", self.value),
        }
    }
}

/// Properties for [`Slider`]
#[derive(Clone, PartialEq, Properties)]
pub struct SliderProperties {
    /// The minimum value.
    pub min: Step,
    /// The maximum value.
    pub max: Step,

    /// The initial value.
    #[prop_or_default]
    pub value: Option<f64>,

    /// Flag to hide the label.
    #[prop_or_default]
    pub hide_labels: bool,

    /// The precision of the value label.
    #[prop_or(2)]
    pub label_precision: usize,

    #[prop_or_default]
    pub ticks: Vec<Step>,

    /// An option to suppress reporting the initial value as change.
    #[prop_or_default]
    pub suppress_initial_change: bool,

    /// A callback reporting changes.
    #[prop_or_default]
    pub onchange: Callback<f64>,
}

#[doc(hidden)]
pub enum Msg {
    // set the value as original value
    SetValue(f64),
    Start(Input, i32),
    Move(i32),
    Stop,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Input {
    Mouse,
    Touch,
}

/// Slider component
///
/// > A **slider** provides a quick and effective way for users to set and adjust a numeric value from a defined range of values.
///
/// See: <https://www.patternfly.org/v4/components/slider>
///
/// ## Properties
///
/// Defined by [`SliderProperties`].
pub struct Slider {
    // value in percent (0..=1)
    value: f64,

    mousemove: Option<EventListener>,
    mouseup: Option<EventListener>,
    touchmove: Option<EventListener>,
    touchend: Option<EventListener>,
    touchcancel: Option<EventListener>,

    refs: Refs,
}

#[derive(Default)]
struct Refs {
    rail: NodeRef,
}

impl Component for Slider {
    type Message = Msg;
    type Properties = SliderProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let value = match ctx.props().value {
            Some(value) => value,
            None => ctx.props().min.value,
        };

        if !ctx.props().suppress_initial_change {
            // initial send a change event
            ctx.props().onchange.emit(value);
        }

        Self {
            value,
            refs: Default::default(),

            mousemove: None,
            mouseup: None,
            touchmove: None,
            touchend: None,
            touchcancel: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetValue(value) => {
                if self.value != value {
                    self.value = value;
                    ctx.props().onchange.emit(self.value);
                } else {
                    return false;
                }
            }
            Msg::Start(input, x) => {
                log::info!("Start: {x}");
                match input {
                    Input::Mouse => self.start_mouse(ctx),
                    Input::Touch => self.start_touch(ctx),
                }
            }
            Msg::Move(x) => {
                log::info!("Move: {x}");
                self.r#move(ctx, x);
            }
            Msg::Stop => {
                log::info!("Stop");
                self.mousemove = None;
                self.mouseup = None;
                self.touchmove = None;
                self.touchend = None;
                self.touchcancel = None;
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        let props = ctx.props();
        if old_props != props {
            if old_props.value != props.value {
                if let Some(value) = props.value {
                    ctx.link().send_message(Msg::SetValue(value));
                }
            };
            true
        } else {
            false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let classes = Classes::from("pf-c-slider");
        let valuestr = format!("{0:.1$}", self.value, ctx.props().label_precision);
        let valuestr = valuestr.trim_end_matches('0').to_string();

        let onmousedown = ctx.link().callback(|e: MouseEvent| {
            e.stop_propagation();
            e.prevent_default();
            Msg::Start(Input::Mouse, e.client_x())
        });

        let ontouchstart = ctx.link().batch_callback(|e: TouchEvent| {
            e.stop_propagation();
            if let Some(t) = e.touches().get(0) {
                vec![Msg::Start(Input::Touch, t.client_x())]
            } else {
                vec![]
            }
        });
        let percent = Self::calc_percent(self.value, ctx.props()) * 100f64;
        let min = &ctx.props().min;
        let max = &ctx.props().max;

        html!(
            <div class={classes} style={format!("--pf-c-slider--value: {}%", percent)}>
                <div class="pf-c-slider__main">
                    <div class="pf-c-slider__rail" ref={self.refs.rail.clone()}>
                        <div class="pf-c-slider__rail-track"></div>
                    </div>
                    if !ctx.props().hide_labels {
                        <div class="pf-c-slider__steps" aria-hidden="true">
                            { self.render_step(min, ctx.props()) }
                            { for ctx.props().ticks.iter()
                                .filter(|t| t.value>min.value && t.value<max.value)
                                .map(|t| self.render_step(t,ctx.props()))}
                            { self.render_step(max, ctx.props()) }
                        </div>
                    }
                    <div class="pf-c-slider__thumb"
                        {onmousedown}
                        {ontouchstart}
                        role="slider"
                        aria-valuemin={ctx.props().min.value.to_string()}
                        aria-valuemax={ctx.props().max.value.to_string()}
                        aria-valuenow={valuestr}
                        aria-label="Value"
                        tabindex="0"
                        >
                    </div>
                </div>
            </div>
        )
    }
}

impl Slider {
    fn start_mouse(&mut self, ctx: &Context<Self>) {
        let onmove = ctx.link().callback(Msg::Move);
        let onstop = ctx.link().callback(|_: ()| Msg::Stop);

        let mousemove = {
            let onmove = onmove;
            EventListener::new_with_options(
                &document(),
                "mousemove",
                EventListenerOptions::enable_prevent_default(),
                move |event| {
                    if let Some(e) = event.dyn_ref::<MouseEvent>() {
                        e.stop_propagation();
                        e.prevent_default();
                        onmove.emit(e.client_x());
                    }
                },
            )
        };
        self.mousemove = Some(mousemove);

        let mouseup = EventListener::new_with_options(
            &document(),
            "mouseup",
            EventListenerOptions::default(),
            move |_| {
                onstop.emit(());
            },
        );
        self.mouseup = Some(mouseup);
    }

    fn start_touch(&mut self, ctx: &Context<Self>) {
        let onmove = ctx.link().callback(Msg::Move);
        let onstop = ctx.link().callback(|_: ()| Msg::Stop);

        let touchmove = EventListener::new_with_options(
            &document(),
            "touchmove",
            EventListenerOptions::enable_prevent_default(),
            move |event| {
                if let Some(e) = event.dyn_ref::<TouchEvent>() {
                    e.prevent_default();
                    e.stop_immediate_propagation();
                    if let Some(t) = e.touches().get(0) {
                        onmove.emit(t.client_x());
                    }
                }
            },
        );
        self.touchmove = Some(touchmove);

        let touchend = {
            let onstop = onstop.clone();
            EventListener::new_with_options(
                &document(),
                "touchend",
                EventListenerOptions::default(),
                move |_| {
                    onstop.emit(());
                },
            )
        };
        self.touchend = Some(touchend);

        let touchcancel = EventListener::new_with_options(
            &document(),
            "touchcancel",
            EventListenerOptions::default(),
            move |_| {
                onstop.emit(());
            },
        );
        self.touchcancel = Some(touchcancel);
    }

    fn r#move(&mut self, ctx: &Context<Self>, x: i32) {
        if let Some(ele) = self.refs.rail.cast::<HtmlElement>() {
            let bounding = ele.get_bounding_client_rect();

            let left = bounding.left();
            let width = bounding.width();

            let value = x as f64 - left;

            log::info!("Left: {left}, width: {width}, value: {value}");

            let value = if value <= 0f64 {
                0f64
            } else if value >= width {
                1f64
            } else {
                value / width
            };

            ctx.link()
                .send_message(Msg::SetValue(Self::calc_value(value, ctx.props())))
        }
    }

    fn calc_percent(value: f64, props: &SliderProperties) -> f64 {
        let delta = props.max.value - props.min.value;
        let p = (value - props.min.value) / delta;
        p.clamp(0f64, 1f64)
    }

    fn calc_value(p: f64, props: &SliderProperties) -> f64 {
        let delta = props.max.value - props.min.value;
        props.min.value + delta * p
    }

    fn render_step(&self, step: &Step, props: &SliderProperties) -> Html {
        let active = step.value <= self.value;

        let mut classes = Classes::from("pf-c-slider__step");
        if active {
            classes.push("pf-m-active");
        }
        let label = if let Some(label) = &step.label {
            label.clone()
        } else {
            format!("{:.1$}", step.value, props.label_precision)
        };
        let position = Self::calc_percent(step.value, props) * 100f64;
        html!(
            <div class={classes} style={format!("--pf-c-slider__step--Left: {}%", position)}>
                <div class="pf-c-slider__step-tick"></div>
                <div class="pf-c-slider__step-label">{ label }</div>
            </div>
        )
    }
}
