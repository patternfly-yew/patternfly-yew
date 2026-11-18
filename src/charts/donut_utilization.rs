//! Donut utilization and threshold charts

use super::element::{BuildContext, Element, ToElement};
use super::label::ChartLabelComponent;
use super::legend::{ChartLegendProperties, color_scale};
use super::pie::ChartSubTitlePosition;
use super::series::standalone;
use super::types::*;
use serde::Serialize;
use serde_json::Value;
use wasm_bindgen::JsValue;
use yew::prelude::*;

/// A threshold of a [`ChartDonutUtilization`].
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ChartThresholdValue {
    pub value: f64,
    /// Color of the utilization, once the threshold is reached
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

impl ChartThresholdValue {
    pub fn new(value: impl Into<f64>) -> Self {
        Self {
            value: value.into(),
            color: None,
        }
    }

    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }
}

/// Properties for [`ChartDonutUtilization`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ChartDonutUtilizationProperties {
    #[prop_or_default]
    pub name: Option<AttrValue>,
    /// Accessible title
    #[prop_or_default]
    pub aria_title: Option<AttrValue>,
    /// Accessible description
    #[prop_or_default]
    pub aria_desc: Option<AttrValue>,
    /// The utilization, using `x` as name and `y` as percentage
    #[prop_or_default]
    pub data: Option<ChartDatum>,
    /// Thresholds, changing the color of the utilization
    #[prop_or_default]
    pub thresholds: Vec<ChartThresholdValue>,
    /// Invert the thresholds (alert when the value is lower)
    #[prop_or_default]
    pub invert: bool,
    /// Title, rendered in the center
    #[prop_or_default]
    pub title: Option<AttrValue>,
    #[prop_or_default]
    pub sub_title: Option<AttrValue>,
    #[prop_or_default]
    pub sub_title_position: Option<ChartSubTitlePosition>,
    /// Labels (tooltips)
    #[prop_or_default]
    pub labels: Option<ChartLabels>,
    #[prop_or_default]
    pub label_component: Option<ChartLabelComponent>,
    /// Show tooltips (true by default)
    #[prop_or_default]
    pub allow_tooltip: Option<bool>,
    #[prop_or_default]
    pub constrain_to_visible_area: bool,
    #[prop_or_default]
    pub legend_data: Vec<ChartLegendDatum>,
    #[prop_or_default]
    pub legend_position: Option<ChartLegendPosition>,
    #[prop_or_default]
    pub legend_orientation: Option<ChartOrientation>,
    #[prop_or_default]
    pub legend_component: Option<ChartLegendProperties>,
    #[prop_or_default]
    pub height: Option<f64>,
    #[prop_or_default]
    pub width: Option<f64>,
    #[prop_or_default]
    pub padding: Option<ChartPadding>,
    #[prop_or_default]
    pub inner_radius: Option<f64>,
    #[prop_or_default]
    pub radius: Option<f64>,
    #[prop_or_default]
    pub theme_color: Option<ChartThemeColor>,
    #[prop_or_default]
    pub color_scale: Vec<AttrValue>,
    #[prop_or_default]
    pub has_patterns: bool,
    /// Victory style
    #[prop_or_default]
    pub style: Option<Value>,
    /// Additional raw properties, passed to the PatternFly component
    #[prop_or_default]
    pub extra: Option<Value>,
}

impl ToElement for ChartDonutUtilizationProperties {
    fn to_element(&self, ctx: &mut BuildContext) -> JsValue {
        let mut element = Element::new("ChartDonutUtilization");
        if let Some(labels) = &self.labels {
            element.set("labels", labels.to_js(ctx));
        }
        element
            .str("name", &self.name)
            .str("ariaTitle", &self.aria_title)
            .str("ariaDesc", &self.aria_desc)
            .opt_json("data", &self.data)
            .list("thresholds", &self.thresholds)
            .opt("invert", self.invert.then_some(true))
            .str("title", &self.title)
            .str("subTitle", &self.sub_title)
            .opt("subTitlePosition", self.sub_title_position)
            .element("labelComponent", &self.label_component, ctx)
            .opt("allowTooltip", self.allow_tooltip)
            .opt(
                "constrainToVisibleArea",
                self.constrain_to_visible_area.then_some(true),
            )
            .list("legendData", &self.legend_data)
            .opt("legendPosition", self.legend_position)
            .opt("legendOrientation", self.legend_orientation)
            .element("legendComponent", &self.legend_component, ctx)
            .opt("height", self.height)
            .opt("width", self.width)
            .opt_json("padding", &self.padding)
            .opt("innerRadius", self.inner_radius)
            .opt("radius", self.radius)
            .opt("themeColor", self.theme_color)
            .list("colorScale", &color_scale(&self.color_scale))
            .opt("hasPatterns", self.has_patterns.then_some(true))
            .opt_json("style", &self.style)
            .extra(&self.extra)
            .build()
    }
}

/// Donut utilization chart
///
/// > A **donut utilization chart** is used to display the utilization (percentage) of a resource,
/// > optionally changing its color when reaching thresholds.
///
/// See: <https://www.patternfly.org/charts/donut-utilization-chart>
///
/// Can be used standalone, or as child of a [`ChartDonutThreshold`].
///
/// ## Properties
///
/// Defined by [`ChartDonutUtilizationProperties`].
#[function_component(ChartDonutUtilization)]
pub fn chart_donut_utilization(props: &ChartDonutUtilizationProperties) -> Html {
    standalone(props)
}

/// Properties for [`ChartDonutThreshold`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ChartDonutThresholdProperties {
    /// The utilization donut, rendered inside the threshold
    #[prop_or_default]
    pub children: ChildrenWithProps<ChartDonutUtilization>,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    /// Accessible title
    #[prop_or_default]
    pub aria_title: Option<AttrValue>,
    /// Accessible description
    #[prop_or_default]
    pub aria_desc: Option<AttrValue>,
    /// The thresholds, using `x` as name and `y` as percentage
    #[prop_or_default]
    pub data: Vec<ChartDatum>,
    /// Invert the thresholds (alert when the value is lower)
    #[prop_or_default]
    pub invert: bool,
    /// Labels (tooltips)
    #[prop_or_default]
    pub labels: Option<ChartLabels>,
    #[prop_or_default]
    pub label_component: Option<ChartLabelComponent>,
    #[prop_or_default]
    pub allow_tooltip: Option<bool>,
    #[prop_or_default]
    pub constrain_to_visible_area: bool,
    #[prop_or_default]
    pub height: Option<f64>,
    #[prop_or_default]
    pub width: Option<f64>,
    #[prop_or_default]
    pub padding: Option<ChartPadding>,
    #[prop_or_default]
    pub theme_color: Option<ChartThemeColor>,
    /// Victory style
    #[prop_or_default]
    pub style: Option<Value>,
    /// Additional raw properties, passed to the PatternFly component
    #[prop_or_default]
    pub extra: Option<Value>,
}

impl ToElement for ChartDonutThresholdProperties {
    fn to_element(&self, ctx: &mut BuildContext) -> JsValue {
        let mut element = Element::new("ChartDonutThreshold");
        if let Some(labels) = &self.labels {
            element.set("labels", labels.to_js(ctx));
        }
        element
            .str("name", &self.name)
            .str("ariaTitle", &self.aria_title)
            .str("ariaDesc", &self.aria_desc)
            .json("data", &self.data)
            .opt("invert", self.invert.then_some(true))
            .element("labelComponent", &self.label_component, ctx)
            .opt("allowTooltip", self.allow_tooltip)
            .opt(
                "constrainToVisibleArea",
                self.constrain_to_visible_area.then_some(true),
            )
            .opt("height", self.height)
            .opt("width", self.width)
            .opt_json("padding", &self.padding)
            .opt("themeColor", self.theme_color)
            .opt_json("style", &self.style)
            .extra(&self.extra);
        for child in self.children.iter() {
            element.child(child.props.to_element(ctx));
        }
        element.build()
    }
}

/// Donut threshold chart
///
/// > A **donut threshold chart** displays thresholds (static ranges), around a
/// > [`ChartDonutUtilization`].
///
/// See: <https://www.patternfly.org/charts/donut-utilization-chart>
///
/// ## Properties
///
/// Defined by [`ChartDonutThresholdProperties`].
///
/// ## Children
///
/// A [`ChartDonutUtilization`].
#[function_component(ChartDonutThreshold)]
pub fn chart_donut_threshold(props: &ChartDonutThresholdProperties) -> Html {
    standalone(props)
}
