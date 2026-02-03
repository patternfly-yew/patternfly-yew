use crate::prelude::*;
use std::{marker::PhantomData, rc::Rc};
use uuid::Uuid;
use yew::{
    prelude::*,
    virtual_dom::{VChild, VNode},
};

// form group

/// Properties for [`FormGroup`]
#[derive(Clone, PartialEq, Properties)]
pub struct FormGroupProperties {
    pub children: Html,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub label_icon: LabelIcon,
    #[prop_or_default]
    pub helper_text: Option<FormHelperText>,
}

#[derive(Clone, Default, PartialEq)]
pub enum LabelIcon {
    /// No label icon
    #[default]
    None,
    /// Help
    Help(VChild<PopoverBody>),
    /// Any children
    Children(Html),
}

/// Helper text information for a [`FormGroup`]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormHelperText {
    pub input_state: InputState,
    pub custom_icon: Option<Icon>,
    pub no_icon: bool,
    pub is_dynamic: bool,
    pub message: String,
}

impl From<&FormHelperText> for VNode {
    fn from(text: &FormHelperText) -> Self {
        let mut classes = Classes::from("pf-v6-c-helper-text__item");

        classes.extend(text.input_state.as_classes());

        if text.is_dynamic {
            classes.push("pf-m-dynamic");
        }

        html!(
            <div class={classes}>
                if !text.no_icon {
                    <span class="pf-v6-c-helper-text__item-icon">
                        { text.custom_icon.unwrap_or_else(|| text.input_state.icon() )}
                    </span>
                }
                <span class="pf-v6-c-helper-text__item-text"> { &text.message } </span>
            </div>
        )
    }
}

impl From<&str> for FormHelperText {
    fn from(text: &str) -> Self {
        FormHelperText {
            input_state: Default::default(),
            custom_icon: None,
            no_icon: true,
            is_dynamic: false,
            message: text.into(),
        }
    }
}

impl From<(String, InputState)> for FormHelperText {
    fn from((message, input_state): (String, InputState)) -> Self {
        Self {
            input_state,
            custom_icon: None,
            no_icon: false,
            is_dynamic: false,
            message,
        }
    }
}

impl From<(&str, InputState)> for FormHelperText {
    fn from((message, input_state): (&str, InputState)) -> Self {
        Self {
            input_state,
            custom_icon: None,
            no_icon: false,
            is_dynamic: false,
            message: message.to_string(),
        }
    }
}

/// A group of components building a field in a [`Form`](crate::prelude::Form)
///
/// ## Properties
///
/// Defined by [`FormGroupProperties`].
pub struct FormGroup {}

impl Component for FormGroup {
    type Message = ();
    type Properties = FormGroupProperties;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let classes = Classes::from("pf-v6-c-form__group");

        html! (
            <div class={classes}>

                if !ctx.props().label.is_empty() {
                    <div class="pf-v6-c-form__group-label">
                        <label class="pf-v6-c-form__label">

                            <span class="pf-v6-c-form__label-text">{&ctx.props().label}</span>

                            if ctx.props().required {
                                {" "}
                                <span class="pf-v6-c-form__label-required" aria-hidden="true">{"*"}</span>
                            }
                        </label>
                        {
                            match &ctx.props().label_icon  {
                                LabelIcon::None => html!(),
                                LabelIcon::Help(popover) => html!(
                                    <span
                                        class="pf-v6-c-form__group-label-help"
                                        role="button"
                                        type="button"
                                        tabindex=0
                                    >
                                        {" "}
                                        <Popover target={html!(Icon::QuestionCircle)} body={popover.clone()} />
                                    </span>
                                ),
                                LabelIcon::Children(children) => children.clone(),
                            }
                        }
                    </div>
                }

                <div class="pf-v6-c-form__group-control">
                    { ctx.props().children.clone() }
                    if let Some(text) = &ctx.props().helper_text {
                        { FormGroupHelpText(text) }
                    }
                </div>
            </div>
        )
    }
}

pub struct FormGroupHelpText<'a>(pub &'a FormHelperText);

impl<'a> FormGroupHelpText<'a> {}

impl<'a> From<FormGroupHelpText<'a>> for VNode {
    fn from(text: FormGroupHelpText<'a>) -> Self {
        let mut classes = classes!("pf-v6-c-helper-text__item");

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
            <div
                class="pf-v6-c-form__helper-text"
                aria-live="polite"
            >
                <div class="pf-v6-c-helper-text">
                    <div
                        class={classes}
                        id="form-help-text-info-helper"
                    >
                        if let Some(icon) = icon {
                            <span class="pf-v6-c-helper-text__item-icon">
                                { icon }
                            </span>
                        }
                        <span class="pf-v6-c-helper-text__item-text">
                            { &text.0.message }
                        </span>
                    </div>
                </div>
            </div>
        )
    }
}

// with validation

/// Properties for [`FormGroupValidated`]
#[derive(Clone, Properties)]
pub struct FormGroupValidatedProperties<C>
where
    C: BaseComponent + ValidatingComponent,
{
    #[prop_or_default]
    pub children: ChildrenWithProps<C>,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub label_icon: LabelIcon,
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
    C: BaseComponent + ValidatingComponent,
{
    fn eq(&self, other: &Self) -> bool {
        self.required == other.required
            && self.label == other.label
            && self.children == other.children
    }
}

pub struct FormGroupValidated<C>
where
    C: BaseComponent,
{
    _marker: PhantomData<C>,

    id: String,
    state: Option<ValidationResult>,
}

impl<C> Component for FormGroupValidated<C>
where
    C: BaseComponent + ValidatingComponent,
    <C as BaseComponent>::Properties: ValidatingComponentProperties<C::Value> + Clone,
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onvalidate = ctx.link().callback(|v| FormGroupValidatedMsg::Validate(v));

        html!(
            <FormGroup
                label={ctx.props().label.clone()}
                label_icon={ctx.props().label_icon.clone()}
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

    fn destroy(&mut self, ctx: &Context<Self>) {
        if let Some((ctx, _)) = ctx
            .link()
            .context::<ValidationFormContext>(Callback::noop())
        {
            ctx.clear_state(self.id.clone());
        }
    }
}
