//! Timestamp

use crate::prelude::{Tooltip, TooltipPosition};
use jiff::fmt::temporal::DateTimePrinter;
use wasm_bindgen::{JsCast, JsValue};
use yew::prelude::*;

/// Date or time format of a [`Timestamp`]
///
/// Maps to the `dateStyle` / `timeStyle` options of `Intl.DateTimeFormat`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimestampFormat {
    Full,
    Long,
    Medium,
    Short,
}

impl TimestampFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::Long => "long",
            Self::Medium => "medium",
            Self::Short => "short",
        }
    }
}

/// Tooltip of a [`Timestamp`]
#[derive(Clone, Debug, PartialEq)]
pub enum TimestampTooltip {
    /// Show the date as a UTC string, using the timestamp's date format.
    Default {
        /// Suffix of the tooltip content, defaults to `UTC`.
        suffix: Option<AttrValue>,
    },
    /// Show custom content.
    Custom(Html),
}

impl TimestampTooltip {
    /// A default tooltip, with the default suffix.
    pub fn default_utc() -> Self {
        Self::Default { suffix: None }
    }
}

/// Properties for [`Timestamp`]
#[derive(Clone, PartialEq, Properties)]
pub struct TimestampProperties {
    /// Custom content, replacing the default formatted date.
    ///
    /// The format properties don't affect it, but still apply to the default tooltip.
    #[prop_or_default]
    pub children: Html,

    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Additional classes added to the outer `span`.
    #[prop_or_default]
    pub class: Classes,

    /// The date to display, also used for the `datetime` attribute. Defaults to "now".
    #[prop_or_default]
    pub date: Option<jiff::Timestamp>,

    /// Date format of the timestamp and the default tooltip.
    #[prop_or_default]
    pub date_format: Option<TimestampFormat>,

    /// Time format of the timestamp and the default tooltip.
    #[prop_or_default]
    pub time_format: Option<TimestampFormat>,

    /// Custom format, as `Intl.DateTimeFormat` options (e.g. `("weekday", "long")`).
    ///
    /// Overrides `date_format` and `time_format`.
    #[prop_or_default]
    pub custom_format: Option<Vec<(AttrValue, AttrValue)>>,

    /// Text appended to the displayed content, usually a time zone.
    #[prop_or_default]
    pub display_suffix: Option<AttrValue>,

    /// Display the time in 12-hour (`true`) or 24-hour (`false`) format.
    ///
    /// When not set, the locale decides.
    #[prop_or_default]
    pub is_12_hour: Option<bool>,

    /// Locale of the displayed content (e.g. `en-GB`). Defaults to the browser's locale.
    #[prop_or_default]
    pub locale: Option<AttrValue>,

    /// Display the default content in UTC, rather than in the local time zone.
    #[prop_or_default]
    pub display_utc: bool,

    /// Adds a tooltip to the timestamp.
    #[prop_or_default]
    pub tooltip: Option<TimestampTooltip>,

    /// Position of the tooltip.
    #[prop_or(TooltipPosition::Top)]
    pub tooltip_position: TooltipPosition,
}

/// Timestamp component
///
/// > A **timestamp** is a consistently formatted visual that displays date and time values.
///
/// See: <https://www.patternfly.org/components/timestamp>
///
/// The date is formatted by the browser (`Date.prototype.toLocaleString`).
///
/// ## Properties
///
/// Defined by [`TimestampProperties`].
///
/// ## Example
///
/// ```rust
/// use patternfly_yew::prelude::*;
/// use yew::prelude::*;
///
/// #[function_component(Example)]
/// fn example() -> Html {
///     html!(
///         <Timestamp
///             date_format={TimestampFormat::Medium}
///             time_format={TimestampFormat::Short}
///             tooltip={TimestampTooltip::default_utc()}
///         />
///     )
/// }
/// ```
#[function_component(Timestamp)]
pub fn timestamp(props: &TimestampProperties) -> Html {
    let date = props.date.unwrap_or_else(jiff::Timestamp::now);
    let datetime = datetime_attribute(date);

    let content = if props.children != Html::default() {
        props.children.clone()
    } else {
        let mut text = format_date(
            &date,
            props,
            props.display_utc,
            props.date_format,
            props.time_format,
        );
        match &props.display_suffix {
            Some(suffix) => {
                text.push(' ');
                text.push_str(suffix);
            }
            None if props.display_utc => text.push_str(" UTC"),
            None => {}
        }
        html!({ text })
    };

    let class = classes!(
        "pf-v6-c-timestamp",
        props.tooltip.is_some().then_some("pf-m-help-text"),
        props.class.clone()
    );

    let tabindex = props.tooltip.is_some().then_some("0");

    let timestamp = html!(
        <span id={props.id.clone()} {class} {tabindex}>
            <time class="pf-v6-c-timestamp__text" {datetime}>{ content }</time>
        </span>
    );

    match &props.tooltip {
        None => timestamp,
        Some(tooltip) => {
            let content = match tooltip {
                TimestampTooltip::Custom(content) => content.clone(),
                TimestampTooltip::Default { suffix } => {
                    // like PatternFly React, never show a time with a precision higher than seconds
                    let time_format = match props.time_format {
                        Some(TimestampFormat::Short) => TimestampFormat::Short,
                        _ => TimestampFormat::Medium,
                    };
                    let text =
                        format_date(&date, props, true, props.date_format, Some(time_format));
                    let suffix = suffix.as_deref().unwrap_or("UTC");
                    html!({ format!("{text} {suffix}") })
                }
            };

            html!(
                <Tooltip
                    {content}
                    position={props.tooltip_position}
                    attach_to_child=true
                    children={timestamp}
                />
            )
        }
    }
}

/// Format a date using the browser's `Date.prototype.toLocaleString`.
fn format_date(
    date: &jiff::Timestamp,
    props: &TimestampProperties,
    utc: bool,
    date_format: Option<TimestampFormat>,
    time_format: Option<TimestampFormat>,
) -> String {
    let options = js_sys::Object::new();
    let set = |key: &str, value: JsValue| {
        let _ = js_sys::Reflect::set(&options, &JsValue::from_str(key), &value);
    };

    match &props.custom_format {
        Some(custom) => {
            for (key, value) in custom {
                set(key.as_str(), JsValue::from_str(value));
            }
        }
        None => {
            if let Some(format) = date_format {
                set("dateStyle", JsValue::from_str(format.as_str()));
            }
            if let Some(format) = time_format {
                set("timeStyle", JsValue::from_str(format.as_str()));
            }
        }
    }
    if let Some(hour12) = props.is_12_hour {
        set("hour12", JsValue::from_bool(hour12));
    }
    if utc {
        set("timeZone", JsValue::from_str("UTC"));
    }

    let locale = props
        .locale
        .as_deref()
        .map(JsValue::from_str)
        .unwrap_or(JsValue::UNDEFINED);

    let js_date = js_sys::Date::new(&JsValue::from_f64(javascript_millis(*date)));

    js_sys::Reflect::get(&js_date, &JsValue::from_str("toLocaleString"))
        .ok()
        .and_then(|f| f.dyn_into::<js_sys::Function>().ok())
        .and_then(|f| f.call2(&js_date, &locale, &options).ok())
        .and_then(|s| s.as_string())
        .unwrap_or_else(|| date.strftime("%a, %d %b %Y %T +0000").to_string())
        // narrow no-break spaces (e.g. before "PM") don't render well everywhere
        .replace('\u{202f}', " ")
}

/// Keep the HTML datetime at millisecond precision, matching JavaScript Date.
fn datetime_attribute(date: jiff::Timestamp) -> String {
    DateTimePrinter::new()
        .precision(Some(3))
        .timestamp_to_string(&date)
}

// Chrono floored submillisecond instants before the epoch; preserve that behavior.
fn javascript_millis(date: jiff::Timestamp) -> f64 {
    date.as_nanosecond().div_euclid(1_000_000) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg_attr(target_family = "wasm", wasm_bindgen_test::wasm_bindgen_test)]
    #[cfg_attr(not(target_family = "wasm"), test)]
    fn preserves_utc_milliseconds_including_before_the_epoch() {
        for (input, expected, millis) in [
            (
                "2024-02-29T12:34:56.123456789Z",
                "2024-02-29T12:34:56.123Z",
                1709210096123.0,
            ),
            ("1970-01-01T00:00:00Z", "1970-01-01T00:00:00.000Z", 0.0),
            (
                "1969-12-31T23:59:59.999999999Z",
                "1969-12-31T23:59:59.999Z",
                -1.0,
            ),
            (
                "2024-03-01T01:00:00+02:00",
                "2024-02-29T23:00:00.000Z",
                1709247600000.0,
            ),
        ] {
            let date = input.parse().unwrap();
            assert_eq!(datetime_attribute(date), expected);
            assert_eq!(javascript_millis(date), millis);
        }
    }

    #[cfg(target_family = "wasm")]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn formats_jiff_timestamps_with_the_browser_and_falls_back_on_invalid_options() {
        let date = "2024-02-29T23:00:00Z".parse().unwrap();
        let mut props = yew::props!(TimestampProperties {
            locale: "en-GB",
            date_format: TimestampFormat::Short,
            time_format: TimestampFormat::Short,
            is_12_hour: false,
        });
        assert_eq!(
            format_date(&date, &props, true, props.date_format, props.time_format),
            "29/02/2024, 23:00"
        );
        props.custom_format = Some(vec![("timeZone".into(), "invalid/timezone".into())]);
        assert_eq!(
            format_date(&date, &props, false, None, None),
            "Thu, 29 Feb 2024 23:00:00 +0000"
        );
    }
}
