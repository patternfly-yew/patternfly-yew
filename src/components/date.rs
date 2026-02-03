use crate::prelude::{
    CalendarView, InputGroup, InputGroupItem, Popover, PopoverBody, PopoverContext, TextInput,
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
/// ## Properties
///
/// Defined by [`DatePickerProperties`].
#[function_component(DatePicker)]
pub fn date_picker(props: &DatePickerProperties) -> Html {
    let value = use_state_eq(|| props.value);
    let string_value =
        use_state_eq(|| props.value.map(|date| date.to_string()).unwrap_or_default());

    let callback_change_value = {
        let onchange = props.onchange.clone();
        use_callback(
            (value.clone(), string_value.clone()),
            move |new_date: NaiveDate, (value, string_value)| {
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
        <i class="fas fa-calendar-alt" aria-hidden="true" style="line-height: 1.5;"></i>
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

    // short circuit the text input to the text value
    let input_change = use_callback(string_value.clone(), |value, string_value| {
        string_value.set(value);
    });
    // when the text value changes, try updating the date value
    {
        let onchange = props.onchange.clone();
        use_effect_with(
            ((*string_value).clone(), value.clone()),
            move |(string_value, value)| {
                // FIXME: should extract an "error" state from this
                let new = NaiveDate::parse_from_str(string_value, "%Y-%m-%d").ok();

                value.set(new);
                if let Some(new) = new {
                    onchange.emit(new);
                }
            },
        );
    }

    // The text input
    let input = html! (
        <TextInput
            onchange={input_change}
            disabled={props.disabled}
            value={(*string_value).clone()}
            placeholder={props.placeholder.clone()}
        />
    );

    html! {
        <div class="pf-v6-c-date-picker">
            <div class="pf-v6-c-date-picker__input">
                <InputGroup>
                    <InputGroupItem>
                        {input}
                    </InputGroupItem>
                    <InputGroupItem>
                        <Popover
                            {target} {body}
                            no_padding=true
                            no_close=true
                            width_auto=true
                        />
                    </InputGroupItem>
                </InputGroup>
            </div>
        </div>
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
