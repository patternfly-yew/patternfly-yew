use crate::prelude::{
    CalendarView, HelperText, HelperTextItem, HelperTextItemVariant, InputGroup, InputGroupItem,
    InputState, Popover, PopoverBody, PopoverContext, TextInput,
};
use chrono::{Local, NaiveDate, Weekday};
use yew::prelude::*;

/// Properties for [`DatePicker`].
#[derive(Clone, PartialEq, Properties)]
pub struct DatePickerProperties {
    /// Disable the component
    #[prop_or_default]
    pub disabled: bool,
    /// The change callback
    #[prop_or_default]
    pub onchange: Callback<NaiveDate>,
    /// The placeholder string
    #[prop_or(String::from("YYYY-MM-DD"))]
    pub placeholder: String,
    #[prop_or_default]
    pub rangestart: Option<NaiveDate>,
    /// The currently selected value
    #[prop_or_default]
    pub value: Option<NaiveDate>,
    /// The day to start the week with
    #[prop_or(Weekday::Mon)]
    pub weekday_start: Weekday,
}

/// Date picker component
///
/// > A *date picker* helps users enter or select a specific date from a calendar.
///
/// See: <https://www.patternfly.org/components/date-and-time/date-picker>
///
/// Invalid non-empty input is marked as an error. Empty input is valid.
/// Only valid user selections emit `onchange`; clearing does not emit a date.
/// Changes to `value` reset the displayed input.
///
/// ## Properties
///
/// Defined by [`DatePickerProperties`].
#[function_component(DatePicker)]
pub fn date_picker(props: &DatePickerProperties) -> Html {
    let value = use_state_eq(|| props.value);
    let string_value =
        use_state_eq(|| props.value.map(|date| date.to_string()).unwrap_or_default());

    let error_id = use_memo((), |_| {
        format!("date-picker-error-{}", uuid::Uuid::new_v4())
    });
    let parsed = parse_date(&string_value);
    let invalid = parsed.is_err();

    {
        let value = value.clone();
        let string_value = string_value.clone();
        use_effect_with(props.value, move |date| {
            value.set(*date);
            string_value.set(date.map(|date| date.to_string()).unwrap_or_default());
        });
    }

    let callback_change_value = {
        use_callback(
            (value.clone(), string_value.clone(), props.onchange.clone()),
            |new_date: NaiveDate, (value, string_value, onchange)| {
                value.set(Some(new_date));
                string_value.set(new_date.to_string());
                onchange.emit(new_date);
            },
        )
    };

    let target = html! {
        <button
            class="pf-v6-c-button pf-m-control"
            type="button"
            aria-label="Toggle date picker"
            disabled={props.disabled}
        >
            <i class="fas fa-calendar-alt" aria-hidden="true" style="line-height: 1.5" />
        </button>
    };

    let body = html_nested! (
        // We need to extract the body component, as we need the PopoverContext using use_context.
        // However, that only works if the call of use_context comes from a component wrapped by
        // Popover.
        <PopoverBody>
            <Body
                date={value.unwrap_or_else(|| Local::now().date_naive())}
                weekday_start={props.weekday_start}
                rangestart={props.rangestart}
                onchange={callback_change_value}
            />
        </PopoverBody>
    );

    let input_change = use_callback(
        (value.clone(), string_value.clone(), props.onchange.clone()),
        |text: String, (value, string_value, onchange)| {
            let date = parse_date(&text).ok().flatten();
            string_value.set(text);
            value.set(date);
            if let Some(date) = date {
                onchange.emit(date);
            }
        },
    );

    // The text input
    let input = html! (
        <TextInput
            onchange={input_change}
            state={if invalid { InputState::Error } else { InputState::Default }}
            aria_describedby={invalid.then(|| (*error_id).clone())}
            disabled={props.disabled}
            value={(*string_value).clone()}
            placeholder={props.placeholder.clone()}
        />
    );

    html! {
        <div class="pf-v6-c-date-picker">
            <div class="pf-v6-c-date-picker__input">
                <InputGroup>
                    <InputGroupItem>{ input }</InputGroupItem>
                    <InputGroupItem>
                        <Popover {target} {body} no_padding=true no_close=true width_auto=true />
                    </InputGroupItem>
                </InputGroup>
            </div>
            <HelperText id={(*error_id).clone()} live_region=true>
                { for invalid.then(|| html_nested! {
                    <HelperTextItem variant={HelperTextItemVariant::Error}>
                        {"Enter a valid date in YYYY-MM-DD format."}
                    </HelperTextItem>
                }) }
            </HelperText>
        </div>
    }
}

fn parse_date(text: &str) -> Result<Option<NaiveDate>, chrono::ParseError> {
    if text.is_empty() {
        Ok(None)
    } else {
        NaiveDate::parse_from_str(text, "%Y-%m-%d").map(Some)
    }
}

/// the body component, using the popover context
#[derive(PartialEq, Properties)]
struct BodyProperties {
    date: NaiveDate,
    weekday_start: Weekday,
    rangestart: Option<NaiveDate>,
    onchange: Callback<NaiveDate>,
}

#[function_component(Body)]
fn body(props: &BodyProperties) -> Html {
    let context = use_context::<PopoverContext>();
    let onchange = use_callback(
        (context, props.onchange.clone()),
        |value, (context, callback)| {
            if let Some(context) = context {
                context.close();
            }
            callback.emit(value);
        },
    );

    html!(
        <CalendarView
            date={props.date}
            weekday_start={props.weekday_start}
            rangestart={props.rangestart}
            {onchange}
        />
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_dates_and_empty_input() {
        assert_eq!(
            parse_date("2024-02-29"),
            Ok(NaiveDate::from_ymd_opt(2024, 2, 29))
        );
        assert_eq!(parse_date(""), Ok(None));
    }

    #[test]
    fn rejects_invalid_dates_and_incomplete_input() {
        for text in [
            "2023-02-29",
            "2024-04-31",
            "2024-13-01",
            "2024-02-",
            "invalid",
            " ",
        ] {
            assert!(parse_date(text).is_err(), "{text}");
        }
    }
}
