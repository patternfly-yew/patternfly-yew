use crate::{AsClasses, Icon, InputState, ValidationContext, ValidationResult, Validator};
use std::marker::PhantomData;
use std::rc::Rc;
use yew::{prelude::*, virtual_dom::VNode};

// form group

#[derive(Clone, PartialEq, Properties)]
pub struct FormGroupProps {
    pub children: Children,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub helper_text: Option<HelperText>,
}

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
        let mut classes = Classes::from_iter(&["pf-c-form__helper-text".to_string()]);

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

#[derive(Clone, Properties)]
pub struct FormGroupValidatedProps<C>
where
    C: Component + ValidatingComponent,
{
    #[prop_or_default]
    pub children: ChildrenWithProps<C>,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub required: bool,
    pub validator: Validator<ValidationResult, C::Value>,
}

pub enum FormGroupValidatedMsg<C>
where
    C: ValidatingComponent,
{
    Validate(ValidationContext<C::Value>),
}

impl<C> PartialEq for FormGroupValidatedProps<C>
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

    state: Option<ValidationResult>,
}

pub trait ValidatingComponent {
    type Value;
}

pub trait ValidatingComponentProperties<T> {
    fn set_onvalidate(&mut self, onvalidate: Callback<ValidationContext<T>>);
    fn set_input_state(&mut self, state: InputState);
}

impl<C> Component for FormGroupValidated<C>
where
    C: Component + ValidatingComponent,
    <C as Component>::Properties: ValidatingComponentProperties<C::Value> + Clone,
{
    type Message = FormGroupValidatedMsg<C>;
    type Properties = FormGroupValidatedProps<C>;

    fn create(_: &Context<Self>) -> Self {
        Self {
            _marker: Default::default(),
            state: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Validate(value) => {
                self.state = ctx.props().validator.run(value);
            }
        }
        true
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
