//! Labels and tooltips

use super::element::{BuildContext, Element, ToElement};
use super::types::*;
use serde_json::Value;
use wasm_bindgen::JsValue;
use yew::html::IntoPropValue;
use yew::prelude::*;

/// A label, rendering text (`ChartLabel`).
///
/// Used as label component of series, or as title component.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChartLabel {
    pub dx: Option<f64>,
    pub dy: Option<f64>,
    pub text_anchor: Option<ChartTextAnchor>,
    pub style: Option<Value>,
    /// Additional raw properties
    pub extra: Option<Value>,
}

impl ChartLabel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_dx(mut self, dx: f64) -> Self {
        self.dx = Some(dx);
        self
    }

    pub fn with_dy(mut self, dy: f64) -> Self {
        self.dy = Some(dy);
        self
    }

    pub fn with_text_anchor(mut self, text_anchor: ChartTextAnchor) -> Self {
        self.text_anchor = Some(text_anchor);
        self
    }

    pub fn with_style(mut self, style: Value) -> Self {
        self.style = Some(style);
        self
    }

    pub fn with_extra(mut self, extra: Value) -> Self {
        self.extra = Some(extra);
        self
    }
}

impl ToElement for ChartLabel {
    fn to_element(&self, _ctx: &mut BuildContext) -> JsValue {
        Element::new("ChartLabel")
            .opt("dx", self.dx)
            .opt("dy", self.dy)
            .opt("textAnchor", self.text_anchor)
            .opt_json("style", &self.style)
            .extra(&self.extra)
            .build()
    }
}

/// A tooltip (`ChartTooltip`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChartTooltip {
    pub constrain_to_visible_area: Option<bool>,
    pub orientation: Option<ChartTooltipOrientation>,
    pub flyout_width: Option<f64>,
    pub flyout_height: Option<f64>,
    pub dx: Option<f64>,
    pub dy: Option<f64>,
    pub style: Option<Value>,
    /// Additional raw properties
    pub extra: Option<Value>,
}

impl ChartTooltip {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn constrain_to_visible_area(mut self) -> Self {
        self.constrain_to_visible_area = Some(true);
        self
    }

    pub fn with_orientation(mut self, orientation: ChartTooltipOrientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    pub fn with_flyout_width(mut self, width: f64) -> Self {
        self.flyout_width = Some(width);
        self
    }

    pub fn with_flyout_height(mut self, height: f64) -> Self {
        self.flyout_height = Some(height);
        self
    }

    pub fn with_dx(mut self, dx: f64) -> Self {
        self.dx = Some(dx);
        self
    }

    pub fn with_dy(mut self, dy: f64) -> Self {
        self.dy = Some(dy);
        self
    }

    pub fn with_style(mut self, style: Value) -> Self {
        self.style = Some(style);
        self
    }

    pub fn with_extra(mut self, extra: Value) -> Self {
        self.extra = Some(extra);
        self
    }
}

impl ToElement for ChartTooltip {
    fn to_element(&self, _ctx: &mut BuildContext) -> JsValue {
        Element::new("ChartTooltip")
            .opt("constrainToVisibleArea", self.constrain_to_visible_area)
            .opt("orientation", self.orientation)
            .opt("flyoutWidth", self.flyout_width)
            .opt("flyoutHeight", self.flyout_height)
            .opt("dx", self.dx)
            .opt("dy", self.dy)
            .opt_json("style", &self.style)
            .extra(&self.extra)
            .build()
    }
}

/// A tooltip, showing the legend of all series at a position (`ChartLegendTooltip`).
///
/// To be used as label component of a [`ChartContainer::voronoi_cursor`] container.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChartLegendTooltip {
    pub legend_data: Vec<ChartLegendDatum>,
    /// Title of the tooltip, receiving the data point
    pub title: Option<Callback<Datum, String>>,
    pub constrain_to_visible_area: Option<bool>,
    /// Additional raw properties
    pub extra: Option<Value>,
}

impl ChartLegendTooltip {
    pub fn new(legend_data: impl IntoIterator<Item = ChartLegendDatum>) -> Self {
        Self {
            legend_data: legend_data.into_iter().collect(),
            ..Default::default()
        }
    }

    pub fn with_title(mut self, title: impl Fn(Datum) -> String + 'static) -> Self {
        self.title = Some(Callback::from(title));
        self
    }

    pub fn constrain_to_visible_area(mut self) -> Self {
        self.constrain_to_visible_area = Some(true);
        self
    }

    pub fn with_extra(mut self, extra: Value) -> Self {
        self.extra = Some(extra);
        self
    }
}

impl ToElement for ChartLegendTooltip {
    fn to_element(&self, ctx: &mut BuildContext) -> JsValue {
        let mut element = Element::new("ChartLegendTooltip");
        element
            .json("legendData", &self.legend_data)
            .opt("constrainToVisibleArea", self.constrain_to_visible_area);
        if let Some(title) = &self.title {
            element.set("title", plain_datum_callback(ctx, title));
        }
        element.extra(&self.extra).build()
    }
}

/// A tooltip following the cursor (`ChartCursorTooltip`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChartCursorTooltip {
    pub constrain_to_visible_area: Option<bool>,
    pub center_offset: Option<ChartAxisValue>,
    pub flyout_width: Option<f64>,
    pub flyout_height: Option<f64>,
    /// Additional raw properties
    pub extra: Option<Value>,
}

impl ChartCursorTooltip {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn constrain_to_visible_area(mut self) -> Self {
        self.constrain_to_visible_area = Some(true);
        self
    }

    pub fn with_center_offset(mut self, offset: ChartAxisValue) -> Self {
        self.center_offset = Some(offset);
        self
    }

    pub fn with_extra(mut self, extra: Value) -> Self {
        self.extra = Some(extra);
        self
    }
}

impl ToElement for ChartCursorTooltip {
    fn to_element(&self, _ctx: &mut BuildContext) -> JsValue {
        Element::new("ChartCursorTooltip")
            .opt("constrainToVisibleArea", self.constrain_to_visible_area)
            .opt_json("centerOffset", &self.center_offset)
            .opt("flyoutWidth", self.flyout_width)
            .opt("flyoutHeight", self.flyout_height)
            .extra(&self.extra)
            .build()
    }
}

/// A component rendering labels of data points.
#[derive(Clone, Debug, PartialEq)]
pub enum ChartLabelComponent {
    Label(ChartLabel),
    Tooltip(ChartTooltip),
    CursorTooltip(ChartCursorTooltip),
    LegendTooltip(ChartLegendTooltip),
}

impl ToElement for ChartLabelComponent {
    fn to_element(&self, ctx: &mut BuildContext) -> JsValue {
        match self {
            Self::Label(label) => label.to_element(ctx),
            Self::Tooltip(tooltip) => tooltip.to_element(ctx),
            Self::CursorTooltip(tooltip) => tooltip.to_element(ctx),
            Self::LegendTooltip(tooltip) => tooltip.to_element(ctx),
        }
    }
}

macro_rules! label_component {
    ($($t:ident),*) => {
        $(
            impl From<$t> for ChartLabelComponent {
                fn from(value: $t) -> Self {
                    Self::$t(value)
                }
            }

            impl IntoPropValue<Option<ChartLabelComponent>> for $t {
                fn into_prop_value(self) -> Option<ChartLabelComponent> {
                    Some(ChartLabelComponent::$t(self))
                }
            }
        )*
    };
}

use ChartCursorTooltip as CursorTooltip;
use ChartLabel as Label;
use ChartLegendTooltip as LegendTooltip;
use ChartTooltip as Tooltip;

label_component!(Label, Tooltip, CursorTooltip, LegendTooltip);
