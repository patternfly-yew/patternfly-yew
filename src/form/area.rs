use crate::{InputState, ValidatingComponentProperties, ValidationContext, Validator};
use std::fmt::{Display, Formatter};
use web_sys::HtmlInputElement;
use yew::prelude::*;

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
    // Called when validation should occur
    #[prop_or_default]
    pub onvalidate: Callback<ValidationContext>,

    #[prop_or_default]
    pub validator: Validator<InputState>,
}

pub struct TextArea {
    value: String,
    input_ref: NodeRef,
}

pub enum TextAreaMsg {
    Init,
    Changed(String),
    Input(String),
}

impl Component for TextArea {
    type Message = TextAreaMsg;
    type Properties = TextAreaProps;

    fn create(ctx: &Context<Self>) -> Self {
        let value = ctx.props().value.clone();
        ctx.link().send_message(Self::Message::Init);

        Self {
            value,
            input_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TextAreaMsg::Init => {
                ctx.props().onvalidate.emit(ValidationContext {
                    value: self.value.clone(),
                    initial: true,
                });
                false
            }
            TextAreaMsg::Changed(data) => {
                self.value = data.clone();
                ctx.props().onchange.emit(data.clone());
                ctx.props().onvalidate.emit(data.into());
                false
            }
            TextAreaMsg::Input(data) => {
                ctx.props().oninput.emit(data);
                if let Some(value) = self.extract_value() {
                    self.value = value.clone();
                    ctx.props().onchange.emit(value.clone());
                    ctx.props().onvalidate.emit(value.clone().into());
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
            Validator::Custom(validator) => validator(self.value.clone().into()),
            _ => ctx.props().state,
        }
    }
}

impl ValidatingComponentProperties for TextAreaProps {
    fn set_onvalidate(&mut self, onvalidate: Callback<ValidationContext>) {
        self.onvalidate = onvalidate;
    }

    fn set_input_state(&mut self, state: InputState) {
        self.state = state;
    }
}
