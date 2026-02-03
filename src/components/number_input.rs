use std::{fmt::Display, str::FromStr};

use crate::prelude::*;
use num_traits::PrimInt;
use yew::prelude::*;

/// Position of the number input unit in relation to the number input.
#[derive(Debug, Clone, PartialEq)]
pub enum NumberInputUnit {
    Before(Html),
    After(Html),
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct NumberInputProperties<T: PrimInt + Display + FromStr + 'static> {
    /// Value of the number input.
    #[prop_or(T::zero())]
    pub value: T,
    /// Additional classes added to the number input.
    #[prop_or_default]
    pub class: Classes,
    /// Sets the width of the number input to a number of characters
    #[prop_or_default]
    pub width_chars: Option<u8>,
    /// Indicates whether the whole number input should be disabled.
    #[prop_or_default]
    pub disabled: bool,
    /// Callback for the minus button.
    #[prop_or_default]
    pub onminus: Option<Callback<()>>,
    /// Callback the text input changing.
    #[prop_or_default]
    pub onchange: Option<Callback<T>>,
    /// Callback for the plus button.
    #[prop_or_default]
    pub onplus: Option<Callback<()>>,
    /// Adds the given unit to the number input.
    #[prop_or_default]
    pub unit: Option<NumberInputUnit>,
    /// Minimum value of the number input, disabling the minus button when reached.
    #[prop_or(T::min_value())]
    pub min: T,
    /// Maximum value of the number input, disabling the plus button when reached.
    #[prop_or(T::max_value())]
    pub max: T,
    /// Value to indicate if the input is modified to show the validiation state.
    #[prop_or_default]
    pub state: InputState,
    /// Name of the input.
    #[prop_or_default]
    pub input_name: Option<String>,
    /// Aria label of the minus button.
    #[prop_or(AttrValue::from("Minus"))]
    pub minus_button_aria_label: AttrValue,
    /// Aria label of the plus button.
    #[prop_or(AttrValue::from("Plus"))]
    pub plus_button_aria_label: AttrValue,
}

#[function_component(NumberInput)]
pub fn number_input<T: PrimInt + Display + FromStr + 'static>(
    props: &NumberInputProperties<T>,
) -> Html {
    let mut class = props.class.clone();
    class.push("pf-v6-c-number-input");
    if props.state != InputState::Default {
        class.push("pf-m-status");
    }
    let width_style_name = "--pf-v6-c-number-input--c-form-control--width-chars";
    let style = props
        .width_chars
        .map(|w| format!("{width_style_name}:{w};"));

    let onminusclick = use_callback(props.onminus.clone(), |_, onminus| {
        if let Some(onminus) = onminus {
            onminus.emit(());
        }
    });
    let onplusclick = use_callback(props.onplus.clone(), |_, onplus| {
        if let Some(onplus) = onplus {
            onplus.emit(());
        }
    });
    let onchange = use_callback(props.onchange.clone(), |new_val: String, onchange| {
        let Some(onchange) = onchange else {
            return;
        };
        match new_val.parse::<T>() {
            Ok(n) => onchange.emit(n),
            Err(_) => log::warn!("[NumberInput] Failed to parse {new_val} into a number."),
        };
    });
    html! {
        <div {class} {style}>
            if let Some(NumberInputUnit::Before(unit)) = &props.unit {
                <Unit>{unit.clone()}</Unit>
            }
            <InputGroup>
                <InputGroupItem>
                    <Button
                        variant={ButtonVariant::Control}
                        aria_label={props.minus_button_aria_label.clone()}
                        disabled={props.disabled || props.value <= props.min}
                        onclick={onminusclick}
                    >
                        <span class="pf-v6-c-number-input__icon">
                            {Icon::Minus}
                        </span>
                    </Button>
                </InputGroupItem>
                <InputGroupItem>
                    <TextInput
                        r#type={TextInputType::Number}
                        value={props.value.to_string()}
                        name={props.input_name.clone()}
                        disabled={props.disabled}
                        onchange={onchange}
                        state={props.state}
                    />
                </InputGroupItem>
                <InputGroupItem>
                    <Button
                        variant={ButtonVariant::Control}
                        aria_label={props.plus_button_aria_label.clone()}
                        disabled={props.disabled || props.value >= props.max}
                        onclick={onplusclick}
                    >
                        <span class="pf-v6-c-number-input__icon">
                            {Icon::Plus}
                        </span>
                    </Button>
                </InputGroupItem>
            </InputGroup>
            if let Some(NumberInputUnit::After(unit)) = &props.unit {
                <Unit>{unit.clone()}</Unit>
            }
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct UnitProperties {
    children: Html,
}

#[function_component(Unit)]
fn unit(props: &UnitProperties) -> Html {
    html!(<div class="pf-v6-c-number-input__unit">{props.children.clone()}</div>)
}
