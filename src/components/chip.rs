//! Chip
use crate::ouia;
use crate::prelude::{Button, ButtonVariant, Icon, OuiaComponentType};
use crate::utils::{Ouia, OuiaSafe};
use std::fmt::Debug;
use yew::prelude::*;
use yew::virtual_dom::Key;

const OUIA: Ouia = ouia!("Chip");

/// Properties for [`Chip`]
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

    /// OUIA Component id
    #[prop_or_default]
    pub ouia_id: Option<String>,
    /// OUIA Component Type
    #[prop_or(OUIA.component_type())]
    pub ouia_type: OuiaComponentType,
    /// OUIA Component Safe
    #[prop_or(OuiaSafe::TRUE)]
    pub ouia_safe: OuiaSafe,
}

/// Chip component
///
/// > A **chip** is used to communicate a value or a set of attribute-value pairs within workflows that involve filtering a set of objects.
///
/// See: <https://www.patternfly.org/components/chip>
///
/// ## Properties
///
/// Defined by [`ChipProperties`].
#[function_component(Chip)]
pub fn chip(props: &ChipProperties) -> Html {
    let ouia_id = use_memo(props.ouia_id.clone(), |id| {
        id.clone().unwrap_or(OUIA.generated_id())
    });
    let mut classes = Classes::from("pf-v5-c-chip");

    if props.draggable {
        classes.push("pf-m-draggable");
    }

    // this is only used in the chip group component
    if props.overflow {
        classes.push("pf-m-overflow");
    }

    let body = html! {
        <>
            <span class="pf-v5-c-chip__content">
                { render_icon(props) }
                <span class="pf-v5-c-chip__text">{props.text.clone()}</span>
                { render_badge(props) }
                { render_close(props) }
            </span>
        </>
    };
    let component = if props.overflow { "button" } else { "div" };

    html! {
        <@{component}
            class={classes}
            data-ouia-component-id={(*ouia_id).clone()}
            data-ouia-component-type={props.ouia_type}
            data-ouia-safe={props.ouia_safe}
        >
            {body}
        </@>
    }
}

fn render_icon(props: &ChipProperties) -> Html {
    html! (
        if let Some(icon) = &props.icon {
            <span class="pf-v5-c-chip__icon"> { icon.as_html() } </span>
        }
    )
}

fn render_badge(props: &ChipProperties) -> Html {
    html! (
        if let Some(badge) = &props.badge {
            <span class="pf-v5-c-badge pf-m-read"> {badge} </span>
        }
    )
}

fn render_close(props: &ChipProperties) -> Html {
    html! (
        if let Some(onclose) = &props.onclose {
            <span class="pf-v5-c-chip__actions">
                <Button variant={ButtonVariant::Plain} icon={Icon::Times} onclick={onclose.reform(|e: MouseEvent| {
                    // This should work, but right now doesn't due to yewstack/yew#3041
                    e.stop_propagation();
                })} />
            </span>
        }
    )
}

// -------------------------------------------------------------
// Example: ChipDragExample - Demonstrates drag & drop behavior
// -------------------------------------------------------------
use web_sys::DragEvent;

#[function_component(ChipDragExample)]
pub fn chip_drag_example() -> Html {
    let chips = use_state(|| vec!["Foo".to_string(), "Bar".to_string(), "Baz".to_string()]);
    let drag_index = use_state(|| None::<usize>);
    let drop_target = use_state(|| None::<usize>);

    let ondragstart = {
        let drag_index = drag_index.clone();
        move |idx: usize| {
            let drag_index = drag_index.clone();
            Callback::from(move |e: DragEvent| {
                e.stop_propagation();
                drag_index.set(Some(idx));
                if let Some(dt) = e.data_transfer() {
                    dt.set_effect_allowed("move");
                }
            })
        }
    };

    let ondragenter = {
        let drop_target = drop_target.clone();
        move |idx: usize| {
            let drop_target = drop_target.clone();
            Callback::from(move |e: DragEvent| {
                e.prevent_default();
                e.stop_propagation();
                drop_target.set(Some(idx));
            })
        }
    };

    let ondragover = {
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            e.stop_propagation();
        })
    };

    let ondrop = {
        let chips = chips.clone();
        let drag_index = drag_index.clone();
        let drop_target = drop_target.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            e.stop_propagation();

            if let (Some(from), Some(to)) = (*drag_index, *drop_target) {
                if from != to {
                    let mut new_chips = (*chips).clone();
                    let item = new_chips.remove(from);
                    let insert_pos = to;
                    new_chips.insert(insert_pos, item);
                    chips.set(new_chips);
                }
            }

            drag_index.set(None);
            drop_target.set(None);
        })
    };

    let ondragend = {
        let drag_index = drag_index.clone();
        let drop_target = drop_target.clone();
        Callback::from(move |_: DragEvent| {
            drag_index.set(None);
            drop_target.set(None);
        })
    };

    html! {
        <div style="display: flex; gap: 0.5rem; flex-wrap: wrap; padding: 1rem; min-height: 60px;">
            { for chips.iter().enumerate().map(|(idx, text)| {
                let is_dragging = *drag_index == Some(idx);
                let is_drop_target = *drop_target == Some(idx) && *drag_index != Some(idx);

                let mut wrapper_style = "display: inline-block; transition: all 0.2s ease;".to_string();

                if is_dragging {
                    wrapper_style.push_str(" opacity: 0.4;");
                }

                if is_drop_target {
                    wrapper_style.push_str(" transform: translateX(4px);");
                }

                html! {
                    <div
                        key={Key::from(format!("{}-{}", idx, text))}
                        draggable="true"
                        ondragstart={ondragstart(idx)}
                        ondragenter={ondragenter(idx)}
                        ondragover={ondragover.clone()}
                        ondrop={ondrop.clone()}
                        ondragend={ondragend.clone()}
                        style={wrapper_style}
                    >
                        <Chip text={text.clone()} />
                    </div>
                }
            }) }
        </div>
    }
}
