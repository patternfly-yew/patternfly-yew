use crate::{InputState, ValidatingComponentProperties, ValidationContext, Validator};
use web_sys::HtmlInputElement;
use yew::prelude::*;

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
    // Called when validation should occur
    #[prop_or_default]
    pub onvalidate: Callback<ValidationContext>,

    #[prop_or_default]
    pub validator: Validator<InputState>,
}

impl ValidatingComponentProperties for TextInputProps {
    fn set_onvalidate(&mut self, onvalidate: Callback<ValidationContext>) {
        self.onvalidate = onvalidate;
    }

    fn set_input_state(&mut self, state: InputState) {
        self.state = state;
    }
}

pub struct TextInput {
    value: Option<String>,
    input_ref: NodeRef,
}

pub enum TextInputMsg {
    Init,
    Changed(String),
    Input(String),
}

impl Component for TextInput {
    type Message = TextInputMsg;
    type Properties = TextInputProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Self::Message::Init);

        Self {
            value: None,
            input_ref: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TextInputMsg::Init => {
                ctx.props().onvalidate.emit(ValidationContext {
                    value: self.value(ctx),
                    initial: true,
                });
            }
            TextInputMsg::Changed(data) => {
                self.value = Some(data.clone());
                ctx.props().onchange.emit(data.clone());
                ctx.props().onvalidate.emit(data.into());
            }
            TextInputMsg::Input(data) => {
                ctx.props().oninput.emit(data);
                if let Some(value) = self.extract_value() {
                    self.value = Some(value.clone());
                    ctx.props().onchange.emit(value.clone());
                    ctx.props().onvalidate.emit(value.into());
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
            Validator::Custom(validator) => validator(self.value(ctx).into()),
            _ => ctx.props().state,
        }
    }
}
