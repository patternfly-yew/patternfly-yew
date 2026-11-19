//! Bullet chart

use super::element::{BuildContext, Element, ToElement};
use super::legend::ChartLegendProperties;
use super::series::standalone;
use super::types::*;
use serde_json::Value;
use wasm_bindgen::JsValue;
use yew::prelude::*;

str_enum!(
    /// Position of the title of a bullet chart.
    ChartBulletTitlePosition {
        Left => "left",
        TopLeft => "top-left",
    }
);

/// Properties for [`ChartBullet`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ChartBulletProperties {
    #[prop_or_default]
    pub name: Option<AttrValue>,
    /// Accessible title
    #[prop_or_default]
    pub aria_title: Option<AttrValue>,
    /// Accessible description
    #[prop_or_default]
    pub aria_desc: Option<AttrValue>,
    #[prop_or_default]
    pub title: Option<AttrValue>,
    #[prop_or_default]
    pub sub_title: Option<AttrValue>,
    #[prop_or_default]
    pub title_position: Option<ChartBulletTitlePosition>,
    #[prop_or_default]
    pub group_title: Option<AttrValue>,
    #[prop_or_default]
    pub group_sub_title: Option<AttrValue>,

    /// Comparative error measure (e.g. a red line)
    #[prop_or_default]
    pub comparative_error_measure_data: Vec<ChartDatum>,
    #[prop_or_default]
    pub comparative_error_measure_legend_data: Vec<ChartLegendDatum>,
    /// Comparative warning measure (e.g. a yellow line)
    #[prop_or_default]
    pub comparative_warning_measure_data: Vec<ChartDatum>,
    #[prop_or_default]
    pub comparative_warning_measure_legend_data: Vec<ChartLegendDatum>,
    /// Primary measure, rendered as dots
    #[prop_or_default]
    pub primary_dot_measure_data: Vec<ChartDatum>,
    #[prop_or_default]
    pub primary_dot_measure_legend_data: Vec<ChartLegendDatum>,
    /// Primary measure, rendered as bar segments
    #[prop_or_default]
    pub primary_segmented_measure_data: Vec<ChartDatum>,
    #[prop_or_default]
    pub primary_segmented_measure_legend_data: Vec<ChartLegendDatum>,
    /// Qualitative ranges, rendered as background
    #[prop_or_default]
    pub qualitative_range_data: Vec<ChartDatum>,
    #[prop_or_default]
    pub qualitative_range_legend_data: Vec<ChartLegendDatum>,

    /// Labels (tooltips) of the measures
    #[prop_or_default]
    pub labels: Option<ChartLabels>,
    #[prop_or_default]
    pub allow_tooltip: Option<bool>,
    #[prop_or_default]
    pub constrain_to_visible_area: bool,
    #[prop_or_default]
    pub legend_position: Option<ChartLegendPosition>,
    #[prop_or_default]
    pub legend_orientation: Option<ChartOrientation>,
    #[prop_or_default]
    pub legend_items_per_row: Option<u32>,
    #[prop_or_default]
    pub legend_allow_wrap: bool,
    #[prop_or_default]
    pub legend_component: Option<ChartLegendProperties>,
    #[prop_or_default]
    pub min_domain: Option<ChartAxisValue>,
    #[prop_or_default]
    pub max_domain: Option<ChartAxisValue>,
    #[prop_or_default]
    pub domain: Option<ChartDomain>,
    /// Render vertically, instead of horizontally
    #[prop_or_default]
    pub horizontal: Option<bool>,
    /// Invert the qualitative range colors
    #[prop_or_default]
    pub invert: bool,
    /// Size of the bullet
    #[prop_or_default]
    pub bullet_size: Option<f64>,
    #[prop_or_default]
    pub height: Option<f64>,
    #[prop_or_default]
    pub width: Option<f64>,
    #[prop_or_default]
    pub padding: Option<ChartPadding>,
    #[prop_or_default]
    pub theme_color: Option<ChartThemeColor>,
    /// Additional raw properties, passed to the PatternFly component
    #[prop_or_default]
    pub extra: Option<Value>,
}

impl ToElement for ChartBulletProperties {
    fn to_element(&self, ctx: &mut BuildContext) -> JsValue {
        let mut element = Element::new("ChartBullet");
        if let Some(labels) = &self.labels {
            element.set("labels", labels.to_js(ctx));
        }
        element
            .str("name", &self.name)
            .str("ariaTitle", &self.aria_title)
            .str("ariaDesc", &self.aria_desc)
            .str("title", &self.title)
            .str("subTitle", &self.sub_title)
            .opt("titlePosition", self.title_position)
            .str("groupTitle", &self.group_title)
            .str("groupSubTitle", &self.group_sub_title)
            .list(
                "comparativeErrorMeasureData",
                &self.comparative_error_measure_data,
            )
            .list(
                "comparativeErrorMeasureLegendData",
                &self.comparative_error_measure_legend_data,
            )
            .list(
                "comparativeWarningMeasureData",
                &self.comparative_warning_measure_data,
            )
            .list(
                "comparativeWarningMeasureLegendData",
                &self.comparative_warning_measure_legend_data,
            )
            .list("primaryDotMeasureData", &self.primary_dot_measure_data)
            .list(
                "primaryDotMeasureLegendData",
                &self.primary_dot_measure_legend_data,
            )
            .list(
                "primarySegmentedMeasureData",
                &self.primary_segmented_measure_data,
            )
            .list(
                "primarySegmentedMeasureLegendData",
                &self.primary_segmented_measure_legend_data,
            )
            .list("qualitativeRangeData", &self.qualitative_range_data)
            .list(
                "qualitativeRangeLegendData",
                &self.qualitative_range_legend_data,
            )
            .opt("allowTooltip", self.allow_tooltip)
            .opt(
                "constrainToVisibleArea",
                self.constrain_to_visible_area.then_some(true),
            )
            .opt("legendPosition", self.legend_position)
            .opt("legendOrientation", self.legend_orientation)
            .opt("legendItemsPerRow", self.legend_items_per_row)
            .opt("legendAllowWrap", self.legend_allow_wrap.then_some(true))
            .element("legendComponent", &self.legend_component, ctx)
            .opt_json("minDomain", &self.min_domain)
            .opt_json("maxDomain", &self.max_domain)
            .opt_json("domain", &self.domain)
            .opt("horizontal", self.horizontal)
            .opt("invert", self.invert.then_some(true))
            .opt("bulletSize", self.bullet_size)
            .opt("height", self.height)
            .opt("width", self.width)
            .opt_json("padding", &self.padding)
            .opt("themeColor", self.theme_color)
            .extra(&self.extra)
            .build()
    }
}

/// Bullet chart
///
/// > A **bullet chart** is used to compare a primary measure against comparative measures and
/// > qualitative ranges (e.g. poor, satisfactory, good).
///
/// See: <https://www.patternfly.org/charts/bullet-chart>
///
/// ## Properties
///
/// Defined by [`ChartBulletProperties`].
#[function_component(ChartBullet)]
pub fn chart_bullet(props: &ChartBulletProperties) -> Html {
    standalone(props)
}
