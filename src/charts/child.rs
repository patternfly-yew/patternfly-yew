//! Children of charts and groups

use super::element::{BuildContext, ToElement};
use super::*;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew::virtual_dom::VChild;

/// A child of a [`Chart`], [`ChartGroup`] or [`ChartStack`].
#[derive(Clone, Debug, PartialEq)]
pub enum ChartChild {
    Area(Rc<ChartAreaProperties>),
    Axis(Rc<ChartAxisProperties>),
    Bar(Rc<ChartBarProperties>),
    BoxPlot(Rc<ChartBoxPlotProperties>),
    Group(Rc<ChartGroupProperties>),
    Legend(Rc<ChartLegendProperties>),
    Line(Rc<ChartLineProperties>),
    Pie(Rc<ChartPieProperties>),
    Scatter(Rc<ChartScatterProperties>),
    Stack(Rc<ChartStackProperties>),
    Threshold(Rc<ChartThresholdProperties>),
}

impl ToElement for ChartChild {
    fn to_element(&self, ctx: &mut BuildContext) -> JsValue {
        match self {
            Self::Area(props) => props.to_element(ctx),
            Self::Axis(props) => props.to_element(ctx),
            Self::Bar(props) => props.to_element(ctx),
            Self::BoxPlot(props) => props.to_element(ctx),
            Self::Group(props) => props.to_element(ctx),
            Self::Legend(props) => props.to_element(ctx),
            Self::Line(props) => props.to_element(ctx),
            Self::Pie(props) => props.to_element(ctx),
            Self::Scatter(props) => props.to_element(ctx),
            Self::Stack(props) => props.to_element(ctx),
            Self::Threshold(props) => props.to_element(ctx),
        }
    }
}

macro_rules! chart_child {
    ($($variant:ident => $component:ident),* $(,)?) => {
        $(
            impl From<VChild<$component>> for ChartChild {
                fn from(child: VChild<$component>) -> Self {
                    Self::$variant(child.props)
                }
            }
        )*

        impl From<ChartChild> for Html {
            fn from(child: ChartChild) -> Self {
                match child {
                    $(ChartChild::$variant(props) => html!(<$component ..(*props).clone() />),)*
                }
            }
        }
    };
}

chart_child!(
    Area => ChartArea,
    Axis => ChartAxis,
    Bar => ChartBar,
    BoxPlot => ChartBoxPlot,
    Group => ChartGroup,
    Legend => ChartLegend,
    Line => ChartLine,
    Pie => ChartPie,
    Scatter => ChartScatter,
    Stack => ChartStack,
    Threshold => ChartThreshold,
);

/// Append the children to an element.
pub(crate) fn append_children(
    element: &mut element::Element,
    children: &ChildrenRenderer<ChartChild>,
    ctx: &mut BuildContext,
) {
    for child in children.iter() {
        element.child(child.to_element(ctx));
    }
}
