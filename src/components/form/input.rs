use crate::{
    AsClasses, InputState, ValidatingComponent, ValidatingComponentProperties, ValidationContext,
    Validator,
};
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

impl AsClasses for TextInputIcon {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::None => {}
            Self::Search => classes.extend(classes!("pf-m-search")),
            Self::Calendar => classes.extend(classes!("pf-m-icon", "pf-m-calendar")),
            Self::Clock => classes.extend(classes!("pf-m-icon", "pf-m-clock")),
            Self::Custom => classes.extend(classes!("pf-m-icon")),
        }
    }
}

impl Default for TextInputIcon {
    fn default() -> Self {
        Self::None
    }
}

/// Properties for [`TextInput`]
#[derive(Clone, PartialEq, Properties)]
pub struct TextInputProperties {
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
    pub autofocus: bool,
    #[prop_or_default]
    pub form: String,
    #[prop_or_default]
    pub autocomplete: String,

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

#[allow(deprecated)]
impl ValidatingComponent for TextInput {
    type Value = String;
}

#[allow(deprecated)]
impl ValidatingComponentProperties<String> for TextInputProperties {
    fn set_onvalidate(&mut self, onvalidate: Callback<ValidationContext<String>>) {
        self.onvalidate = onvalidate;
    }

    fn set_input_state(&mut self, state: InputState) {
        self.state = state;
    }
}

#[deprecated(
    since = "0.4.0",
    note = "Will be replaced with the implementation from `next::TextInput`"
)]
/// A text input component
pub struct TextInput {
    value: Option<String>,
    refs: Refs,
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

#[allow(deprecated)]
impl Component for TextInput {
    type Message = TextInputMsg;
    type Properties = TextInputProperties;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Self::Message::Init);

        Self {
            value: None,
            refs: Refs {
                input: ctx.props().r#ref.clone(),
            },
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
                .send_message(TextInputMsg::Changed(ctx.props().value.clone()))
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

#[allow(deprecated)]
impl TextInput {
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
            .unwrap_or_else(|| ctx.props().value.clone())
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

/// Upcoming version of the [`TextInput`] component.
pub mod next {
    use super::*;

    use crate::{
        focus, value, ExtendClasses, InputState, ValidatingComponent,
        ValidatingComponentProperties, ValidationContext,
    };

    /// Properties for [`TextInput`]
    #[derive(Clone, PartialEq, Properties)]
    pub struct TextInputProperties {
        #[prop_or_default]
        pub name: AttrValue,
        #[prop_or_default]
        pub id: AttrValue,
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
        pub r#type: AttrValue,
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
        ///
        /// The difference is that the oninput event occurs immediately after the value of an element has changed.
        ///
        /// **NOTE:** Contrary to the HTML definition of oninput, the callback provides the full value
        /// of the input element, not just the changed part.
        #[prop_or_default]
        pub oninput: Callback<String>,
        // Called when validation should occur
        #[prop_or_default]
        pub onvalidate: Callback<ValidationContext<String>>,

        #[prop_or_default]
        pub r#ref: NodeRef,
    }

    impl ValidatingComponent for TextInput {
        type Value = String;
    }

    impl ValidatingComponentProperties<String> for TextInputProperties {
        fn set_onvalidate(&mut self, onvalidate: Callback<ValidationContext<String>>) {
            self.onvalidate = onvalidate;
        }

        fn set_input_state(&mut self, state: InputState) {
            self.state = state;
        }
    }

    /// Text input component
    ///
    /// > A **text input** is used to gather free-form text from a user.
    ///
    /// See: <https://www.patternfly.org/v4/components/text-input>
    ///
    /// ## Properties
    ///
    /// Defined by [`TextInputProperties].
    ///
    /// ## Change events
    ///
    /// The component emits changes of the input value through the `onchange` event once the
    /// component looses the focus (same of plain HTML). It also emits the full input value via the
    /// `oninput` event and does the same using the `onvalidate` event. This duplication is required
    /// to support both change events as well as supporting the [`ValidatingComponent`] trait.
    ///
    /// If a value is provided via the `value` property, that value must be updated through the
    /// `oninput` callback. Otherwise the value will be reset immediately and the component will
    /// be effectively read-only:
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use patternfly_yew::next::TextInput;
    /// use patternfly_yew::prelude::*;
    ///
    /// #[function_component(Example)]
    /// fn example() -> Html {
    ///   let value = use_state_eq(String::default);
    ///   let onchange = {
    ///     let value = value.clone();
    ///     Callback::from(move |data| value.set(data))
    ///   };
    ///
    ///   html!(<TextInput value={(*value).clone()}/>)
    /// }
    /// ```
    #[function_component(TextInput)]
    pub fn text_input(props: &TextInputProperties) -> Html {
        let input_ref = props.r#ref.clone();
        let mut classes = classes!("pf-c-form-control");
        classes.extend_from(&props.icon);

        // validation

        {
            let value = props.value.clone();
            let onvalidate = props.onvalidate.clone();
            use_effect_with_deps(
                move |()| {
                    onvalidate.emit(ValidationContext {
                        value,
                        initial: true,
                    });
                },
                (),
            );
        }

        let (classes, aria_invalid) = props.state.convert(classes);

        // autofocus

        {
            let autofocus = props.autofocus;
            use_effect_with_deps(
                move |input_ref| {
                    if autofocus {
                        focus(input_ref)
                    }
                },
                input_ref.clone(),
            );
        }

        // change events

        let onchange = use_memo(
            |(onchange, input_ref)| {
                let input_ref = input_ref.clone();
                onchange.reform(move |_: Event| value(&input_ref).unwrap_or_default())
            },
            (props.onchange.clone(), input_ref.clone()),
        );

        let oninput = use_memo(
            |(oninput, onvalidate, input_ref)| {
                let input_ref = input_ref.clone();
                let oninput = oninput.clone();
                let onvalidate = onvalidate.clone();
                Callback::from(move |_: InputEvent| {
                    // get the (complete) current value
                    let value = value(&input_ref).unwrap_or_default();
                    oninput.emit(value.clone());
                    onvalidate.emit(value.into());
                })
            },
            (
                props.oninput.clone(),
                props.onvalidate.clone(),
                input_ref.clone(),
            ),
        );

        html! {
            <input
                ref={input_ref}
                class={classes}
                type={&props.r#type}
                name={&props.name}
                id={&props.id}
                required={props.required}
                disabled={props.disabled}
                readonly={props.readonly}
                aria-invalid={aria_invalid.to_string()}
                value={props.value.clone()}
                placeholder={&props.placeholder}
                form={&props.form}
                autocomplete={&props.autocomplete}
                onchange={(*onchange).clone()}
                oninput={(*oninput).clone()}
            />
        }
    }
}
