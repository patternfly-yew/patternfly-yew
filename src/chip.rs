use crate::{Button, Icon, Variant};
use std::fmt::Debug;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ChipProperties {
    #[prop_or_default]
    pub text: String,
    #[prop_or_default]
    pub badge: Option<String>,
    #[prop_or_default]
    pub overflow: bool,
    #[prop_or_default]
    pub draggable: bool,
    #[prop_or_default]
    pub onclose: Option<Callback<()>>,
    #[prop_or_default]
    pub icon: Option<Icon>,
}

/// The Chip component.
///
/// > A **chip** is used to communicate a value or a set of attribute-value pairs within workflows that involve filtering a set of objects.
///
/// See: https://www.patternfly.org/v4/components/chip
///
/// ## Properties
///
/// Defined by [`ChipProperties`].
pub struct Chip {}

impl Component for Chip {
    type Message = ();
    type Properties = ChipProperties;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-c-chip");

        if ctx.props().draggable {
            classes.push("pf-m-draggable");
        }

        // this is only used in the chip group component
        if ctx.props().overflow {
            classes.push("pf-m-overflow");
        }

        let body = html! {
            <>
                { self.render_icon(ctx) }
                <span class="pf-c-chip__text">{ctx.props().text.clone()}</span>
                { self.render_badge(ctx) }
                { self.render_close(ctx) }
            </>
        };

        if ctx.props().overflow {
            html! {<button class={classes}>{body}</button>}
        } else {
            html! {<div class={classes}>{body}</div>}
        }
    }
}

impl Chip {
    fn render_icon(&self, ctx: &Context<Self>) -> Html {
        if let Some(icon) = &ctx.props().icon {
            html! {
                <span class="pf-c-chip__icon">
                    { icon.as_html() }
                </span>
            }
        } else {
            html! {}
        }
    }

    fn render_badge(&self, ctx: &Context<Self>) -> Html {
        if let Some(badge) = &ctx.props().badge {
            return html! {
                <span class="pf-c-badge pf-m-read">{badge}</span>
            };
        } else {
            return html! {};
        }
    }

    fn render_close(&self, ctx: &Context<Self>) -> Html {
        if let Some(onclose) = &ctx.props().onclose {
            let onclose = onclose.reform(|_| {});
            return html! {
                <Button variant={Variant::Plain} icon={Icon::Times} onclick={onclose}/>
            };
        } else {
            return html! {};
        }
    }
}
