//! Data series: area, bar, box plot, line, scatter and threshold

use super::element::{BuildContext, Element, ToElement};
use super::host::ChartHost;
use super::label::ChartLabelComponent;
use super::types::*;
use serde_json::Value;
use wasm_bindgen::JsValue;
use yew::prelude::*;

/// Render a chart element standalone.
pub(crate) fn standalone<P>(props: &P) -> Html
where
    P: ToElement + Clone + PartialEq + 'static,
{
    html!(<ChartHost<P> element={props.clone()} />)
}

macro_rules! series {
    (
        $(#[$meta:meta])*
        $component:ident, $fn:ident, $props:ident, $js:literal,
        { $($(#[$fmeta:meta])* $field:ident: $ty:ty => $key:literal),* $(,)? }
    ) => {
        #[doc = concat!("Properties for [`", stringify!($component), "`]")]
        #[derive(Clone, Debug, PartialEq, Properties)]
        pub struct $props {
            /// Name of the series, used to reference it (e.g. in a container blacklist)
            #[prop_or_default]
            pub name: Option<AttrValue>,
            /// Data points
            #[prop_or_default]
            pub data: Vec<ChartDatum>,
            /// Labels of the data points
            #[prop_or_default]
            pub labels: Option<ChartLabels>,
            /// The component rendering the labels (e.g. a tooltip)
            #[prop_or_default]
            pub label_component: Option<ChartLabelComponent>,
            /// Victory style (e.g. `{"data": {"fill": "red"}}`)
            #[prop_or_default]
            pub style: Option<Value>,
            #[prop_or_default]
            pub theme_color: Option<ChartThemeColor>,
            #[prop_or_default]
            pub horizontal: Option<bool>,
            #[prop_or_default]
            pub domain: Option<ChartDomain>,
            #[prop_or_default]
            pub min_domain: Option<ChartAxisValue>,
            #[prop_or_default]
            pub max_domain: Option<ChartAxisValue>,
            #[prop_or_default]
            pub domain_padding: Option<ChartAxisValue>,
            /// Height, when used standalone
            #[prop_or_default]
            pub height: Option<f64>,
            /// Width, when used standalone
            #[prop_or_default]
            pub width: Option<f64>,
            /// Padding, when used standalone
            #[prop_or_default]
            pub padding: Option<ChartPadding>,
            $(
                $(#[$fmeta])*
                #[prop_or_default]
                pub $field: $ty,
            )*
            /// Additional raw properties, passed to the PatternFly component
            #[prop_or_default]
            pub extra: Option<Value>,
        }

        impl ToElement for $props {
            fn to_element(&self, ctx: &mut BuildContext) -> JsValue {
                let mut element = Element::new($js);
                if let Some(labels) = &self.labels {
                    element.set("labels", labels.to_js(ctx));
                }
                element
                    .str("name", &self.name)
                    .json("data", &self.data)
                    .element("labelComponent", &self.label_component, ctx)
                    .opt_json("style", &self.style)
                    .opt("themeColor", self.theme_color)
                    .opt("horizontal", self.horizontal)
                    .opt_json("domain", &self.domain)
                    .opt_json("minDomain", &self.min_domain)
                    .opt_json("maxDomain", &self.max_domain)
                    .opt_json("domainPadding", &self.domain_padding)
                    .opt("height", self.height)
                    .opt("width", self.width)
                    .opt_json("padding", &self.padding)
                    $(.opt($key, self.$field.clone()))*
                    .extra(&self.extra)
                    .build()
            }
        }

        $(#[$meta])*
        ///
        /// ## Properties
        ///
        #[doc = concat!("Defined by [`", stringify!($props), "`].")]
        #[function_component($component)]
        pub fn $fn(props: &$props) -> Html {
            standalone(props)
        }
    };
}

series!(
    /// Area chart series
    ///
    /// > An **area chart** is used to display the development of quantitative values over an interval or time period.
    ///
    /// See: <https://www.patternfly.org/charts/area-chart>
    ///
    /// Usually used as child of a [`Chart`](super::Chart), it can also be used standalone
    /// (e.g. for sparklines, inside a [`ChartGroup`](super::ChartGroup)).
    ChartArea, chart_area, ChartAreaProperties, "ChartArea",
    {
        interpolation: Option<ChartInterpolation> => "interpolation",
    }
);

series!(
    /// Bar chart series
    ///
    /// > A **bar chart** is used to compare values across categories.
    ///
    /// See: <https://www.patternfly.org/charts/bar-chart>
    ChartBar, chart_bar, ChartBarProperties, "ChartBar",
    {
        bar_width: Option<f64> => "barWidth",
        bar_ratio: Option<f64> => "barRatio",
        alignment: Option<ChartAlignment> => "alignment",
    }
);

series!(
    /// Box plot series
    ///
    /// > A **box plot chart** is used to visualize the distribution of a data set.
    ///
    /// See: <https://www.patternfly.org/charts/box-plot-chart>
    ///
    /// Data points provide multiple `y` values, see [`ChartDatum::values`].
    ChartBoxPlot, chart_box_plot, ChartBoxPlotProperties, "ChartBoxPlot",
    {
        box_width: Option<f64> => "boxWidth",
        /// Show labels for the maximum values
        max_labels: Option<bool> => "maxLabels",
        /// Show labels for the median values
        median_labels: Option<bool> => "medianLabels",
        /// Show labels for the minimum values
        min_labels: Option<bool> => "minLabels",
        /// Show labels for the first quartile values
        q1_labels: Option<bool> => "q1Labels",
        /// Show labels for the third quartile values
        q3_labels: Option<bool> => "q3Labels",
        label_orientation: Option<ChartTooltipOrientation> => "labelOrientation",
    }
);

series!(
    /// Line chart series
    ///
    /// > A **line chart** is used to display the development of quantitative values over an interval or time period.
    ///
    /// See: <https://www.patternfly.org/charts/line-chart>
    ChartLine, chart_line, ChartLineProperties, "ChartLine",
    {
        interpolation: Option<ChartInterpolation> => "interpolation",
    }
);

series!(
    /// Scatter chart series
    ///
    /// > A **scatter chart** is used to display the relationship between two variables.
    ///
    /// See: <https://www.patternfly.org/charts/scatter-chart>
    ChartScatter, chart_scatter, ChartScatterProperties, "ChartScatter",
    {
        /// Size of the symbols
        size: Option<f64> => "size",
        symbol: Option<ChartSymbol> => "symbol",
    }
);

series!(
    /// Threshold series
    ///
    /// > A **threshold chart** is used to illustrate a limit, as a dashed line.
    ///
    /// See: <https://www.patternfly.org/charts/threshold-chart>
    ChartThreshold, chart_threshold, ChartThresholdProperties, "ChartThreshold",
    {
        interpolation: Option<ChartInterpolation> => "interpolation",
    }
);
