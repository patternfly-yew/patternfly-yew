mod select;

pub use select::*;

use crate::{Button, Validator, WithBreakpoints};
use std::fmt::{Display, Formatter};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormHorizontal;

impl Display for FormHorizontal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("pf-m-horizontal")
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub horizontal: WithBreakpoints<FormHorizontal>,

    #[prop_or_default]
    pub limit_width: bool,

    #[prop_or_default]
    pub children: Children,
}

#[function_component(Form)]
pub fn form(props: &Props) -> Html {
    let mut classes = Classes::from("pf-c-form");

    classes.extend(props.horizontal.clone());

    if props.limit_width {
        classes.push("pf-m-limit-width");
    }

    html! {
        <form novalidate=true class={classes}>
            { for props.children.iter().map(|child|{
                    child
            }) }
        </form>
    }
}

// form group

#[derive(Clone, PartialEq, Properties)]
pub struct FormGroupProps {
    pub children: Children,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub helper_text: String,
}

pub struct FormGroup {}

impl Component for FormGroup {
    type Message = ();
    type Properties = FormGroupProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let classes = Classes::from("pf-c-form__group");

        html! {
            <div class={classes}>
                <div class="pf-c-form__group-label">

                    {if !ctx.props().label.is_empty() {
                        html!{
                            <div class="pf-c-form__label">
                                <span class="pf-c-form__label-text">{&ctx.props().label}</span>

                                {if ctx.props().required {
                                    html!{
                                        <span class="pf-c-form__label-required" aria-hidden="true">{"*"}</span>
                                    }
                                } else {
                                    html!{}
                                }}

                            </div>
                        }
                    } else {
                        html!{}
                    }}
                </div>

                <div class="pf-c-form__group-control">
                    { for ctx.props().children.iter() }
                    if !ctx.props().helper_text.is_empty() {
                        <p class="pf-c-form__helper-text" aria-live="polite">{ &ctx.props().helper_text }</p>
                    }
                </div>
            </div>
        }
    }
}

// form control

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InputState {
    Default,
    Success,
    Warning,
    Error,
}

impl Default for InputState {
    fn default() -> Self {
        Self::Default
    }
}

impl InputState {
    pub fn convert(&self, mut classes: Classes) -> (Classes, bool) {
        let mut aria_invalid = false;
        match self {
            InputState::Default => {}
            InputState::Success => classes.push("pf-m-success"),
            InputState::Warning => classes.push("pf-m-warning"),
            InputState::Error => aria_invalid = true,
        };
        (classes, aria_invalid)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TextInputIcon {
    None,
    Calendar,
    Clock,
    Search,
    Custom,
}

impl Default for TextInputIcon {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct TextInputProps {
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub state: InputState,
    #[prop_or_default]
    pub icon: TextInputIcon,
    #[prop_or("text".into())]
    pub r#type: String,
    #[prop_or_default]
    pub placeholder: String,

    #[prop_or_default]
    pub onchange: Callback<String>,
    #[prop_or_default]
    pub oninput: Callback<String>,

    #[prop_or_default]
    pub validator: Validator,
}

pub struct TextInput {
    value: Option<String>,
    input_ref: NodeRef,
}

pub enum TextInputMsg {
    Changed(String),
    Input(String),
}

impl Component for TextInput {
    type Message = TextInputMsg;
    type Properties = TextInputProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            value: None,
            input_ref: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TextInputMsg::Changed(data) => {
                self.value = Some(data.clone());
                ctx.props().onchange.emit(data);
            }
            TextInputMsg::Input(data) => {
                ctx.props().oninput.emit(data);
                if let Some(value) = self.extract_value() {
                    self.value = Some(value.clone());
                    ctx.props().onchange.emit(value);
                }
                // only re-render if we have a validator
                return ctx.props().validator.is_custom();
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if ctx.props().readonly {
            self.value = None;
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-c-form-control");

        match ctx.props().icon {
            TextInputIcon::None => {}
            TextInputIcon::Search => classes.push("pf-m-search"),
            TextInputIcon::Calendar => classes.extend(vec!["pf-m-icon", "pf-m-calendar"]),
            TextInputIcon::Clock => classes.extend(vec!["pf-m-icon", "pf-m-clock"]),
            TextInputIcon::Custom => classes.extend(vec!["pf-m-icon"]),
        };

        let (classes, aria_invalid) = self.input_state(ctx).convert(classes);

        let input_ref = self.input_ref.clone();
        let onchange = ctx.link().batch_callback(move |_| {
            input_ref
                .cast::<HtmlInputElement>()
                .map(|input| TextInputMsg::Changed(input.value()))
        });
        let oninput = ctx
            .link()
            .callback(|evt: InputEvent| TextInputMsg::Input(evt.data().unwrap_or_default()));

        let value = self.value(ctx);

        html! {
            <input
                ref={self.input_ref.clone()}
                class={classes}
                type={ctx.props().r#type.clone()}
                name={ctx.props().name.clone()}
                id={ctx.props().id.clone()}
                required={ctx.props().required}
                disabled={ctx.props().disabled}
                readonly={ctx.props().readonly}
                aria-invalid={aria_invalid.to_string()}
                value={value}
                placeholder={ctx.props().placeholder.clone()}
                onchange={onchange}
                oninput={oninput}
                />
        }
    }
}

impl TextInput {
    /// Extract the current value from the input element
    fn extract_value(&self) -> Option<String> {
        self.input_ref
            .cast::<HtmlInputElement>()
            .map(|input| input.value())
    }

    fn value(&self, ctx: &Context<Self>) -> String {
        self.value
            .clone()
            .unwrap_or_else(|| ctx.props().value.clone())
    }

    /// Get the effective input state
    ///
    /// This may be the result of the validator, or if none was set, the provided input state
    /// from the properties.
    fn input_state(&self, ctx: &Context<Self>) -> InputState {
        match &ctx.props().validator {
            Validator::Custom(validator) => validator(&self.value(ctx)),
            _ => ctx.props().state,
        }
    }
}

//
// Text area
//

#[derive(Clone, PartialEq, Eq)]
pub enum ResizeOrientation {
    Horizontal,
    Vertical,
    Both,
}

impl Default for ResizeOrientation {
    fn default() -> Self {
        Self::Both
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Wrap {
    Hard,
    Soft,
    Off,
}

impl Default for Wrap {
    fn default() -> Self {
        Self::Soft
    }
}

impl Display for Wrap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Off => f.write_str("off"),
            Self::Soft => f.write_str("soft"),
            Self::Hard => f.write_str("hard"),
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct TextAreaProps {
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub state: InputState,

    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub spellcheck: Option<bool>,
    #[prop_or_default]
    pub wrap: Wrap,

    #[prop_or_default]
    pub rows: Option<usize>,
    #[prop_or_default]
    pub cols: Option<usize>,

    #[prop_or_default]
    pub resize: ResizeOrientation,

    #[prop_or_default]
    pub onchange: Callback<String>,
    #[prop_or_default]
    pub oninput: Callback<String>,

    #[prop_or_default]
    pub validator: Validator,
}

pub struct TextArea {
    value: String,
    input_ref: NodeRef,
}

pub enum TextAreaMsg {
    Changed(String),
    Input(String),
}

impl Component for TextArea {
    type Message = TextAreaMsg;
    type Properties = TextAreaProps;

    fn create(ctx: &Context<Self>) -> Self {
        let value = ctx.props().value.clone();
        Self {
            value,
            input_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TextAreaMsg::Changed(data) => {
                self.value = data.clone();
                ctx.props().onchange.emit(data);
                false
            }
            TextAreaMsg::Input(data) => {
                ctx.props().oninput.emit(data);
                if let Some(value) = self.extract_value() {
                    self.value = value.clone();
                    ctx.props().onchange.emit(value);
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let classes = Classes::from("pf-c-form-control");
        let (mut classes, aria_invalid) = self.input_state(ctx).convert(classes);

        match ctx.props().resize {
            ResizeOrientation::Horizontal => classes.push("pf-m-resize-horizontal"),
            ResizeOrientation::Vertical => classes.push("pf-m-resize-vertical"),
            _ => {}
        }

        let input_ref = self.input_ref.clone();
        let onchange = ctx.link().batch_callback(move |_| {
            input_ref
                .cast::<HtmlInputElement>()
                .map(|input| TextAreaMsg::Changed(input.value()))
        });
        let oninput = ctx
            .link()
            .callback(|data: InputEvent| TextAreaMsg::Input(data.data().unwrap_or_default()));

        html! {
            <textarea
                ref={self.input_ref.clone()}
                class={classes}
                name={ctx.props().name.clone()}
                required={ctx.props().required}
                disabled={ctx.props().disabled}
                readonly={ctx.props().readonly}
                aria-invalid={aria_invalid.to_string()}
                value={ctx.props().value.clone()}

                cols={ctx.props().cols.as_ref().map(|v|v.to_string())}
                rows={ctx.props().rows.as_ref().map(|v|v.to_string())}

                wrap={ctx.props().wrap.to_string()}
                spellcheck={ctx.props().spellcheck.map(|v|v.to_string())}
                placeholder={ctx.props().placeholder.clone()}

                onchange={onchange}
                oninput={oninput}
                />
        }
    }
}

impl TextArea {
    /// Extract the current value from the input element
    fn extract_value(&self) -> Option<String> {
        self.input_ref
            .cast::<HtmlInputElement>()
            .map(|input| input.value())
    }

    /// Get the effective input state
    ///
    /// This may be the result of the validator, or if none was set, the provided input state
    /// from the properties.
    fn input_state(&self, ctx: &Context<Self>) -> InputState {
        match &ctx.props().validator {
            Validator::Custom(validator) => validator(&self.value),
            _ => ctx.props().state,
        }
    }
}

//
// Action group
//

#[derive(Clone, PartialEq, Properties)]
pub struct ActionGroupProps {
    pub children: ChildrenWithProps<Button>,
}

#[function_component(ActionGroup)]
pub fn action_group(props: &ActionGroupProps) -> Html {
    html! {
        <div class="pf-c-form__group pf-m-action">
            <div class="pf-c-form__group-control">
                <div class="pf-c-form__actions">
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}

//
// Input group
//

#[derive(Clone, PartialEq, Properties)]
pub struct InputGroupProps {
    pub children: Children,
}

#[function_component(InputGroup)]
pub fn input_group(props: &InputGroupProps) -> Html {
    html! {
        <div class="pf-c-input-group">
            { for props.children.iter() }
        </div>
    }
}
