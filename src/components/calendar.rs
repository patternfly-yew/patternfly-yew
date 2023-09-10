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

fn tmp(date: NaiveDate) -> String {
    String::from(Month::from_u32(date.month()).unwrap().name())
}

#[function_component(CalendarView)]
pub fn calendar(props: &CalendarMonthProperties) -> Html {
    let date = use_state_eq(|| props.date);
    let weeks = build_calendar(*date, props.weekday_start);
    let month = tmp(*date);

    let my_onchange = props.onchange.clone();

    let callback_month_select = {
        let date = date.clone();
        let onchange = my_onchange.clone();
        Callback::from(move |month: String| {
            let new = NaiveDate::from_ymd_opt(
                date.year(),
                month.parse::<Month>().unwrap().number_from_month(),
                date.day(),
            )
            .unwrap();
            date.set(new);
            onchange.emit(new);
        })
    };

    let callback_years = {
        let date = date.clone();
        let onchange = my_onchange.clone();
        Callback::from(move |year: String| {
            if let Ok(y) = i32::from_str(&year) {
                if let Some(d) = NaiveDate::from_ymd_opt(y, date.month(), date.day()) {
                    date.set(d);
                    onchange.emit(d);
                }
            }
        })
    };

    let callback_prev = {
        let date = date.clone();
        let onchange = my_onchange.clone();
        Callback::from(move |_| {
            let new = *date - Months::new(1);
            date.set(new);
            onchange.emit(new);
        })
    };

    let callback_next = {
        let date = date.clone();
        let onchange = my_onchange.clone();
        Callback::from(move |_| {
            let new = *date + Months::new(1);
            date.set(new);
            onchange.emit(new);
        })
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
                            <Select<String>
                                initial_selection={Vec::from([month])}
                                variant={SelectVariant::Single(callback_month_select)}
                            >
                                <SelectOption<String>  value={String::from(Month::January.name())} />
                                <SelectOption<String>  value={String::from(Month::February.name())} />
                                <SelectOption<String>  value={String::from(Month::March.name())} />
                                <SelectOption<String>  value={String::from(Month::April.name())} />
                                <SelectOption<String>  value={String::from(Month::May.name())} />
                                <SelectOption<String>  value={String::from(Month::June.name())} />
                                <SelectOption<String>  value={String::from(Month::July.name())} />
                                <SelectOption<String>  value={String::from(Month::August.name())} />
                                <SelectOption<String>  value={String::from(Month::September.name())} />
                                <SelectOption<String>  value={String::from(Month::October.name())} />
                                <SelectOption<String>  value={String::from(Month::November.name())} />
                                <SelectOption<String>  value={String::from(Month::December.name())} />
                            </Select<String>>
                        </div>
                    </InputGroupItem>
                    <InputGroupItem>
                        <div class="pf-v5-c-calendar-month__header-year">
                            <TextInput
                                value={date.year().to_string()}
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
                        weeks[0].clone().into_iter().enumerate().map(|(id, day)| {
                            html!{
                                <th class="pf-v5-c-calendar-month__day">
                                    <span class="pf-v5-screen-reader">{day.weekday().to_string()}</span>
                                    <span aria-hidden="true">{day.weekday().to_string()}</span>
                                    //<span aria-hidden>{weekdayFormat(date)}</span>
                                </th>
                            }
                        }).collect::<Html>()
                    }
                    </tr>
                </thead>
                <tbody class="pf-v5-c-calendar-month__dates">
                {
                    weeks.into_iter().enumerate().map(|(id, week)| {
                        html!{
                            <>
                            <tr key={id} class="pf-v5-c-calendar-month__dates-row">
                            {
                            week.into_iter().enumerate().map(|(jid, day)| {
                                let callback_date = {
                                    let date = date.clone();
                                    let onchange = props.onchange.clone();
                                    move |day: NaiveDate| {
                                        Callback::from(move |_| {
                                            let new = NaiveDate::from_ymd_opt(day.year(), day.month(), day.day()).unwrap();
                                            date.set(new);
                                            onchange.emit(new);
                                        })}
                                };

                                let mut classes = classes!("pf-v5-c-calendar-month__dates-cell");
                                if day == *date {
                                    classes.extend(classes!("pf-m-selected"));
                                }

                                if day.month() != date.month() {
                                    classes.extend(classes!("pf-m-adjacent-month"));
                                }

                                html!{
                                    <>
                                    <td class={classes}>
                                        <Button
                                            class="pf-v5-c-calendar-month__date"
                                            r#type={ButtonType::Button}
                                            onclick={callback_date(day)}
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
