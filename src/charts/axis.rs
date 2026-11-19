//! Axis

use super::element::{BuildContext, Element, ToElement};
use super::label::ChartLabelComponent;
use super::series::standalone;
use super::types::*;
use serde_json::Value;
use wasm_bindgen::JsValue;
use yew::prelude::*;

/// Properties for [`ChartAxis`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ChartAxisProperties {
    #[prop_or_default]
    pub name: Option<AttrValue>,
    /// Render the dependent (y) axis
    #[prop_or_default]
    pub dependent_axis: bool,
    /// Label of the axis
    #[prop_or_default]
    pub label: Option<AttrValue>,
    /// Show grid lines
    #[prop_or_default]
    pub show_grid: bool,
    /// Explicit values of the ticks
    #[prop_or_default]
    pub tick_values: Vec<ChartValue>,
    /// Format the label of a tick
    #[prop_or_default]
    pub tick_format: Option<Callback<ChartTick, String>>,
    /// Approximate number of ticks
    #[prop_or_default]
    pub tick_count: Option<u32>,
    /// Drop ticks labels which would overlap
    #[prop_or_default]
    pub fix_label_overlap: bool,
    /// Adjust the legend position, taking the axis label into account
    #[prop_or_default]
    pub fix_axis_label_height: bool,
    #[prop_or_default]
    pub orientation: Option<ChartAxisOrientation>,
    #[prop_or_default]
    pub offset_x: Option<f64>,
    #[prop_or_default]
    pub offset_y: Option<f64>,
    /// Whether the axis crosses the other axis at zero
    #[prop_or_default]
    pub cross_axis: Option<bool>,
    #[prop_or_default]
    pub invert_axis: bool,
    #[prop_or_default]
    pub domain: Option<[f64; 2]>,
    #[prop_or_default]
    pub axis_label_component: Option<ChartLabelComponent>,
    #[prop_or_default]
    pub tick_label_component: Option<ChartLabelComponent>,
    /// Victory style (e.g. `{"tickLabels": {"fontSize": 12}}`)
    #[prop_or_default]
    pub style: Option<Value>,
    #[prop_or_default]
    pub theme_color: Option<ChartThemeColor>,
    /// Additional raw properties, passed to the PatternFly component
    #[prop_or_default]
    pub extra: Option<Value>,
}

impl ToElement for ChartAxisProperties {
    fn to_element(&self, ctx: &mut BuildContext) -> JsValue {
        let mut element = Element::new("ChartAxis");
        if let Some(tick_format) = &self.tick_format {
            element.set("tickFormat", tick_callback(ctx, tick_format));
        }
        element
            .str("name", &self.name)
            .opt("dependentAxis", self.dependent_axis.then_some(true))
            .str("label", &self.label)
            .opt("showGrid", self.show_grid.then_some(true))
            .list("tickValues", &self.tick_values)
            .opt("tickCount", self.tick_count)
            .opt("fixLabelOverlap", self.fix_label_overlap.then_some(true))
            .opt(
                "fixAxisLabelHeight",
                self.fix_axis_label_height.then_some(true),
            )
            .opt("orientation", self.orientation)
            .opt("offsetX", self.offset_x)
            .opt("offsetY", self.offset_y)
            .opt("crossAxis", self.cross_axis)
            .opt("invertAxis", self.invert_axis.then_some(true))
            .opt_json("domain", &self.domain)
            .element("axisLabelComponent", &self.axis_label_component, ctx)
            .element("tickLabelComponent", &self.tick_label_component, ctx)
            .opt_json("style", &self.style)
            .opt("themeColor", self.theme_color)
            .extra(&self.extra)
            .build()
    }
}

/// Chart axis
///
/// > An axis of a chart, rendering ticks, tick labels and an optional label and grid.
///
/// See: <https://www.patternfly.org/charts/about-charts>
///
/// ## Properties
///
/// Defined by [`ChartAxisProperties`].
#[function_component(ChartAxis)]
pub fn chart_axis(props: &ChartAxisProperties) -> Html {
    standalone(props)
}
