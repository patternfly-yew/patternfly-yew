use crate::prelude::*;
use chrono::*;
use num_traits::cast::FromPrimitive;
use std::str::FromStr;
use web_sys::HtmlElement;
use yew::html::IntoPropValue;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

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
    const one_day: Days = Days::new(1);
    let mut ret: Vec<Vec<NaiveDate>> = Vec::new();
    // first day of the week. It's initialized first at the first day of the month
    let mut first_day = date.with_day(1).unwrap();
    let mut day = first_day.week(weekday_start).first_day();
    let mut week: Vec<NaiveDate>;

    while first_day.month() == date.month() {
        week = Vec::new();
        while first_day.week(weekday_start).days().contains(&day) {
            week.push(day.clone());
            day = day + one_day;
        }

        first_day = first_day.week(weekday_start).last_day() + one_day;
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

    let callback_month_select = {
        let show_date = show_date.clone();
        let month = month.clone();
        use_callback(
            move |new_month: String, show_date| {
                if let Ok(m) = new_month.parse::<Month>() {
                    if let Some(d) = NaiveDate::from_ymd_opt(
                        show_date.year(),
                        m.number_from_month(),
                        show_date.day(),
                    ) {
                        show_date.set(d);
                        month.set(m);
                    }
                }
            },
            show_date,
        )
    };

    let callback_years = {
        let show_date = show_date.clone();
        use_callback(
            move |new_year: String, show_date| {
                if let Ok(y) = i32::from_str(&new_year) {
                    if let Some(d) = NaiveDate::from_ymd_opt(y, show_date.month(), show_date.day())
                    {
                        show_date.set(d)
                    }
                }
            },
            show_date,
        )
    };

    let callback_prev = {
        let show_date = show_date.clone();
        let month = month.clone();
        use_callback(
            move |_, show_date| {
                if let Some(d) = show_date.checked_sub_months(Months::new(1)) {
                    show_date.set(d);
                    month.set(month.pred());
                }
            },
            show_date,
        )
    };

    let callback_next = {
        let show_date = show_date.clone();
        let month = month.clone();
        use_callback(
            move |_, show_date| {
                if let Some(d) = show_date.checked_add_months(Months::new(1)) {
                    show_date.set(d);
                    month.set(month.succ());
                }
            },
            show_date,
        )
    };

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
                            <SimpleSelect<String>
                                entries={vec![
                                    String::from(Month::January.name()),
                                    String::from(Month::February.name()),
                                    String::from(Month::March.name()),
                                    String::from(Month::April.name()),
                                    String::from(Month::May.name()),
                                    String::from(Month::June.name()),
                                    String::from(Month::July.name()),
                                    String::from(Month::August.name()),
                                    String::from(Month::September.name()),
                                    String::from(Month::October.name()),
                                    String::from(Month::November.name()),
                                    String::from(Month::December.name())
                                ]}
                                selected={String::from(month.name())}
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
                                    <span class="pf-v5-screen-reader">{day.weekday().to_string()}</span>
                                    <span aria-hidden="true">{day.weekday().to_string()}</span>
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
