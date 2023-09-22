use crate::prelude::{CalendarView, InputGroup, InputGroupItem, Popover, PopoverBody, TextInput};
use chrono::{NaiveDate, Weekday};
use std::ops::Deref;
use yew::{
    function_component, html, html_nested, use_callback, use_state_eq, Callback, Html, Properties,
};

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
    pub value: Option<String>,
    #[prop_or(Weekday::Mon)]
    pub weekday_start: Weekday,
}

#[function_component(DatePicker)]
pub fn date_picker(props: &DatePickerProperties) -> Html {
    let value = use_state_eq(|| props.value.clone());
    let date = {
        let value = value.clone();
        if let Some(s) = value.deref() {
            if let Ok(d) = s.parse::<NaiveDate>() {
                Some(d)
            } else {
                None
            }
        } else {
            None
        }
    };

    let callback_change_value = {
        let value = value.clone();
        let onchange = props.onchange.clone();
        use_callback(
            move |new_date: NaiveDate, value| {
                value.set(Some(new_date.to_string()));
                onchange.emit(new_date);
            },
            value,
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
            if let Some(d) = date {
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

    let input = {
        let value = value.clone();
        if let Some(value) = value.deref() {
            html! {
                <TextInput
                    disabled={props.disabled}
                value={String::from(value)}
                />
            }
        } else {
            html! {
                <TextInput
                    disabled={props.disabled}
                placeholder={props.placeholder.clone()}
                />
            }
        }
    };

    html! {
        <div class="pf-v5-c-date-picker">
            <div class="pf-v5-c-date-picker__input">
                <InputGroup>
                    <InputGroupItem>
                        {input}
                    </InputGroupItem>
                    <InputGroupItem>
                        <Popover {target} {body} />
                    </InputGroupItem>
                </InputGroup>
            </div>
        </div>
    }
}
