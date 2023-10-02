use crate::prelude::{CalendarView, InputGroup, InputGroupItem, Popover, PopoverBody, TextInput};
use chrono::{NaiveDate, Weekday};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct DatePickerProperties {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub onchange: Callback<NaiveDate>,
    #[prop_or(String::from("YYYY-MM-DD"))]
    pub placeholder: String,
    #[prop_or_default]
    pub rangestart: Option<NaiveDate>,
    #[prop_or_default]
    pub value: Option<NaiveDate>,
    #[prop_or(Weekday::Mon)]
    pub weekday_start: Weekday,
}

#[function_component(DatePicker)]
pub fn date_picker(props: &DatePickerProperties) -> Html {
    let value = use_state_eq(|| props.value.clone());
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
            class="pf-v5-c-button pf-m-control"
            type="button"
            aria-label="Toggle date picker"
            disabled={props.disabled}
        >
        <i class="fas fa-calendar-alt" aria-hidden="true"></i>
        </button>
    };

    let body = html_nested! {
        <PopoverBody>
            if let Some(d) = *value {
                <CalendarView
                    date={d}
                    weekday_start={props.weekday_start}
                    rangestart={props.rangestart}
                    onchange={callback_change_value}
                />
            } else {
                <CalendarView
                    weekday_start={props.weekday_start}
                    rangestart={props.rangestart}
                    onchange={callback_change_value}
                />
            }
        </PopoverBody>
    };

    let input_change = use_callback(string_value.clone(), |value, string_value| {
        string_value.set(value);
    });
    {
        let onchange = props.onchange.clone();
        use_effect_with(
            ((*string_value).clone(), value.clone()),
            move |(string_value, value)| {
                let new = match NaiveDate::parse_from_str(&string_value, "%Y-%m-%d") {
                    Ok(v) => Some(v),
                    Err(_err) => None,
                };

                value.set(new);
                if let Some(new) = new {
                    onchange.emit(new);
                }
            },
        );
    }

    let input = html! (
        <TextInput
            onchange={input_change}
            disabled={props.disabled}
            value={(*string_value).clone()}
            placeholder={props.placeholder.clone()}
        />
    );

    html! {
        <div class="pf-v5-c-date-picker">
            <div class="pf-v5-c-date-picker__input">
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
