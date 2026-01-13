use crate::prelude::{
    Button, ButtonType, ButtonVariant, Icon, InputGroup, InputGroupItem, SimpleSelect, TextInput,
    TextInputType,
};
use chrono::{Datelike, Days, Local, Month, Months, NaiveDate, Weekday};
use num_traits::cast::FromPrimitive;
use std::str::FromStr;

use yew::{
    Callback, Html, Properties, classes, function_component, html, use_callback, use_state_eq,
};

use super::select::SelectItemRenderer;

#[derive(Clone, PartialEq, Properties)]
pub struct CalendarMonthProperties {
    #[prop_or(Local::now().date_naive())]
    pub date: NaiveDate,
    #[prop_or_default]
    pub onchange: Callback<NaiveDate>,
    #[prop_or_default]
    pub rangestart: Option<NaiveDate>,
    #[prop_or(Weekday::Mon)]
    pub weekday_start: Weekday,
}

// Build a vec (month) which contains vecs (weeks) of a month with the first
// and last day of week, even if they aren't in the same month.
//
// The month is set by `date` and the first day of the week by `weekday_start`.
fn build_calendar(date: NaiveDate, weekday_start: Weekday) -> Vec<Vec<NaiveDate>> {
    const ONE_DAY: Days = Days::new(1);
    let mut ret: Vec<Vec<NaiveDate>> = Vec::new();
    // first day of the week. It's initialized first at the first day of the month
    let mut first_day = date.with_day(1).unwrap();
    let mut day = first_day.week(weekday_start).first_day();
    let mut week: Vec<NaiveDate>;

    while first_day.month() == date.month() {
        week = Vec::new();
        while first_day.week(weekday_start).days().contains(&day) {
            week.push(day);
            day = day + ONE_DAY;
        }

        first_day = first_day.week(weekday_start).last_day() + ONE_DAY;
        ret.push(week);
    }

    ret
}

#[function_component(CalendarView)]
pub fn calendar(props: &CalendarMonthProperties) -> Html {
    // the date which is selected by user
    let date = use_state_eq(|| props.date);
    // the date which is showed when the user changes month or year without selecting a new date
    let show_date = use_state_eq(|| props.date);
    // an array which contains the week of the selected date
    let weeks = build_calendar(*show_date, props.weekday_start);
    // the month of the selected date, used for selector
    let month = use_state_eq(|| Month::from_u32(props.date.month()).unwrap());

    let callback_month_select = use_callback(
        (show_date.clone(), month.clone()),
        move |new_month: MonthLocal, (show_date, month)| {
            if let Some(d) = NaiveDate::from_ymd_opt(
                show_date.year(),
                new_month.0.number_from_month(),
                show_date.day(),
            ) {
                show_date.set(d);
                month.set(new_month.0);
            }
        },
    );

    let callback_years = use_callback(show_date.clone(), move |new_year: String, show_date| {
        if let Ok(y) = i32::from_str(&new_year) {
            if let Some(d) = NaiveDate::from_ymd_opt(y, show_date.month(), show_date.day()) {
                show_date.set(d)
            }
        }
    });

    let callback_prev = use_callback(
        (show_date.clone(), month.clone()),
        move |_, (show_date, month)| {
            if let Some(d) = show_date.checked_sub_months(Months::new(1)) {
                show_date.set(d);
                month.set(month.pred());
            }
        },
    );

    let callback_next = use_callback(
        (show_date.clone(), month.clone()),
        move |_, (show_date, month)| {
            if let Some(d) = show_date.checked_add_months(Months::new(1)) {
                show_date.set(d);
                month.set(month.succ());
            }
        },
    );

    html! {
        <div class="pf-v5-c-calendar-month">
            <div class="pf-v5-c-calendar-month__header">
                <div class="pf-v5-c-calendar-month__header-nav-control pf-m-prev-month">
                    <Button
                        variant={ButtonVariant::Plain}
                        aria_label="Previous month"
                        onclick={callback_prev}
                    >
                    {Icon::AngleLeft.as_html()}
                    </Button>
                </div>
                <InputGroup>
                    <InputGroupItem>
                        <div class="pf-v5-c-calendar-month__header-month">
                            <SimpleSelect<MonthLocal>
                                entries={vec![
                                    MonthLocal(Month::January),
                                    MonthLocal(Month::February),
                                    MonthLocal(Month::March),
                                    MonthLocal(Month::April),
                                    MonthLocal(Month::May),
                                    MonthLocal(Month::June),
                                    MonthLocal(Month::July),
                                    MonthLocal(Month::August),
                                    MonthLocal(Month::September),
                                    MonthLocal(Month::October),
                                    MonthLocal(Month::November),
                                    MonthLocal(Month::December)
                                ]}
                                selected={MonthLocal(*month)}
                                onselect={callback_month_select}
                            />
                        </div>
                    </InputGroupItem>
                    <InputGroupItem>
                        <div class="pf-v5-c-calendar-month__header-year">
                            <TextInput
                                value={show_date.year().to_string()}
                                r#type={TextInputType::Number}
                                onchange={callback_years}
                            >
                            </TextInput>
                        </div>
                    </InputGroupItem>
                </InputGroup>
                <div class="pf-v5-c-calendar-month__header-nav-control pf-m-next-month">
                    <Button
                        variant={ButtonVariant::Plain}
                        aria_label="Next month"
                        onclick={callback_next}
                    >
                    {Icon::AngleRight.as_html()}
                    </Button>
                </div>
            </div>
            <table class="pf-v5-c-calendar-month__calendar">
                <thead class="pf-v5-c-calendar-month__days">
                    <tr class="pf-v5-c-calendar-month__days-row">
                    {
                        weeks[0].clone().into_iter().map(|day| {
                            html!{
                                <th class="pf-v5-c-calendar-month__day">
                                    <span class="pf-v5-screen-reader">{weekday_name(day.weekday())}</span>
                                    <span aria-hidden="true">{weekday_name(day.weekday())}</span>
                                </th>
                            }
                        }).collect::<Html>()
                    }
                    </tr>
                </thead>
                <tbody class="pf-v5-c-calendar-month__dates">
                {
                    weeks.into_iter().map(|week| {
                        html!{
                            <>
                            <tr class="pf-v5-c-calendar-month__dates-row">
                            {
                            week.into_iter().map(|day| {
                                let callback_date = {
                                    let date = date.clone();
                                    let month = month.clone();
                                    let show_date = show_date.clone();
                                    let onchange = props.onchange.clone();
                                    move |day: NaiveDate| {
                                        Callback::from(move |_| {
                                            let new = NaiveDate::from_ymd_opt(day.year(), day.month(), day.day()).unwrap();
                                            date.set(new);
                                            show_date.set(new);
                                            month.set(Month::from_u32(day.month()).unwrap());
                                            onchange.emit(new);
                                        })
                                    }
                                };

                                let mut classes = classes!("pf-v5-c-calendar-month__dates-cell");

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
                                            class="pf-v5-c-calendar-month__date"
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
                    }).collect::<Html>()
                }
                </tbody>
            </table>
        </div>
    }
}

/// A wrapper around [`chrono::Month`] to extend it
#[derive(Clone, PartialEq, Eq)]
struct MonthLocal(Month);

impl SelectItemRenderer for MonthLocal {
    type Item = String;

    #[cfg(feature = "localization")]
    fn label(&self) -> Self::Item {
        self.0.localized_name()
    }

    #[cfg(not(feature = "localization"))]
    fn label(&self) -> Self::Item {
        self.0.name().to_string()
    }
}

#[cfg(feature = "localization")]
trait Localized {
    fn localized_name(&self) -> String;
}

#[cfg(feature = "localization")]
impl Localized for Month {
    /// Convert to text in the current system language
    fn localized_name(&self) -> String {
        // Build a dummy NaiveDate with month whose name I'm interested in
        let date = NaiveDate::from_ymd_opt(2024, self.number_from_month(), 1).unwrap();

        // Get a localized full month name
        date.format_localized("%B", current_locale()).to_string()
    }
}

#[cfg(feature = "localization")]
fn weekday_name(weekday: Weekday) -> String {
    localized_weekday_name(weekday)
}

#[cfg(not(feature = "localization"))]
fn weekday_name(weekday: Weekday) -> String {
    weekday.to_string()
}

#[cfg(feature = "localization")]
fn localized_weekday_name(weekday: Weekday) -> String {
    // Get today NaiveDateTime
    let today = chrono::Local::now().naive_local();

    // Calculate the distance in days between today and the next 'weekday'
    let days_until_weekday = (7 + weekday.num_days_from_monday() as i64
        - today.weekday().num_days_from_monday() as i64)
        % 7;

    // Calculate the date of the next 'weekday'
    let one_day = today + chrono::Duration::days(days_until_weekday);

    // Get a localized 'weekday' short name
    one_day
        .date()
        .format_localized("%a", current_locale())
        .to_string()
}

#[cfg(feature = "localization")]
static CURRENT_LOCALE_CELL: std::sync::OnceLock<chrono::Locale> = std::sync::OnceLock::new();

#[cfg(feature = "localization")]
fn current_locale() -> chrono::Locale {
    CURRENT_LOCALE_CELL
        .get_or_init(|| {
            // Get the current system locale text representation
            let current_locale = sys_locale::get_locale().unwrap_or_else(|| String::from("en-US"));

            // Convert the locale representation to snake case
            let current_locale_snake_case = current_locale
                .as_str()
                .split('.')
                .next()
                .map(|s| s.replace('-', "_"))
                .unwrap_or("en_US".to_string());

            // Build the chono::Locale from locale text represantation
            chrono::Locale::try_from(current_locale_snake_case.as_str())
                .unwrap_or(chrono::Locale::POSIX)
        })
        .to_owned()
}
