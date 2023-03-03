use crate::{
    AsClasses, GroupValidationResult, Icon, InputState, ValidatingComponent,
    ValidatingComponentProperties, ValidationContext, ValidationFormContext, ValidationResult,
    Validator,
};
use std::{marker::PhantomData, rc::Rc};
use uuid::Uuid;
use yew::{prelude::*, virtual_dom::VNode};

// form group

/// Properties for [`FormGroup`]
#[derive(Clone, PartialEq, Properties)]
pub struct FormGroupProperties {
    pub children: Children,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub helper_text: Option<HelperText>,
}

/// Helper text information for a [`FormGroup`]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HelperText {
    pub input_state: InputState,
    pub custom_icon: Option<Icon>,
    pub no_icon: bool,
    pub is_dynamic: bool,
    pub message: String,
}

impl From<&HelperText> for VNode {
    fn from(text: &HelperText) -> Self {
        let mut classes = Classes::from("pf-c-helper-text__item");

        classes.extend(text.input_state.as_classes());

        if text.is_dynamic {
            classes.push("pf-m-dynamic");
        }

        html!(
            <div class={classes}>
                if !text.no_icon {
                    <span class="pf-c-helper-text__item-icon">
                        { text.custom_icon.unwrap_or_else(|| text.input_state.icon() )}
                    </span>
                }
                <span class="pf-c-helper-text__item-text"> { &text.message } </span>
            </div>
        )
    }
}

impl From<&str> for HelperText {
    fn from(text: &str) -> Self {
        HelperText {
            input_state: Default::default(),
            custom_icon: None,
            no_icon: true,
            is_dynamic: false,
            message: text.into(),
        }
    }
}

impl From<(&str, InputState)> for HelperText {
    fn from(value: (&str, InputState)) -> Self {
        Self {
            input_state: value.1,
            custom_icon: None,
            no_icon: false,
            is_dynamic: false,
            message: value.0.into(),
        }
    }
}

/// A group of components building a field in a [`Form`](crate::prelude::Form)
pub struct FormGroup {}

impl Component for FormGroup {
    type Message = ();
    type Properties = FormGroupProperties;

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
                    if let Some(text) = &ctx.props().helper_text {
                        { FormGroupHelpText(text) }
                    }
                </div>
            </div>
        }
    }
}

pub struct FormGroupHelpText<'a>(&'a HelperText);

impl<'a> FormGroupHelpText<'a> {}

impl<'a> From<FormGroupHelpText<'a>> for VNode {
    fn from(text: FormGroupHelpText<'a>) -> Self {
        let mut classes = classes!("pf-c-form__helper-text");

        classes.extend(text.0.input_state.as_classes());

        let icon = match text.0.no_icon {
            true => None,
            false => Some(
                text.0
                    .custom_icon
                    .unwrap_or_else(|| text.0.input_state.icon()),
            ),
        };

        html!(
            <p
                class={classes}
                aria-live="polite"
            >
                if let Some(icon) = icon {
                    <span class="pf-c-form__helper-text-icon">
                        { icon }
                    </span>
                }
                { &text.0.message }
            </p>
        )
    }
}

// with validation

/// Properties for [`FormGroupValidated`]
#[derive(Clone, Properties)]
pub struct FormGroupValidatedProperties<C>
where
    C: Component + ValidatingComponent,
{
    #[prop_or_default]
    pub children: ChildrenWithProps<C>,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub required: bool,
    pub validator: Validator<C::Value, ValidationResult>,

    #[prop_or_default]
    pub onvalidated: Callback<ValidationResult>,
}

#[doc(hidden)]
pub enum FormGroupValidatedMsg<C>
where
    C: ValidatingComponent,
{
    Validate(ValidationContext<C::Value>),
}

impl<C> PartialEq for FormGroupValidatedProperties<C>
where
    C: Component + ValidatingComponent,
{
    fn eq(&self, other: &Self) -> bool {
        self.required == other.required
            && self.label == other.label
            && self.children == other.children
    }
}

pub struct FormGroupValidated<C>
where
    C: Component,
{
    _marker: PhantomData<C>,

    id: String,
    state: Option<ValidationResult>,
}

impl<C> Component for FormGroupValidated<C>
where
    C: Component + ValidatingComponent,
    <C as Component>::Properties: ValidatingComponentProperties<C::Value> + Clone,
{
    type Message = FormGroupValidatedMsg<C>;
    type Properties = FormGroupValidatedProperties<C>;

    fn create(_: &Context<Self>) -> Self {
        Self {
            _marker: Default::default(),
            id: Uuid::new_v4().to_string(),
            state: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Validate(value) => {
                let state = ctx.props().validator.run(value);
                if self.state != state {
                    self.state = state;
                    ctx.props()
                        .onvalidated
                        .emit(self.state.clone().unwrap_or_default());
                    if let Some((validation_ctx, _)) = ctx
                        .link()
                        .context::<ValidationFormContext>(Callback::noop())
                    {
                        validation_ctx
                            .push_state(GroupValidationResult(self.id.clone(), self.state.clone()));
                    }
                }
            }
        }
        true
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        if let Some((ctx, _)) = ctx
            .link()
            .context::<ValidationFormContext>(Callback::noop())
        {
            ctx.clear_state(self.id.clone());
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onvalidate = ctx.link().callback(|v| FormGroupValidatedMsg::Validate(v));

        html!(
            <FormGroup
                label={ctx.props().label.clone()}
                required={ctx.props().required}
                helper_text={self.state.clone().and_then(|s|s.into())}
            >
                { for ctx.props().children.iter().map(|mut c|{
                    let props = Rc::make_mut(&mut c.props);
                    props.set_onvalidate(onvalidate.clone());
                    props.set_input_state(self.state.as_ref().map(|s|s.state).unwrap_or_default());
                    c
                })}
            </FormGroup>
        )
    }
}
