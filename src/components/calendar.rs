use crate::prelude::{
    Button, ButtonType, ButtonVariant, Icon, InputGroup, InputGroupItem, SimpleSelect, TextInput,
    TextInputType,
};
use jiff::{
    ToSpan,
    civil::{Date, Weekday},
};
use std::str::FromStr;

use yew::{
    Callback, Html, Properties, classes, function_component, html, use_callback, use_state_eq,
};

use super::select::SelectItemRenderer;

#[derive(Clone, PartialEq, Properties)]
pub struct CalendarMonthProperties {
    #[prop_or(today())]
    pub date: Date,
    #[prop_or_default]
    pub onchange: Callback<Date>,
    #[prop_or_default]
    pub rangestart: Option<Date>,
    #[prop_or(Weekday::Monday)]
    pub weekday_start: Weekday,
}

// Build a vec (month) which contains vecs (weeks) of a month with the first
// and last day of week, even if they aren't in the same month.
//
// The month is set by `date` and the first day of the week by `weekday_start`.
fn build_calendar(date: Date, weekday_start: Weekday) -> Vec<Vec<Option<Date>>> {
    let first = date.first_of_month();
    let leading = (first.weekday().to_monday_zero_offset() - weekday_start.to_monday_zero_offset())
        .rem_euclid(7) as i64;
    let cells = (leading + i64::from(date.days_in_month()) + 6) / 7 * 7;
    (0..cells / 7)
        .map(|week| {
            (0..7)
                .map(|day| first.checked_add((week * 7 + day - leading).days()).ok())
                .collect()
        })
        .collect()
}

/// Today's civil date in the local environment, without shipping a browser tzdb.
pub(super) fn today() -> Date {
    #[cfg(target_family = "wasm")]
    {
        let now = js_sys::Date::new_0();
        Date::new(
            now.get_full_year() as i16,
            (now.get_month() + 1) as i8,
            now.get_date() as i8,
        )
        .expect("current browser date is in Jiff's supported range")
    }
    #[cfg(not(target_family = "wasm"))]
    {
        jiff::ZonedDateTime::now().date()
    }
}

#[function_component(CalendarView)]
pub fn calendar(props: &CalendarMonthProperties) -> Html {
    // the date which is selected by user
    let date = use_state_eq(|| props.date);
    // the date which is showed when the user changes month or year without selecting a new date
    let show_date = use_state_eq(|| props.date);
    // an array which contains the week of the selected date
    let weeks = build_calendar(*show_date, props.weekday_start);
    let callback_month_select = use_callback(
        show_date.clone(),
        move |new_month: MonthLocal, show_date| {
            if let Ok(d) = Date::new(show_date.year(), new_month.0, show_date.day()) {
                show_date.set(d);
            }
        },
    );

    let callback_years = use_callback(show_date.clone(), move |new_year: String, show_date| {
        if let Ok(y) = i16::from_str(&new_year)
            && let Ok(d) = Date::new(y, show_date.month(), show_date.day())
        {
            show_date.set(d)
        }
    });

    let callback_prev = use_callback(show_date.clone(), move |_, show_date| {
        if let Ok(d) = show_date.checked_sub(1.month()) {
            show_date.set(d);
        }
    });

    let callback_next = use_callback(show_date.clone(), move |_, show_date| {
        if let Ok(d) = show_date.checked_add(1.month()) {
            show_date.set(d);
        }
    });

    html! {
        <div class="pf-v6-c-calendar-month">
            <div class="pf-v6-c-calendar-month__header">
                <div class="pf-v6-c-calendar-month__header-nav-control pf-m-prev-month">
                    <Button
                        variant={ButtonVariant::Plain}
                        aria_label="Previous month"
                        onclick={callback_prev}
                    >
                        { Icon::AngleLeft.as_html() }
                    </Button>
                </div>
                <InputGroup>
                    <InputGroupItem>
                        <div class="pf-v6-c-calendar-month__header-month">
                            <SimpleSelect<MonthLocal>
                                entries={(1..=12).map(MonthLocal).collect::<Vec<_>>()}
                                selected={MonthLocal(show_date.month())}
                                onselect={callback_month_select}
                            />
                        </div>
                    </InputGroupItem>
                    <InputGroupItem>
                        <div class="pf-v6-c-calendar-month__header-year">
                            <TextInput
                                value={show_date.year().to_string()}
                                r#type={TextInputType::Number}
                                onchange={callback_years}
                            />
                        </div>
                    </InputGroupItem>
                </InputGroup>
                <div class="pf-v6-c-calendar-month__header-nav-control pf-m-next-month">
                    <Button
                        variant={ButtonVariant::Plain}
                        aria_label="Next month"
                        onclick={callback_next}
                    >
                        { Icon::AngleRight.as_html() }
                    </Button>
                </div>
            </div>
            <table class="pf-v6-c-calendar-month__calendar">
                <thead class="pf-v6-c-calendar-month__days">
                    <tr class="pf-v6-c-calendar-month__days-row">
                        { (0..7).map(|offset| {
                            let weekday = Weekday::from_monday_zero_offset((props.weekday_start.to_monday_zero_offset() + offset) % 7).unwrap();
                            html!{
                                <th class="pf-v6-c-calendar-month__day">
                                    <span class="pf-v6-screen-reader">{weekday_name(weekday)}</span>
                                    <span aria-hidden="true">{weekday_name(weekday)}</span>
                                </th>
                            }
                        }).collect::<Html>() }
                    </tr>
                </thead>
                <tbody class="pf-v6-c-calendar-month__dates">
                    { weeks.into_iter().map(|week| {
                        html!{
                            <>
                            <tr class="pf-v6-c-calendar-month__dates-row">
                            {
                            week.into_iter().map(|day| {
                                let Some(day) = day else {
                                    return html! { <td class="pf-v6-c-calendar-month__dates-cell" /> };
                                };
                                let callback_date = {
                                    let date = date.clone();
                                    let show_date = show_date.clone();
                                    let onchange = props.onchange.clone();
                                    move |day: Date| {
                                        Callback::from(move |_| {
                                            let new = day;
                                            date.set(new);
                                            show_date.set(new);
                                            onchange.emit(new);
                                        })
                                    }
                                };

                                let mut classes = classes!("pf-v6-c-calendar-month__dates-cell");

                                if day == *date {
                                    classes.extend(classes!("pf-m-selected"));
                                }

                                if day.month() != show_date.month() {
                                    classes.extend(classes!("pf-m-adjacent-month"));
                                }

                                let before_range = if let Some(range_start) = props.rangestart {
                                    if day < range_start {
                                        classes.extend(classes!("pf-m-disabled"));
                                    }

                                    if day == range_start {
                                        classes.extend(classes!("pf-m-start-range"));
                                        classes.extend(classes!("pf-m-selected"));
                                    }

                                    if day >= range_start && day <= *date {
                                        classes.extend(classes!("pf-m-in-range"));
                                    }

                                    if day == *date {
                                        classes.extend(classes!("pf-m-end-range"));
                                    }

                                    day < range_start
                                } else { false };

                                html!{
                                    <>
                                    <td class={classes}>
                                        <Button
                                            class="pf-v6-c-calendar-month__date"
                                            r#type={ButtonType::Button}
                                            variant={if before_range {
                                                ButtonVariant::Plain
                                            } else {
                                                ButtonVariant::None
                                            }}
                                            onclick={callback_date(day)}
                                            disabled={before_range}
                                        >
                                        {day.day()}
                                        </Button>
                                    </td>
                                    </>
                                }
                            }).collect::<Html>()
                            }
                            </tr>
                            </>
                        }
                    }).collect::<Html>() }
                </tbody>
            </table>
        </div>
    }
}

/// A month number (1–12) rendered as a localized name.
#[derive(Clone, PartialEq, Eq)]
struct MonthLocal(i8);

impl SelectItemRenderer for MonthLocal {
    type Item = String;

    fn label(&self) -> Self::Item {
        let date = Date::new(2024, self.0, 1).unwrap();
        calendar_label(date, "month", "long", "%B")
    }
}

fn weekday_name(weekday: Weekday) -> String {
    let date = Date::new(2024, 1, 1)
        .unwrap()
        .checked_add(i64::from(weekday.to_monday_zero_offset()).days())
        .unwrap();
    calendar_label(date, "weekday", "short", "%a")
}

fn calendar_label(date: Date, field: &str, width: &str, fallback: &str) -> String {
    #[cfg(all(feature = "localization", target_family = "wasm"))]
    if let Some(label) =
        browser_calendar_label(date, field, width, &wasm_bindgen::JsValue::UNDEFINED)
    {
        return label;
    }
    let _ = (field, width);
    date.strftime(fallback).to_string()
}

#[cfg(all(feature = "localization", target_family = "wasm"))]
fn browser_calendar_label(
    date: Date,
    field: &str,
    width: &str,
    locale: &wasm_bindgen::JsValue,
) -> Option<String> {
    use wasm_bindgen::{JsCast, JsValue};

    // Use a fixed UTC date so DST and the local offset cannot change the label.
    let js_date = js_sys::Date::new(&JsValue::from_str(&format!("{date}T00:00:00Z")));
    let options = js_sys::Object::new();
    let set =
        |key: &str, value: &str| js_sys::Reflect::set(&options, &key.into(), &value.into()).ok();
    set(field, width);
    set("timeZone", "UTC");
    set("calendar", "gregory");
    js_sys::Reflect::get(&js_date, &"toLocaleDateString".into())
        .ok()
        .and_then(|f| f.dyn_into::<js_sys::Function>().ok())
        .and_then(|f| f.call2(&js_date, locale, &options).ok())
        .and_then(|label| label.as_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg_attr(target_family = "wasm", wasm_bindgen_test::wasm_bindgen_test)]
    #[cfg_attr(not(target_family = "wasm"), test)]
    fn calendar_includes_adjacent_dates_and_leap_day() {
        let weeks = build_calendar(Date::new(2024, 2, 15).unwrap(), Weekday::Monday);
        assert_eq!(weeks.len(), 5);
        assert!(weeks.iter().all(|week| week.len() == 7));
        assert_eq!(weeks[0][0], Some(Date::new(2024, 1, 29).unwrap()));
        assert_eq!(weeks[4][3], Some(Date::new(2024, 2, 29).unwrap()));
        assert_eq!(weeks[4][6], Some(Date::new(2024, 3, 3).unwrap()));

        let weeks = build_calendar(Date::new(2023, 2, 15).unwrap(), Weekday::Sunday);
        assert_eq!(weeks[0][0], Some(Date::new(2023, 1, 29).unwrap()));
        assert_eq!(weeks[4][2], Some(Date::new(2023, 2, 28).unwrap()));
        assert_eq!(weeks[4][3], Some(Date::new(2023, 3, 1).unwrap()));
    }

    #[cfg_attr(target_family = "wasm", wasm_bindgen_test::wasm_bindgen_test)]
    #[cfg_attr(not(target_family = "wasm"), test)]
    fn calendar_handles_year_boundaries_and_date_limits() {
        for date in [Date::MIN, Date::MAX, Date::new(2024, 12, 31).unwrap()] {
            for offset in 0..7 {
                let start = Weekday::from_monday_zero_offset(offset).unwrap();
                let weeks = build_calendar(date, start);
                assert!(weeks.iter().all(|week| week.len() == 7));
                let days: Vec<_> = weeks.iter().flatten().flatten().copied().collect();
                assert!(days.contains(&date));
                assert_eq!(
                    days.iter()
                        .filter(|day| day.month() == date.month())
                        .count(),
                    date.days_in_month() as usize
                );
                assert!(
                    days.windows(2)
                        .all(|pair| pair[0].tomorrow().unwrap() == pair[1])
                );
            }
        }
    }

    #[cfg(all(feature = "localization", target_family = "wasm"))]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn localizes_calendar_labels_without_shifting_the_day() {
        let date = Date::new(2024, 1, 1).unwrap(); // Monday
        let locale = wasm_bindgen::JsValue::from_str("fr-FR");
        assert_eq!(
            browser_calendar_label(date, "month", "long", &locale).as_deref(),
            Some("janvier")
        );
        assert_eq!(
            browser_calendar_label(date, "weekday", "short", &locale).as_deref(),
            Some("lun.")
        );
    }
}
