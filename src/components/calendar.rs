use crate::prelude::*;
use web_sys::HtmlElement;
use yew::html::IntoPropValue;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use chrono::*;
use num_traits::cast::FromPrimitive;
use std::str::FromStr;

#[derive(Clone, PartialEq, Properties)]
pub struct CalendarMonthProperties {
    #[prop_or(Local::now().date_naive())]
    pub date: NaiveDate,
    #[prop_or_default]
    pub onchange: Callback<NaiveDate>,
}

//pub struct CalendarView;

fn build_calendar(date: NaiveDate) -> Vec<Vec<NaiveDate>> {
    let mut ret : Vec<Vec<NaiveDate>>= Vec::new();
    let month = date.month();
    let mut first_date = date.with_day(1).unwrap();
    let mut tmp_date = first_date.week(Weekday::Mon).first_day();
    let mut tmp : Vec<NaiveDate> = Vec::new();

    // just insert the first week before iterate on all month and stop at the new one
    while first_date.week(Weekday::Mon).days().contains(&tmp_date) {
        tmp.push(tmp_date.clone());
        tmp_date = tmp_date + Days::new(1);
    }

    ret.push(tmp);

    first_date = first_date.week(Weekday::Mon).last_day() + Days::new(1);

    while first_date.month() == month {
        tmp = Vec::new();
        while first_date.week(Weekday::Mon).days().contains(&tmp_date) {
            tmp.push(tmp_date.clone());
            tmp_date = tmp_date + Days::new(1);
        }

        first_date = first_date.week(Weekday::Mon).last_day() + Days::new(1);
        ret.push(tmp);
    }

    ret
}

fn tmp(date: NaiveDate) -> String {
    String::from(Month::from_u32(date.month()).unwrap().name())
}

#[function_component(CalendarView)]
pub fn calendar(props: &CalendarMonthProperties) -> Html {
    //let date: DateTime<Local> = use_state(Local::now());
    let date = use_state_eq(|| props.date);
    let weeks = build_calendar(*date);
    let month = tmp(*date);

    let my_onchange = props.onchange.clone();

    let callback_test = {
        let date = date.clone();
        let onchange = my_onchange.clone();
        Callback::from(move |mois: String| {
            let new = NaiveDate::from_ymd_opt(date.year(), mois.parse::<Month>().unwrap().number_from_month(), date.day()).unwrap();
            date.set(new);
            onchange.emit(new);
        })
    };

    let callback_second_test = {
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

    html!{
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
                            <Select<String> initial_selection={Vec::from([month])} variant={SelectVariant::Single(callback_test)}>
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
                            <TextInput value={date.year().to_string()} r#type={TextInputType::Number} onchange={callback_second_test}>
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
                                    move |day: u32| {
                                        Callback::from(move |_| {
                                            let new = NaiveDate::from_ymd_opt(date.year(), date.month(), day).unwrap();
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
                                            onclick={callback_date(day.day())}
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
