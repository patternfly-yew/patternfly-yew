//! Legend

use super::element::{BuildContext, Element, ToElement};
use super::label::ChartLabelComponent;
use super::series::standalone;
use super::types::*;
use serde_json::Value;
use wasm_bindgen::JsValue;
use yew::prelude::*;

/// Properties for [`ChartLegend`]
#[derive(Clone, Debug, Default, PartialEq, Properties)]
pub struct ChartLegendProperties {
    #[prop_or_default]
    pub name: Option<AttrValue>,
    /// Entries of the legend
    #[prop_or_default]
    pub data: Vec<ChartLegendDatum>,
    #[prop_or_default]
    pub title: Option<AttrValue>,
    #[prop_or_default]
    pub orientation: Option<ChartOrientation>,
    #[prop_or_default]
    pub items_per_row: Option<u32>,
    /// Horizontal space between entries
    #[prop_or_default]
    pub gutter: Option<f64>,
    /// Vertical space between rows
    #[prop_or_default]
    pub row_gutter: Option<f64>,
    #[prop_or_default]
    pub symbol_spacer: Option<f64>,
    #[prop_or_default]
    pub x: Option<f64>,
    #[prop_or_default]
    pub y: Option<f64>,
    #[prop_or_default]
    pub height: Option<f64>,
    #[prop_or_default]
    pub width: Option<f64>,
    #[prop_or_default]
    pub padding: Option<ChartPadding>,
    /// Colors of the symbols
    #[prop_or_default]
    pub color_scale: Vec<AttrValue>,
    #[prop_or_default]
    pub label_component: Option<ChartLabelComponent>,
    #[prop_or_default]
    pub title_component: Option<ChartLabelComponent>,
    /// Victory style (e.g. `{"labels": {"fontSize": 12}}`)
    #[prop_or_default]
    pub style: Option<Value>,
    #[prop_or_default]
    pub theme_color: Option<ChartThemeColor>,
    /// Additional raw properties, passed to the PatternFly component
    #[prop_or_default]
    pub extra: Option<Value>,
}

impl ChartLegendProperties {
    pub fn new(data: impl IntoIterator<Item = ChartLegendDatum>) -> Self {
        Self {
            data: data.into_iter().collect(),
            ..Default::default()
        }
    }
}

impl ToElement for ChartLegendProperties {
    fn to_element(&self, ctx: &mut BuildContext) -> JsValue {
        let mut element = Element::new("ChartLegend");
        if !self.data.is_empty() {
            element.json("data", &self.data);
        }
        element
            .str("name", &self.name)
            .str("title", &self.title)
            .opt("orientation", self.orientation)
            .opt("itemsPerRow", self.items_per_row)
            .opt("gutter", self.gutter)
            .opt("rowGutter", self.row_gutter)
            .opt("symbolSpacer", self.symbol_spacer)
            .opt("x", self.x)
            .opt("y", self.y)
            .opt("height", self.height)
            .opt("width", self.width)
            .opt_json("padding", &self.padding)
            .list("colorScale", &color_scale(&self.color_scale))
            .element("labelComponent", &self.label_component, ctx)
            .element("titleComponent", &self.title_component, ctx)
            .opt_json("style", &self.style)
            .opt("themeColor", self.theme_color)
            .extra(&self.extra)
            .build()
    }
}

pub(crate) fn color_scale(colors: &[AttrValue]) -> Vec<&str> {
    colors.iter().map(|c| c.as_str()).collect()
}

/// Chart legend
///
/// > A legend, describing the series of a chart.
///
/// See: <https://www.patternfly.org/charts/legends>
///
/// Charts like [`Chart`](super::Chart) or [`ChartPie`](super::ChartPie) render a legend from
/// their `legend_data`. This component can be used to customize that legend
/// (`legend_component`), as child of a chart, or standalone.
///
/// ## Properties
///
/// Defined by [`ChartLegendProperties`].
#[function_component(ChartLegend)]
pub fn chart_legend(props: &ChartLegendProperties) -> Html {
    standalone(props)
}
