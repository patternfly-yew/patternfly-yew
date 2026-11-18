//! Chart containers, providing interactivity (tooltips, cursor).

use super::element::{BuildContext, Element, ToElement};
use super::label::ChartLabelComponent;
use super::sys;
use super::types::*;
use serde_json::Value;
use wasm_bindgen::JsValue;
use yew::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum ChartContainerKind {
    #[default]
    Default,
    Voronoi,
    Cursor,
    VoronoiCursor,
}

/// The container of a chart (`containerComponent`).
///
/// * [`ChartContainer::new`]: the default container (`ChartContainer`)
/// * [`ChartContainer::voronoi`]: shows tooltips for the nearest data points (`ChartVoronoiContainer`)
/// * [`ChartContainer::cursor`]: shows a cursor line (`ChartCursorContainer`)
/// * [`ChartContainer::voronoi_cursor`]: combination of both, e.g. with a [`ChartLegendTooltip`](super::ChartLegendTooltip)
///   (`createContainer("voronoi", "cursor")`)
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChartContainer {
    kind: ChartContainerKind,
    pub labels: Option<ChartLabels>,
    pub label_component: Option<ChartLabelComponent>,
    pub constrain_to_visible_area: Option<bool>,
    pub mouse_follow_tooltips: Option<bool>,
    pub voronoi_dimension: Option<ChartDimension>,
    pub voronoi_padding: Option<f64>,
    /// Names of series to ignore
    pub voronoi_blacklist: Vec<String>,
    pub cursor_dimension: Option<ChartDimension>,
    /// Label of the cursor, receiving a data point with the `x` and `y` values of the cursor
    pub cursor_label: Option<Callback<Datum, String>>,
    pub cursor_label_component: Option<ChartLabelComponent>,
    pub responsive: Option<bool>,
    /// Additional raw properties
    pub extra: Option<Value>,
}

impl ChartContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn voronoi() -> Self {
        Self {
            kind: ChartContainerKind::Voronoi,
            ..Default::default()
        }
    }

    pub fn cursor() -> Self {
        Self {
            kind: ChartContainerKind::Cursor,
            ..Default::default()
        }
    }

    pub fn voronoi_cursor() -> Self {
        Self {
            kind: ChartContainerKind::VoronoiCursor,
            ..Default::default()
        }
    }

    /// Set the labels of the tooltips, from a function.
    pub fn with_labels(mut self, labels: impl Fn(Datum) -> String + 'static) -> Self {
        self.labels = Some(ChartLabels::from_fn(labels));
        self
    }

    pub fn with_label_component(mut self, component: impl Into<ChartLabelComponent>) -> Self {
        self.label_component = Some(component.into());
        self
    }

    pub fn constrain_to_visible_area(mut self) -> Self {
        self.constrain_to_visible_area = Some(true);
        self
    }

    pub fn mouse_follow_tooltips(mut self) -> Self {
        self.mouse_follow_tooltips = Some(true);
        self
    }

    pub fn with_voronoi_dimension(mut self, dimension: ChartDimension) -> Self {
        self.voronoi_dimension = Some(dimension);
        self
    }

    pub fn with_voronoi_padding(mut self, padding: f64) -> Self {
        self.voronoi_padding = Some(padding);
        self
    }

    pub fn with_voronoi_blacklist(
        mut self,
        names: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.voronoi_blacklist = names.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_cursor_dimension(mut self, dimension: ChartDimension) -> Self {
        self.cursor_dimension = Some(dimension);
        self
    }

    pub fn with_cursor_label(mut self, label: impl Fn(Datum) -> String + 'static) -> Self {
        self.cursor_label = Some(Callback::from(label));
        self
    }

    pub fn with_cursor_label_component(
        mut self,
        component: impl Into<ChartLabelComponent>,
    ) -> Self {
        self.cursor_label_component = Some(component.into());
        self
    }

    pub fn with_responsive(mut self, responsive: bool) -> Self {
        self.responsive = Some(responsive);
        self
    }

    pub fn with_extra(mut self, extra: Value) -> Self {
        self.extra = Some(extra);
        self
    }
}

impl ToElement for ChartContainer {
    fn to_element(&self, ctx: &mut BuildContext) -> JsValue {
        let mut element = match self.kind {
            ChartContainerKind::Default => Element::new("ChartContainer"),
            ChartContainerKind::Voronoi => Element::new("ChartVoronoiContainer"),
            ChartContainerKind::Cursor => Element::new("ChartCursorContainer"),
            ChartContainerKind::VoronoiCursor => match sys::container_type("voronoi", "cursor") {
                Ok(r#type) => Element::of_type(r#type),
                Err(err) => {
                    log::error!("Failed to create container type: {err:?}");
                    Element::new("ChartContainer")
                }
            },
        };

        if let Some(labels) = &self.labels {
            element.set("labels", labels.to_js(ctx));
        }
        if let Some(cursor_label) = &self.cursor_label {
            element.set("cursorLabel", datum_callback(ctx, cursor_label));
        }

        element
            .element("labelComponent", &self.label_component, ctx)
            .element("cursorLabelComponent", &self.cursor_label_component, ctx)
            .opt("constrainToVisibleArea", self.constrain_to_visible_area)
            .opt("mouseFollowTooltips", self.mouse_follow_tooltips)
            .opt("voronoiDimension", self.voronoi_dimension)
            .opt("voronoiPadding", self.voronoi_padding)
            .list("voronoiBlacklist", &self.voronoi_blacklist)
            .opt("cursorDimension", self.cursor_dimension)
            .opt("responsive", self.responsive)
            .extra(&self.extra)
            .build()
    }
}
