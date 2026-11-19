//! Charts
//!
//! The [PatternFly charts](https://www.patternfly.org/charts/about-charts), which are built on
//! top of [Victory](https://commerce.nearform.com/open-source/victory/).
//!
//! Requires the `charts` feature.
//!
//! ## Implementation
//!
//! The components are rendered by the Victory based PatternFly React charts
//! (`@patternfly/react-charts/victory`). Those, together with React, are bundled into a JavaScript
//! module shipped with this crate, and loaded automatically through `wasm-bindgen`. There is no
//! need to add any script to your application.
//!
//! Each Yew component maps to the PatternFly component of the same name, with properties using
//! the snake case version of their names. Properties which are not (yet) mapped can be provided
//! using the `extra` property (a JSON object).
//!
//! Charts are responsive by default: they scale to the width of their parent, keeping the aspect
//! ratio given by their `width` and `height`.
//!
//! ## Styling
//!
//! The chart themes come with built-in defaults. To have the charts follow the PatternFly tokens
//! (e.g. dark theme), also include the `patternfly-charts.css` stylesheet of PatternFly.
//!
//! ## Rebuilding the JavaScript bundle
//!
//! The bundle is generated from the `js/charts` directory of the repository:
//! `npm ci && npm run build`.

mod axis;
mod bullet;
mod chart;
mod child;
mod container;
mod donut_utilization;
mod element;
mod group;
mod host;
mod label;
mod legend;
mod pie;
mod series;
mod sys;
mod types;

pub use axis::*;
pub use bullet::*;
pub use chart::*;
pub use child::ChartChild;
pub use container::*;
pub use donut_utilization::*;
pub use group::*;
pub use label::*;
pub use legend::*;
pub use pie::*;
pub use series::{
    ChartArea, ChartAreaProperties, ChartBar, ChartBarProperties, ChartBoxPlot,
    ChartBoxPlotProperties, ChartLine, ChartLineProperties, ChartScatter, ChartScatterProperties,
    ChartThreshold, ChartThresholdProperties,
};
pub use types::*;
