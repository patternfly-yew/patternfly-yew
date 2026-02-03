//! Progress bar

use crate::core::{AsClasses, ExtendClasses};
use crate::icon::Icon;
use crate::prelude::*;
use std::ops::Range;
use yew::prelude::*;
use yew::virtual_dom::VChild;

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub enum ProgressValueFormat {
    #[default]
    Percentage,
    Decimal(usize),
    Integer,
    Raw,
}

impl ProgressValueFormat {
    pub fn format(&self, value: f64) -> String {
        match self {
            Self::Percentage => format!("{:.0}%", value * 100f64),
            Self::Decimal(precision) => format!("{value:.precision$}"),
            Self::Raw => format!("{value}"),
            Self::Integer => format!("{:.0}", value),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub enum ProgressSize {
    #[default]
    Default,
    Small,
    Large,
}

impl AsClasses for ProgressSize {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Default => {}
            Self::Small => classes.push(classes!("pf-m-sm")),
            Self::Large => classes.push(classes!("pf-m-lg")),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub enum ProgressMeasureLocation {
    #[default]
    Default,
    Inside,
    Outside,
    None,
}

impl AsClasses for ProgressMeasureLocation {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Default | Self::None => {}
            Self::Outside => classes.push(classes!("pf-m-outside")),
            Self::Inside => classes.push(classes!("pf-m-inside")),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub enum ProgressVariant {
    #[default]
    Default,
    Success,
    Warning,
    Danger,
}

impl AsClasses for ProgressVariant {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Default => {}
            Self::Success => classes.push(classes!("pf-m-success")),
            Self::Warning => classes.push(classes!("pf-m-warning")),
            Self::Danger => classes.push(classes!("pf-m-danger")),
        }
    }
}

impl ProgressVariant {
    pub fn icon(&self) -> Option<Icon> {
        match self {
            Self::Default => None,
            Self::Success => Some(Icon::CheckCircle),
            Self::Warning => Some(Icon::ExclamationTriangle),
            Self::Danger => Some(Icon::TimesCircle),
        }
    }
}

/// Properties for [`Progress`]
#[derive(PartialEq, Properties)]
pub struct ProgressProperties {
    /// The value
    pub value: f64,

    #[prop_or_default]
    pub format: ProgressValueFormat,

    /// The possible value range
    #[prop_or(0f64..1f64)]
    pub range: Range<f64>,

    #[prop_or_default]
    pub description: OptionalHtml,

    /// Base ID of the element
    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub size: ProgressSize,

    /// Location of the measurement
    #[prop_or_default]
    pub location: ProgressMeasureLocation,

    /// Truncate the description, when necessary
    #[prop_or_default]
    pub truncate: bool,

    #[prop_or_default]
    pub variant: ProgressVariant,

    /// Always suppress the icon.
    #[prop_or_default]
    pub no_icon: bool,

    /// Add some helper text.
    #[prop_or_default]
    pub helper_text: Option<VChild<HelperText>>,

    /// Override the text used to represent the value.
    #[prop_or_default]
    pub value_text: Option<String>,

    /// Additional classes for the main element.
    #[prop_or_default]
    pub class: Classes,

    /// Additional classes for the measure element
    #[prop_or_default]
    pub measure_class: Classes,

    /// Additional CSS style
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// Progress component
///
/// > A **progress bar** informs users about the completion status of an ongoing process or task.
///
/// See: <https://www.patternfly.org/components/progress>
///
/// ## Values
///
/// By default, a progress bar work with percent, provided as a value between 0 and 1 (inclusive).
/// Its value will be rendered as "0%" to "100%", with no decimal places. This behavior can be
/// tweaked or fully overridden.
///
/// ## Properties
///
/// Defined by [`ProgressProperties`].
#[function_component(Progress)]
pub fn progress(props: &ProgressProperties) -> Html {
    let id = use_id(props.id.clone());
    let desc_id = use_suffixed_id(&id, "-description");

    let percentage =
        ((props.value / (props.range.end - props.range.start)) * 100f64).clamp(0f64, 100f64);
    let style = format!("width: {percentage:.0}%;");

    let mut class = classes!("pf-v6-c-progress");
    class.extend_from(&props.size);
    class.extend_from(&props.location);
    class.extend_from(&props.variant);
    if props.description.is_none() {
        class.extend(classes!("pf-m-singleline"));
    }
    class.extend(&props.class);

    let measure_class = classes!("pf-v6-c-progress__measure", props.measure_class.clone());
    let mut measure = match props.location {
        ProgressMeasureLocation::None => None,
        _ => {
            let measure = props
                .value_text
                .clone()
                .unwrap_or_else(|| props.format.format(props.value));
            Some(html!(<span class={measure_class}> { measure } </span>))
        }
    };

    let mut description_class = classes!("pf-v6-c-progress__description");
    if props.truncate {
        description_class.push(classes!("pf-m-truncate"));
    }

    let icon = match props.no_icon {
        true => None,
        false => props.variant.icon(),
    }
    .map(|icon| {
        html!(
            <span class="pf-v6-c-progress__status-icon">
                { icon }
            </span>
        )
    });

    html!(
        <div {class} {id} style={props.style.clone()}>
            if let Some(description) = &props.description.0 {
                <div
                    class={description_class}
                    id={desc_id.clone()}
                >
                    { description.clone() }
                </div>
            }
            <div class="pf-v6-c-progress__status" aria-hidden="true">
                if matches!(props.location, ProgressMeasureLocation::Default | ProgressMeasureLocation::Outside) {
                    { measure.take() }
                }
                { icon }
            </div>
            <div
                class="pf-v6-c-progress__bar"
                role="progressbar"
                aria-valuemin={ props.range.start.to_string() }
                aria-valuemax={ props.range.end.to_string() }
                aria-valuenow={ props.value.to_string() }
                aria-labelledby={desc_id}
            >
                <div class="pf-v6-c-progress__indicator" {style}>
                    if matches!(props.location, ProgressMeasureLocation::Inside) {
                        { measure }
                    }
                </div>
            </div>
            if let Some(helper_text) = &props.helper_text {
                <div class="pf-v6-c-progress__helper-text">
                    { helper_text.clone() }
                </div>
            }
        </div>
    )
}
