//! Group and stack

use super::child::{ChartChild, append_children};
use super::element::{BuildContext, Element, ToElement};
use super::label::ChartLabelComponent;
use super::legend::color_scale;
use super::series::standalone;
use super::types::*;
use serde_json::Value;
use wasm_bindgen::JsValue;
use yew::html::ChildrenRenderer;
use yew::prelude::*;

macro_rules! group {
    (
        $(#[$meta:meta])*
        $component:ident, $fn:ident, $props:ident, $js:literal,
        { $($(#[$fmeta:meta])* $field:ident: $ty:ty => $key:literal),* $(,)? }
    ) => {
        #[doc = concat!("Properties for [`", stringify!($component), "`]")]
        #[derive(Clone, Debug, PartialEq, Properties)]
        pub struct $props {
            /// The series of the group
            #[prop_or_default]
            pub children: ChildrenRenderer<ChartChild>,
            #[prop_or_default]
            pub name: Option<AttrValue>,
            /// Accessible title, when used standalone
            #[prop_or_default]
            pub aria_title: Option<AttrValue>,
            /// Accessible description, when used standalone
            #[prop_or_default]
            pub aria_desc: Option<AttrValue>,
            /// Colors applied to the series
            #[prop_or_default]
            pub color_scale: Vec<AttrValue>,
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
            /// Labels of the data points
            #[prop_or_default]
            pub labels: Option<ChartLabels>,
            /// The component rendering the labels (e.g. a tooltip)
            #[prop_or_default]
            pub label_component: Option<ChartLabelComponent>,
            /// Height, when used standalone
            #[prop_or_default]
            pub height: Option<f64>,
            /// Width, when used standalone
            #[prop_or_default]
            pub width: Option<f64>,
            /// Padding, when used standalone
            #[prop_or_default]
            pub padding: Option<ChartPadding>,
            /// Container, when used standalone
            #[prop_or_default]
            pub container_component: Option<super::ChartContainer>,
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
                    .list("colorScale", &color_scale(&self.color_scale))
                    .opt("themeColor", self.theme_color)
                    .opt("horizontal", self.horizontal)
                    .opt_json("domain", &self.domain)
                    .opt_json("minDomain", &self.min_domain)
                    .opt_json("maxDomain", &self.max_domain)
                    .opt_json("domainPadding", &self.domain_padding)
                    .element("labelComponent", &self.label_component, ctx)
                    .opt("height", self.height)
                    .opt("width", self.width)
                    .opt_json("padding", &self.padding)
                    .element("containerComponent", &self.container_component, ctx)
                    .opt_json("style", &self.style)
                    $(.opt($key, self.$field.clone()))*
                    .extra(&self.extra);
                append_children(&mut element, &self.children, ctx);
                element.build()
            }
        }

        $(#[$meta])*
        ///
        /// ## Properties
        ///
        #[doc = concat!("Defined by [`", stringify!($props), "`].")]
        ///
        /// ## Children
        ///
        /// Series components, like [`ChartBar`](super::ChartBar) or [`ChartArea`](super::ChartArea).
        #[function_component($component)]
        pub fn $fn(props: &$props) -> Html {
            standalone(props)
        }
    };
}

group!(
    /// Chart group
    ///
    /// Groups series, sharing a common configuration. For example, displaying bars side by side
    /// (using `offset`), or rendering a sparkline when used standalone.
    ///
    /// See: <https://www.patternfly.org/charts/sparkline-chart>
    ChartGroup, chart_group, ChartGroupProperties, "ChartGroup",
    {
        /// Offset between the series (e.g. for grouped bars)
        offset: Option<f64> => "offset",
    }
);

group!(
    /// Chart stack
    ///
    /// > A **stack chart** is used to display the composition of values, stacking series on top of each other.
    ///
    /// See: <https://www.patternfly.org/charts/stack-chart>
    ChartStack, chart_stack, ChartStackProperties, "ChartStack",
    {
        /// Offset between stacks (when multiple stacks are grouped)
        x_offset: Option<f64> => "xOffset",
    }
);
