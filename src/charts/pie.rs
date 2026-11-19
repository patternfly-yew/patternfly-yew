//! Pie and donut charts

use super::container::ChartContainer;
use super::element::{BuildContext, Element, ToElement};
use super::label::ChartLabelComponent;
use super::legend::{ChartLegendProperties, color_scale};
use super::series::standalone;
use super::types::*;
use serde_json::Value;
use wasm_bindgen::JsValue;
use yew::prelude::*;

str_enum!(
    /// Position of the subtitle of a donut chart.
    ChartSubTitlePosition {
        Bottom => "bottom",
        Center => "center",
        Right => "right",
    }
);

macro_rules! pie {
    (
        $(#[$meta:meta])*
        $component:ident, $fn:ident, $props:ident, $js:literal,
        { $($(#[$fmeta:meta])* $field:ident: $ty:ty => $key:literal),* $(,)? }
    ) => {
        #[doc = concat!("Properties for [`", stringify!($component), "`]")]
        #[derive(Clone, Debug, PartialEq, Properties)]
        pub struct $props {
            #[prop_or_default]
            pub name: Option<AttrValue>,
            /// Accessible title
            #[prop_or_default]
            pub aria_title: Option<AttrValue>,
            /// Accessible description
            #[prop_or_default]
            pub aria_desc: Option<AttrValue>,
            /// Data points (slices), using `x` as category and `y` as value
            #[prop_or_default]
            pub data: Vec<ChartDatum>,
            /// Labels (tooltips) of the slices
            #[prop_or_default]
            pub labels: Option<ChartLabels>,
            #[prop_or_default]
            pub label_component: Option<ChartLabelComponent>,
            #[prop_or_default]
            pub label_radius: Option<f64>,
            /// Show tooltips (true by default)
            #[prop_or_default]
            pub allow_tooltip: Option<bool>,
            /// Keep tooltips inside the chart
            #[prop_or_default]
            pub constrain_to_visible_area: bool,
            /// Entries of the legend
            #[prop_or_default]
            pub legend_data: Vec<ChartLegendDatum>,
            #[prop_or_default]
            pub legend_position: Option<ChartLegendPosition>,
            #[prop_or_default]
            pub legend_orientation: Option<ChartOrientation>,
            #[prop_or_default]
            pub legend_allow_wrap: bool,
            #[prop_or_default]
            pub legend_component: Option<ChartLegendProperties>,
            #[prop_or_default]
            pub height: Option<f64>,
            #[prop_or_default]
            pub width: Option<f64>,
            #[prop_or_default]
            pub padding: Option<ChartPadding>,
            #[prop_or_default]
            pub radius: Option<f64>,
            #[prop_or_default]
            pub pad_angle: Option<f64>,
            #[prop_or_default]
            pub theme_color: Option<ChartThemeColor>,
            /// Colors applied to the slices, instead of the theme colors
            #[prop_or_default]
            pub color_scale: Vec<AttrValue>,
            /// Render patterns, in addition to colors
            #[prop_or_default]
            pub has_patterns: bool,
            #[prop_or_default]
            pub container_component: Option<ChartContainer>,
            /// Victory style
            #[prop_or_default]
            pub style: Option<Value>,
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
                    .str("ariaTitle", &self.aria_title)
                    .str("ariaDesc", &self.aria_desc)
                    .json("data", &self.data)
                    .element("labelComponent", &self.label_component, ctx)
                    .opt("labelRadius", self.label_radius)
                    .opt("allowTooltip", self.allow_tooltip)
                    .opt("constrainToVisibleArea", self.constrain_to_visible_area.then_some(true))
                    .list("legendData", &self.legend_data)
                    .opt("legendPosition", self.legend_position)
                    .opt("legendOrientation", self.legend_orientation)
                    .opt("legendAllowWrap", self.legend_allow_wrap.then_some(true))
                    .element("legendComponent", &self.legend_component, ctx)
                    .opt("height", self.height)
                    .opt("width", self.width)
                    .opt_json("padding", &self.padding)
                    .opt("radius", self.radius)
                    .opt("padAngle", self.pad_angle)
                    .opt("themeColor", self.theme_color)
                    .list("colorScale", &color_scale(&self.color_scale))
                    .opt("hasPatterns", self.has_patterns.then_some(true))
                    .element("containerComponent", &self.container_component, ctx)
                    .opt_json("style", &self.style)
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

pie!(
    /// Pie chart
    ///
    /// > A **pie chart** is used to display the proportions of a whole.
    ///
    /// See: <https://www.patternfly.org/charts/pie-chart>
    ///
    /// ## Example
    ///
    /// ```rust
    /// use yew::prelude::*;
    /// use patternfly_yew::prelude::*;
    ///
    /// #[function_component(Example)]
    /// fn example() -> Html {
    ///     html!(
    ///         <div style="height: 230px; width: 350px;">
    ///             <ChartPie
    ///                 aria_title="Pets"
    ///                 data={chart_data([("Cats", 35), ("Dogs", 55), ("Birds", 10)])}
    ///                 labels={ChartLabels::from_fn(|d| format!("{}: {}", d.x().unwrap(), d.y().unwrap()))}
    ///                 legend_data={legend_data(["Cats: 35", "Dogs: 55", "Birds: 10"])}
    ///                 legend_position={ChartLegendPosition::Right}
    ///                 padding={ChartPadding::new(20.0, 140.0, 20.0, 20.0)}
    ///                 height={230.0}
    ///                 width={350.0}
    ///             />
    ///         </div>
    ///     )
    /// }
    /// ```
    ChartPie, chart_pie, ChartPieProperties, "ChartPie",
    {}
);

pie!(
    /// Donut chart
    ///
    /// > A **donut chart** is used to display the proportions of a whole, with a title in its center.
    ///
    /// See: <https://www.patternfly.org/charts/donut-chart>
    ChartDonut, chart_donut, ChartDonutProperties, "ChartDonut",
    {
        /// Title, rendered in the center
        title: Option<AttrValue> => "title",
        sub_title: Option<AttrValue> => "subTitle",
        sub_title_position: Option<ChartSubTitlePosition> => "subTitlePosition",
        inner_radius: Option<f64> => "innerRadius",
        cap_height: Option<f64> => "capHeight",
    }
);
