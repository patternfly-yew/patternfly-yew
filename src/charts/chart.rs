//! Chart

use super::child::{ChartChild, append_children};
use super::container::ChartContainer;
use super::element::{BuildContext, Element, ToElement};
use super::legend::{ChartLegendProperties, color_scale};
use super::series::standalone;
use super::types::*;
use serde_json::Value;
use wasm_bindgen::JsValue;
use yew::html::ChildrenRenderer;
use yew::prelude::*;

/// Properties for [`Chart`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ChartProperties {
    /// Axes and series of the chart
    #[prop_or_default]
    pub children: ChildrenRenderer<ChartChild>,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    /// Accessible title
    #[prop_or_default]
    pub aria_title: Option<AttrValue>,
    /// Accessible description
    #[prop_or_default]
    pub aria_desc: Option<AttrValue>,
    /// Height of the chart.
    ///
    /// The chart is responsive: it scales to the width of its parent, keeping the aspect ratio
    /// given by `width` and `height`.
    #[prop_or_default]
    pub height: Option<f64>,
    /// Width of the chart, see `height`.
    #[prop_or_default]
    pub width: Option<f64>,
    #[prop_or_default]
    pub padding: Option<ChartPadding>,
    #[prop_or_default]
    pub theme_color: Option<ChartThemeColor>,
    /// Colors applied to the series, instead of the theme colors
    #[prop_or_default]
    pub color_scale: Vec<AttrValue>,
    /// Render patterns, in addition to colors
    #[prop_or_default]
    pub has_patterns: bool,
    /// Render the axes (true by default)
    #[prop_or_default]
    pub show_axis: Option<bool>,
    /// Entries of the legend
    #[prop_or_default]
    pub legend_data: Vec<ChartLegendDatum>,
    #[prop_or_default]
    pub legend_position: Option<ChartLegendPosition>,
    #[prop_or_default]
    pub legend_orientation: Option<ChartOrientation>,
    /// Allow the legend items to wrap
    #[prop_or_default]
    pub legend_allow_wrap: bool,
    /// Customize the legend
    #[prop_or_default]
    pub legend_component: Option<ChartLegendProperties>,
    #[prop_or_default]
    pub domain: Option<ChartDomain>,
    #[prop_or_default]
    pub min_domain: Option<ChartAxisValue>,
    #[prop_or_default]
    pub max_domain: Option<ChartAxisValue>,
    #[prop_or_default]
    pub domain_padding: Option<ChartAxisValue>,
    #[prop_or_default]
    pub scale: Option<ChartScale>,
    /// Swap the x and y axes
    #[prop_or_default]
    pub horizontal: Option<bool>,
    /// The container, providing interactivity (e.g. tooltips)
    #[prop_or_default]
    pub container_component: Option<ChartContainer>,
    /// Victory style (e.g. `{"background": {"fill": "white"}}`)
    #[prop_or_default]
    pub style: Option<Value>,
    /// Additional raw properties, passed to the PatternFly component
    #[prop_or_default]
    pub extra: Option<Value>,
}

impl ToElement for ChartProperties {
    fn to_element(&self, ctx: &mut BuildContext) -> JsValue {
        let mut element = Element::new("Chart");
        element
            .str("name", &self.name)
            .str("ariaTitle", &self.aria_title)
            .str("ariaDesc", &self.aria_desc)
            .opt("height", self.height)
            .opt("width", self.width)
            .opt_json("padding", &self.padding)
            .opt("themeColor", self.theme_color)
            .list("colorScale", &color_scale(&self.color_scale))
            .opt("hasPatterns", self.has_patterns.then_some(true))
            .opt("showAxis", self.show_axis)
            .list("legendData", &self.legend_data)
            .opt("legendPosition", self.legend_position)
            .opt("legendOrientation", self.legend_orientation)
            .opt("legendAllowWrap", self.legend_allow_wrap.then_some(true))
            .element("legendComponent", &self.legend_component, ctx)
            .opt_json("domain", &self.domain)
            .opt_json("minDomain", &self.min_domain)
            .opt_json("maxDomain", &self.max_domain)
            .opt_json("domainPadding", &self.domain_padding)
            .opt_json("scale", &self.scale)
            .opt("horizontal", self.horizontal)
            .element("containerComponent", &self.container_component, ctx)
            .opt_json("style", &self.style)
            .extra(&self.extra);
        append_children(&mut element, &self.children, ctx);
        element.build()
    }
}

/// Chart
///
/// > The chart component wraps axes and data series (area, bar, line, …), rendering them on
/// > a common coordinate system, together with an optional legend.
///
/// See: <https://www.patternfly.org/charts/about-charts>
///
/// The chart is responsive: it scales to the width of its parent, keeping the aspect ratio
/// given by `width` and `height`. Wrap it in an element with the desired size.
///
/// ## Properties
///
/// Defined by [`ChartProperties`].
///
/// ## Children
///
/// Axes ([`ChartAxis`](super::ChartAxis)), series (e.g. [`ChartBar`](super::ChartBar)), groups
/// ([`ChartGroup`](super::ChartGroup), [`ChartStack`](super::ChartStack)), and legends.
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
///         <div style="height: 250px; width: 600px;">
///             <Chart
///                 aria_title="Pets"
///                 domain_padding={ChartAxisValue::x(50)}
///                 legend_data={legend_data(["Cats", "Dogs"])}
///                 legend_position={ChartLegendPosition::Right}
///                 height={250.0}
///                 width={600.0}
///                 padding={ChartPadding::new(50.0, 200.0, 50.0, 50.0)}
///             >
///                 <ChartAxis />
///                 <ChartAxis dependent_axis=true show_grid=true />
///                 <ChartGroup offset={11.0}>
///                     <ChartBar name="cats" data={chart_data([("2015", 1), ("2016", 2)])} />
///                     <ChartBar name="dogs" data={chart_data([("2015", 2), ("2016", 1)])} />
///                 </ChartGroup>
///             </Chart>
///         </div>
///     )
/// }
/// ```
#[function_component(Chart)]
pub fn chart(props: &ChartProperties) -> Html {
    standalone(props)
}
