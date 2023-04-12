use crate::{
    InitialValue, InputState, ValidatingComponent, ValidatingComponentProperties,
    ValidationContext, Validator,
};
use std::marker::PhantomData;
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Icons as part of a [`TextInput`] component.
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

/// Properties for [`TextInput`]
#[derive(Clone, PartialEq, Properties)]
pub struct TextInputProperties<I = String>
where
    I: InitialValue<String>,
{
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub value: I,
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
    pub placeholder: AttrValue,
    #[prop_or_default]
    pub autofocus: bool,
    #[prop_or_default]
    pub form: AttrValue,
    #[prop_or_default]
    pub autocomplete: AttrValue,

    /// This event is triggered when the element loses focus.
    #[prop_or_default]
    pub onchange: Callback<String>,
    /// This event is similar to the onchange event.
    /// The difference is that the oninput event occurs immediately after the value of an element has changed.
    #[prop_or_default]
    pub oninput: Callback<String>,
    // Called when validation should occur
    #[prop_or_default]
    pub onvalidate: Callback<ValidationContext<String>>,

    #[prop_or_default]
    pub validator: Validator<String, InputState>,

    #[prop_or_default]
    pub r#ref: NodeRef,
}

impl ValidatingComponent for TextInput {
    type Value = String;
}

impl<I> ValidatingComponentProperties<String> for TextInputProperties<I>
where
    I: InitialValue<String>,
{
    fn set_onvalidate(&mut self, onvalidate: Callback<ValidationContext<String>>) {
        self.onvalidate = onvalidate;
    }

    fn set_input_state(&mut self, state: InputState) {
        self.state = state;
    }
}

/// A text input component
pub struct TextInput<I = String>
where
    I: InitialValue<String>,
{
    value: Option<String>,
    refs: Refs,
    _marker: PhantomData<I>,
}

#[derive(Default)]
struct Refs {
    input: NodeRef,
}

#[doc(hidden)]
pub enum TextInputMsg {
    Init,
    Changed(String),
    Input(String),
}

impl<I> Component for TextInput<I>
where
    I: InitialValue<String> + 'static,
{
    type Message = TextInputMsg;
    type Properties = TextInputProperties<I>;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Self::Message::Init);

        Self {
            value: None,
            refs: Refs {
                input: ctx.props().r#ref.clone(),
            },
            _marker: Default::default(),
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

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if ctx.props().value != old_props.value {
            // initial value has changed
            ctx.link()
                .send_message(TextInputMsg::Changed(ctx.props().value.create()))
        }
        if ctx.props().readonly {
            self.value = None;
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = classes!("pf-c-form-control");

        match ctx.props().icon {
            TextInputIcon::None => {}
            TextInputIcon::Search => classes.extend(classes!("pf-m-search")),
            TextInputIcon::Calendar => classes.extend(classes!("pf-m-icon", "pf-m-calendar")),
            TextInputIcon::Clock => classes.extend(classes!("pf-m-icon", "pf-m-clock")),
            TextInputIcon::Custom => classes.extend(classes!("pf-m-icon")),
        };

        let (classes, aria_invalid) = self.input_state(ctx).convert(classes);

        let input_ref = self.refs.input.clone();
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
                ref={self.refs.input.clone()}
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
                form={ctx.props().form.clone()}
                autocomplete={ctx.props().autocomplete.clone()}
                onchange={onchange}
                oninput={oninput}
            />
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render && ctx.props().autofocus {
            self.focus();
        }
    }
}

impl<I> TextInput<I>
where
    I: InitialValue<String> + 'static,
{
    /// Extract the current value from the input element
    fn extract_value(&self) -> Option<String> {
        self.refs
            .input
            .cast::<HtmlInputElement>()
            .map(|input| input.value())
    }

    fn value(&self, ctx: &Context<Self>) -> String {
        self.value
            .clone()
            .unwrap_or_else(|| ctx.props().value.create())
    }

    fn focus(&self) {
        if let Some(input) = self.refs.input.cast::<HtmlInputElement>() {
            input.focus().ok();
        }
    }

    /// Get the effective input state
    ///
    /// This may be the result of the validator, or if none was set, the provided input state
    /// from the properties.
    fn input_state(&self, ctx: &Context<Self>) -> InputState {
        ctx.props()
            .validator
            .run_if(|| ValidationContext::from(self.value(ctx)))
            .unwrap_or_else(|| ctx.props().state)
    }
}
