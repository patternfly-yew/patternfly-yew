//! Types shared by the chart components.

use super::element::{BuildContext, get};
use serde::Serialize;
use serde_json::{Map, Value};
use std::fmt;
use wasm_bindgen::JsValue;
use yew::prelude::*;

/// A value of the independent (x) axis, either a number or a category.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ChartValue {
    Number(f64),
    Text(String),
}

impl ChartValue {
    fn from_js(value: &JsValue) -> Option<Self> {
        if let Some(n) = value.as_f64() {
            Some(Self::Number(n))
        } else {
            value.as_string().map(Self::Text)
        }
    }

    /// The value as number, if it is one.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Number(n) => Some(*n),
            Self::Text(_) => None,
        }
    }
}

impl fmt::Display for ChartValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::Text(s) => f.write_str(s),
        }
    }
}

macro_rules! chart_value_from_number {
    ($($t:ty),*) => {
        $(impl From<$t> for ChartValue {
            fn from(value: $t) -> Self {
                Self::Number(value as f64)
            }
        })*
    };
}

chart_value_from_number!(f64, f32, i8, i16, i32, i64, u8, u16, u32, u64, usize);

impl From<&str> for ChartValue {
    fn from(value: &str) -> Self {
        Self::Text(value.to_string())
    }
}

impl From<String> for ChartValue {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

/// A value of the dependent (y) axis.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ChartY {
    Number(f64),
    /// Multiple values, e.g. for a box plot
    Values(Vec<f64>),
}

/// A data point of a chart.
///
/// Fields which are not set are omitted. Additional fields can be added using
/// [`ChartDatum::with`], they are accessible from callbacks through [`Datum::get`].
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct ChartDatum {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<ChartValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<ChartY>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y0: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

impl ChartDatum {
    /// A data point with an `x` and a `y` value.
    pub fn new(x: impl Into<ChartValue>, y: impl Into<f64>) -> Self {
        Self {
            x: Some(x.into()),
            y: Some(ChartY::Number(y.into())),
            ..Default::default()
        }
    }

    /// A data point with only a `y` value (e.g. for bullet charts).
    pub fn y(y: impl Into<f64>) -> Self {
        Self {
            y: Some(ChartY::Number(y.into())),
            ..Default::default()
        }
    }

    /// A data point with multiple `y` values (e.g. for box plots).
    pub fn values(x: impl Into<ChartValue>, y: impl IntoIterator<Item = f64>) -> Self {
        Self {
            x: Some(x.into()),
            y: Some(ChartY::Values(y.into_iter().collect())),
            ..Default::default()
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_y0(mut self, y0: impl Into<f64>) -> Self {
        self.y0 = Some(y0.into());
        self
    }

    pub fn with_size(mut self, size: impl Into<f64>) -> Self {
        self.size = Some(size.into());
        self
    }

    pub fn with_symbol(mut self, symbol: ChartSymbol) -> Self {
        self.symbol = Some(symbol.as_str().to_string());
        self
    }

    /// Add an additional, custom, field.
    pub fn with(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.extra.insert(key.into(), value.into());
        self
    }
}

impl<X: Into<ChartValue>, Y: Into<f64>> From<(X, Y)> for ChartDatum {
    fn from((x, y): (X, Y)) -> Self {
        Self::new(x, y)
    }
}

/// Create a list of [`ChartDatum`] from items convertible into one (like `(x, y)` tuples).
pub fn chart_data<T: Into<ChartDatum>>(data: impl IntoIterator<Item = T>) -> Vec<ChartDatum> {
    data.into_iter().map(Into::into).collect()
}

/// A data point, as provided by the chart to callbacks.
#[derive(Clone, Debug)]
pub struct Datum(JsValue);

impl Datum {
    pub(crate) fn new(value: JsValue) -> Self {
        Self(value)
    }

    /// Get a raw field of the data point.
    pub fn get(&self, key: &str) -> JsValue {
        get(&self.0, key)
    }

    /// Get a field as string.
    pub fn get_string(&self, key: &str) -> Option<String> {
        let value = self.get(key);
        value
            .as_string()
            .or_else(|| value.as_f64().map(|n| n.to_string()))
    }

    /// Get a field as number.
    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.get(key).as_f64()
    }

    pub fn x(&self) -> Option<ChartValue> {
        ChartValue::from_js(&self.get("x"))
    }

    pub fn y(&self) -> Option<f64> {
        self.get_f64("y")
    }

    pub fn name(&self) -> Option<String> {
        self.get_string("name")
    }

    /// The name of the series (child) the data point belongs to.
    pub fn child_name(&self) -> Option<String> {
        self.get_string("childName")
    }

    /// The raw JavaScript value.
    pub fn raw(&self) -> &JsValue {
        &self.0
    }
}

/// A tick of an axis, as provided to [`ChartAxisProperties::tick_format`](super::ChartAxisProperties::tick_format).
#[derive(Clone, Debug)]
pub struct ChartTick {
    pub value: ChartValue,
    pub index: usize,
}

/// Labels of data points.
#[derive(Clone, Debug, PartialEq)]
pub enum ChartLabels {
    /// A label per data point
    Static(Vec<AttrValue>),
    /// A function returning the label of a data point
    Callback(Callback<Datum, String>),
}

impl ChartLabels {
    /// Create labels from a function.
    pub fn from_fn(f: impl Fn(Datum) -> String + 'static) -> Self {
        Self::Callback(Callback::from(f))
    }

    pub(crate) fn to_js(&self, ctx: &mut BuildContext) -> JsValue {
        match self {
            Self::Static(labels) => {
                let labels: Vec<&str> = labels.iter().map(|l| l.as_str()).collect();
                super::element::to_js(&labels)
            }
            Self::Callback(callback) => datum_callback(ctx, callback),
        }
    }
}

impl From<Callback<Datum, String>> for ChartLabels {
    fn from(value: Callback<Datum, String>) -> Self {
        Self::Callback(value)
    }
}

/// A JavaScript function receiving `({datum})`.
pub(crate) fn datum_callback(
    ctx: &mut BuildContext,
    callback: &Callback<Datum, String>,
) -> JsValue {
    let callback = callback.clone();
    ctx.function1(move |args| JsValue::from(callback.emit(Datum::new(get(&args, "datum")))))
}

/// A JavaScript function receiving `(datum)`.
pub(crate) fn plain_datum_callback(
    ctx: &mut BuildContext,
    callback: &Callback<Datum, String>,
) -> JsValue {
    let callback = callback.clone();
    ctx.function1(move |datum| JsValue::from(callback.emit(Datum::new(datum))))
}

/// A JavaScript function receiving `(tick, index)`.
pub(crate) fn tick_callback(
    ctx: &mut BuildContext,
    callback: &Callback<ChartTick, String>,
) -> JsValue {
    let callback = callback.clone();
    ctx.function2(move |tick, index| {
        let value = ChartValue::from_js(&tick).unwrap_or(ChartValue::Text(String::new()));
        let index = index.as_f64().unwrap_or_default() as usize;
        JsValue::from(callback.emit(ChartTick { value, index }))
    })
}

/// Padding of a chart.
///
/// Sides which are not set use the default of the theme.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize)]
pub struct ChartPadding {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<f64>,
}

impl ChartPadding {
    /// The same padding on all sides.
    pub fn all(value: impl Into<f64>) -> Self {
        let value = Some(value.into());
        Self {
            top: value,
            bottom: value,
            left: value,
            right: value,
        }
    }

    pub fn new(top: f64, right: f64, bottom: f64, left: f64) -> Self {
        Self {
            top: Some(top),
            bottom: Some(bottom),
            left: Some(left),
            right: Some(right),
        }
    }

    pub fn with_top(mut self, value: impl Into<f64>) -> Self {
        self.top = Some(value.into());
        self
    }

    pub fn with_bottom(mut self, value: impl Into<f64>) -> Self {
        self.bottom = Some(value.into());
        self
    }

    pub fn with_left(mut self, value: impl Into<f64>) -> Self {
        self.left = Some(value.into());
        self
    }

    pub fn with_right(mut self, value: impl Into<f64>) -> Self {
        self.right = Some(value.into());
        self
    }
}

/// The domain (range of values) of a chart.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize)]
pub struct ChartDomain {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<[f64; 2]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<[f64; 2]>,
}

impl ChartDomain {
    pub fn x(min: f64, max: f64) -> Self {
        Self {
            x: Some([min, max]),
            y: None,
        }
    }

    pub fn y(min: f64, max: f64) -> Self {
        Self {
            x: None,
            y: Some([min, max]),
        }
    }

    pub fn with_x(mut self, min: f64, max: f64) -> Self {
        self.x = Some([min, max]);
        self
    }

    pub fn with_y(mut self, min: f64, max: f64) -> Self {
        self.y = Some([min, max]);
        self
    }
}

/// A value per axis, used for minimum/maximum domain, or domain padding.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize)]
pub struct ChartAxisValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
}

impl ChartAxisValue {
    pub fn x(value: impl Into<f64>) -> Self {
        Self {
            x: Some(value.into()),
            y: None,
        }
    }

    pub fn y(value: impl Into<f64>) -> Self {
        Self {
            x: None,
            y: Some(value.into()),
        }
    }

    pub fn xy(x: impl Into<f64>, y: impl Into<f64>) -> Self {
        Self {
            x: Some(x.into()),
            y: Some(y.into()),
        }
    }
}

/// Scale of an axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ChartScaleType {
    Linear,
    Time,
    Log,
    Sqrt,
}

/// Scale per axis.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub struct ChartScale {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<ChartScaleType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<ChartScaleType>,
}

macro_rules! str_enum {
    ($(#[$meta:meta])* $name:ident { $($(#[$vmeta:meta])* $variant:ident => $value:literal),* $(,)? }) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum $name {
            $($(#[$vmeta])* $variant),*
        }

        impl $name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => $value),*
                }
            }
        }

        impl $crate::charts::element::IntoJs for $name {
            fn into_js(self) -> JsValue {
                JsValue::from_str(self.as_str())
            }
        }
    };
}

pub(crate) use str_enum;

str_enum!(
    /// Color theme of a chart.
    ///
    /// See: <https://www.patternfly.org/charts/colors-for-charts>
    #[derive(Default)]
    ChartThemeColor {
        #[default]
        Blue => "blue",
        Gray => "gray",
        Green => "green",
        Multi => "multi",
        MultiOrdered => "multi-ordered",
        MultiUnordered => "multi-unordered",
        Orange => "orange",
        Purple => "purple",
        Skeleton => "skeleton",
        Teal => "teal",
        Yellow => "yellow",
    }
);

str_enum!(
    /// Position of the legend.
    ChartLegendPosition {
        Bottom => "bottom",
        BottomLeft => "bottom-left",
        Right => "right",
    }
);

str_enum!(
    /// Orientation of a legend or a label.
    ChartOrientation {
        Horizontal => "horizontal",
        Vertical => "vertical",
    }
);

str_enum!(
    /// Orientation of a tooltip.
    ChartTooltipOrientation {
        Top => "top",
        Bottom => "bottom",
        Left => "left",
        Right => "right",
    }
);

str_enum!(
    /// A dimension of a chart.
    ChartDimension {
        X => "x",
        Y => "y",
    }
);

str_enum!(
    /// Interpolation of lines and areas.
    ChartInterpolation {
        Basis => "basis",
        Cardinal => "cardinal",
        CatmullRom => "catmullRom",
        Linear => "linear",
        MonotoneX => "monotoneX",
        MonotoneY => "monotoneY",
        Natural => "natural",
        Step => "step",
        StepAfter => "stepAfter",
        StepBefore => "stepBefore",
    }
);

str_enum!(
    /// Symbol of a data point (scatter, legend).
    ChartSymbol {
        Circle => "circle",
        Cross => "cross",
        Diamond => "diamond",
        Eyeglasses => "eyeglasses",
        Minus => "minus",
        Plus => "plus",
        Square => "square",
        Star => "star",
        Dash => "dash",
        Threshold => "threshold",
        TriangleDown => "triangleDown",
        TriangleUp => "triangleUp",
    }
);

str_enum!(
    /// Alignment of bars, relative to their value.
    ChartAlignment {
        Start => "start",
        Middle => "middle",
        End => "end",
    }
);

str_enum!(
    /// Anchor of a text.
    ChartTextAnchor {
        Start => "start",
        Middle => "middle",
        End => "end",
        Inherit => "inherit",
    }
);

str_enum!(
    /// Orientation of an axis.
    ChartAxisOrientation {
        Top => "top",
        Bottom => "bottom",
        Left => "left",
        Right => "right",
    }
);

/// An entry of a legend.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct ChartLegendDatum {
    pub name: String,
    /// The name of the series this entry is for (required by [`ChartLegendTooltip`](super::ChartLegendTooltip))
    #[serde(rename = "childName", skip_serializing_if = "Option::is_none")]
    pub child_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<ChartLegendSymbol>,
}

/// The symbol of a legend entry.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct ChartLegendSymbol {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl ChartLegendDatum {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            child_name: None,
            symbol: None,
        }
    }

    /// Set the name of the series this entry is for.
    pub fn with_child_name(mut self, child_name: impl Into<String>) -> Self {
        self.child_name = Some(child_name.into());
        self
    }

    /// Set the fill color of the symbol (any CSS color).
    pub fn with_fill(mut self, fill: impl Into<String>) -> Self {
        self.symbol.get_or_insert_default().fill = Some(fill.into());
        self
    }

    pub fn with_symbol(mut self, symbol: ChartSymbol) -> Self {
        self.symbol.get_or_insert_default().r#type = Some(symbol.as_str().to_string());
        self
    }
}

impl From<&str> for ChartLegendDatum {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for ChartLegendDatum {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

/// Create a list of legend entries from items convertible into one.
pub fn legend_data<T: Into<ChartLegendDatum>>(
    data: impl IntoIterator<Item = T>,
) -> Vec<ChartLegendDatum> {
    data.into_iter().map(Into::into).collect()
}
