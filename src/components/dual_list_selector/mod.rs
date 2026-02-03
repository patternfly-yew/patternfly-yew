//! The dynamic and composable [dual list selector](https://www.patternfly.org/components/dual-list-selector)

use crate::{components::tooltip::TooltipProperties, icon::Icon};
use yew::prelude::*;

mod control;
mod item_renderer;
mod list;
mod pane;

pub use control::*;
pub use item_renderer::*;
pub use list::*;
pub use pane::*;

/// The inputs of the onlistchanged event. Has the corresponding mouse event of the
/// button press, as well as the available and chosen options after the change.
pub type DualListSelectorOnListChangedInputs<T> = (MouseEvent, Vec<T>, Vec<T>);

/// The event causing an option to be selected
#[derive(Debug, Clone, PartialEq)]
pub enum OnOptionSelectEvent {
    Mouse(MouseEvent),
    Keyboard(KeyboardEvent),
}

impl From<MouseEvent> for OnOptionSelectEvent {
    fn from(e: MouseEvent) -> Self {
        Self::Mouse(e)
    }
}

impl From<KeyboardEvent> for OnOptionSelectEvent {
    fn from(e: KeyboardEvent) -> Self {
        Self::Keyboard(e)
    }
}

/// The arguments passed to an onoptionselect event.
#[derive(Debug, Clone, PartialEq)]
pub struct OnOptionSelectArgs {
    pub event: OnOptionSelectEvent,
    pub index: usize,
    pub is_chosen: bool,
}

/// Same as [`OnOptionsSelectArgs`] but without the `chosen` field
/// because that is passed in from the outside.
pub struct OnOptionSelectArgsNoChosen {
    pub event: OnOptionSelectEvent,
    pub index: usize,
}

impl OnOptionSelectArgsNoChosen {
    fn with_chosen(self, is_chosen: bool) -> OnOptionSelectArgs {
        OnOptionSelectArgs {
            event: self.event,
            index: self.index,
            is_chosen,
        }
    }
}

/// Acts as a container for all other DualListSelector sub-components when using a
/// composable dual list selector.
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct DualListSelectorProps<T: DualListSelectorItemRenderer> {
    /// Additional classes applied to the dual list selector.
    #[prop_or_default]
    pub class: Classes,

    /// Title applied to the dynamically built available options pane.
    #[prop_or_default]
    pub available_options_title: Option<AttrValue>,

    /// Status message to display above the dynamically built available options pane.
    #[prop_or_default]
    pub available_options_status: Option<AttrValue>,
    /// Options to display in the dynamically built available options pane.
    #[prop_or_default]
    pub available: Vec<T>,

    /// Title applied to the dynamically built chosen options pane.
    #[prop_or_default]
    pub chosen_options_title: Option<AttrValue>,
    /// Status message to display above the dynamically built chosen options pane.
    #[prop_or_default]
    pub chosen_options_status: Option<AttrValue>,
    /// Options to display in the dynamically built chosen options pane.
    #[prop_or_default]
    pub chosen: Vec<T>,

    /// Tooltip content for the dynamically built add selected button.
    #[prop_or_default]
    pub add_selected_tooltip: Option<AttrValue>,
    /// Additional tooltip properties to the dynamically built add selected tooltip.
    #[prop_or_default]
    pub add_selected_tooltip_props: Option<TooltipProperties>,
    /// Optional callback for the dynamically built add selected button.
    #[prop_or_default]
    pub add_selected: Option<Callback<(Vec<T>, Vec<T>)>>,
    /// Tooltip content for the dynamically built add all button.
    #[prop_or_default]
    pub add_all_available_tooltip: Option<AttrValue>,
    /// Additional tooltip properties to the dynamically built add all tooltip.
    #[prop_or_default]
    pub add_all_available_tooltip_props: Option<TooltipProperties>,
    /// Optional callback for the dynamically built add all button.
    #[prop_or_default]
    pub add_all: Option<Callback<(Vec<T>, Vec<T>)>>,
    /// Tooltip content for the dynamically built remove selected button.
    #[prop_or_default]
    pub remove_selected_tooltip: Option<AttrValue>,
    /// Additional tooltip properties to the dynamically built remove selected tooltip.
    #[prop_or_default]
    pub remove_selected_tooltip_props: Option<TooltipProperties>,
    /// Optional callback for the dynamically built remove selected button.
    #[prop_or_default]
    pub remove_selected: Option<Callback<(Vec<T>, Vec<T>)>>,
    /// Tooltip content for the dynamically built remove all button.
    #[prop_or_default]
    pub remove_all_chosen_tooltip: Option<AttrValue>,
    /// Additional tooltip properties to the dynamically built remove selected tooltip.
    #[prop_or_default]
    pub remove_all_chosen_tooltip_props: Option<TooltipProperties>,
    /// Optional callback for the dynamically built remove all button.
    #[prop_or_default]
    pub remove_all: Option<Callback<(Vec<T>, Vec<T>)>>,

    /// Callback fired every time dynamically built options are chosen or removed.
    /// Inputs are the mouse event as well as the available and chosen options after the change.
    #[prop_or_default]
    pub onlistchange: Option<Callback<DualListSelectorOnListChangedInputs<T>>>,
    /// Optional callback fired when a dynamically built option is selected.
    #[prop_or_default]
    pub onoptionselect: Option<Callback<OnOptionSelectArgs>>,

    /// Flag indicating if the dual list selector is in a disabled state
    #[prop_or_default]
    pub disabled: bool,

    /// Content to be rendered in the dual list selector. Panes & controls will not be built dynamically when children are provided.
    #[prop_or_default]
    pub children: Children,
}

/// The state of the dual list selector.
/// Saves which options exist and which of those are selected
/// for the "available" and "chosen" panels.
///
/// The selected vectors save the indices of the selected items
/// of the options vectors.
#[derive(Debug, Clone)]
struct State<T: DualListSelectorItemRenderer> {
    onlistchange: Option<Callback<DualListSelectorOnListChangedInputs<T>>>,
    available_options: Vec<T>,
    available_options_selected: Vec<usize>,
    chosen_options: Vec<T>,
    chosen_options_selected: Vec<usize>,
    add_selected: Option<Callback<(Vec<T>, Vec<T>)>>,
    add_all: Option<Callback<(Vec<T>, Vec<T>)>>,
    remove_all: Option<Callback<(Vec<T>, Vec<T>)>>,
    remove_selected: Option<Callback<(Vec<T>, Vec<T>)>>,
}

impl<T: DualListSelectorItemRenderer> State<T> {
    pub fn toggle_chosen_option(&mut self, index: usize) {
        Self::toggle_option(&mut self.chosen_options_selected, index);
    }

    pub fn toggle_available_option(&mut self, index: usize) {
        Self::toggle_option(&mut self.available_options_selected, index);
    }

    pub fn add_all_visible(&mut self, e: MouseEvent) {
        Self::move_all(
            &mut self.available_options_selected,
            &mut self.available_options,
            &mut self.chosen_options,
        );
        self.emit_onlistchange(e);
        self.emit_callback(&self.add_all);
    }

    pub fn add_selected(&mut self, e: MouseEvent) {
        Self::move_selected(
            &mut self.available_options_selected,
            &mut self.available_options,
            &mut self.chosen_options,
        );
        self.emit_onlistchange(e);
        self.emit_callback(&self.add_selected);
    }

    pub fn remove_selected(&mut self, e: MouseEvent) {
        Self::move_selected(
            &mut self.chosen_options_selected,
            &mut self.chosen_options,
            &mut self.available_options,
        );
        self.emit_onlistchange(e);
        self.emit_callback(&self.remove_selected);
    }

    pub fn remove_all_visible(&mut self, e: MouseEvent) {
        Self::move_all(
            &mut self.chosen_options_selected,
            &mut self.chosen_options,
            &mut self.available_options,
        );
        self.emit_onlistchange(e);
        self.emit_callback(&self.remove_all);
    }

    fn move_all(src_selected: &mut Vec<usize>, src_options: &mut Vec<T>, dst_options: &mut Vec<T>) {
        dst_options.extend_from_slice(src_options);
        src_options.clear();
        src_selected.clear();
    }

    fn move_selected(
        src_selected: &mut Vec<usize>,
        src_options: &mut Vec<T>,
        dst_options: &mut Vec<T>,
    ) {
        let selected_html = src_selected
            .iter()
            .map(|&idx| src_options[idx].clone())
            .collect::<Vec<T>>();
        dst_options.extend_from_slice(&selected_html);
        src_options.retain(|i| !selected_html.contains(i));
        src_selected.clear();
    }

    fn toggle_option(v: &mut Vec<usize>, elem: usize) {
        match v.iter().position(|&x| x == elem) {
            // Remove from selected
            Some(i) => {
                v.remove(i);
            }
            // Add to selected
            None => v.push(elem),
        }
    }

    fn emit_onlistchange(&self, e: MouseEvent) {
        if let Some(f) = &self.onlistchange {
            f.emit((
                e,
                self.available_options.clone(),
                self.chosen_options.clone(),
            ))
        }
    }

    fn emit_callback(&self, f: &Option<Callback<(Vec<T>, Vec<T>)>>) {
        if let Some(f) = f {
            f.emit((self.available_options.clone(), self.chosen_options.clone()));
        }
    }
}

#[function_component(DualListSelector)]
pub fn dual_list_selector<T: DualListSelectorItemRenderer>(
    props: &DualListSelectorProps<T>,
) -> Html {
    let state = use_state(|| State {
        add_selected: props.add_selected.clone(),
        add_all: props.add_all.clone(),
        remove_all: props.remove_all.clone(),
        remove_selected: props.remove_selected.clone(),
        onlistchange: props.onlistchange.clone(),
        available_options: props.available.clone(),
        available_options_selected: Vec::new(),
        chosen_options: props.chosen.clone(),
        chosen_options_selected: Vec::new(),
    });
    let onoptionselect = {
        let state = state.clone();
        let onoptionselect = props.onoptionselect.clone();
        Callback::from(move |args: OnOptionSelectArgs| {
            let mut new_state = (*state).clone();
            let onoptionselect = onoptionselect.clone();
            if args.is_chosen {
                new_state.toggle_chosen_option(args.index);
            } else {
                new_state.toggle_available_option(args.index);
            }
            state.set(new_state);
            if let Some(f) = onoptionselect {
                f.emit(args.clone())
            }
        })
    };
    let available_options_status = props.available_options_status.clone().unwrap_or_else(|| {
        format!(
            "{} of {} item selected",
            state.available_options_selected.len(),
            state.available_options.len()
        )
        .into()
    });
    let chosen_options_status = props.chosen_options_status.clone().unwrap_or_else(|| {
        format!(
            "{} of {} item selected",
            state.chosen_options_selected.len(),
            state.chosen_options.len()
        )
        .into()
    });
    let control_option = |f: fn(&mut State<T>, MouseEvent)| {
        let state = state.clone();
        Callback::from(move |e| {
            let mut new_state = (*state).clone();
            f(&mut new_state, e);
            state.set(new_state);
        })
    };
    html! {
      <div class={classes!["pf-v6-c-dual-list-selector", props.class.clone()]}>
        if !props.children.is_empty() {
            { props.children.clone() }
        } else {
            <DualListSelectorPane<T>
                title={props.available_options_title.clone()}
                status={available_options_status}
                options={state.available_options.clone()}
                onoptionselect={
                    let onoptionselect = onoptionselect.clone();
                    Callback::from(move |args: OnOptionSelectArgsNoChosen| onoptionselect.emit(args.with_chosen(false)))
                }
                selected_options={state.available_options_selected.clone()}
                disabled={props.disabled}
            />
            <DualListSelectorControlsWrapper>
                <DualListSelectorControl
                    tooltip={props.add_selected_tooltip.clone()}
                    disabled={props.disabled}
                    onclick={control_option(State::add_selected)}
                    tooltip_props={props.add_selected_tooltip_props.clone()}
                >
                    { Icon::AngleRight.with_style("width:1em;display:block;") }
                </DualListSelectorControl>
                <DualListSelectorControl
                    tooltip={props.add_all_available_tooltip.clone()}
                    disabled={props.disabled}
                    onclick={control_option(State::add_all_visible)}
                    tooltip_props={props.add_all_available_tooltip_props.clone()}
                >
                    { Icon::AngleDoubleRight.with_style("width:1em;display:block;") }
                </DualListSelectorControl>
                <DualListSelectorControl
                    tooltip={props.remove_all_chosen_tooltip.clone()}
                    disabled={props.disabled}
                    onclick={control_option(State::remove_all_visible)}
                    tooltip_props={props.remove_all_chosen_tooltip_props.clone()}
                >
                    { Icon::AngleDoubleLeft.with_style("width:1em;display:block;") }
                </DualListSelectorControl>
                <DualListSelectorControl
                    tooltip={props.remove_selected_tooltip.clone()}
                    disabled={props.disabled}
                    onclick={control_option(State::remove_selected)}
                    tooltip_props={props.remove_selected_tooltip_props.clone()}
                >
                    { Icon::AngleLeft.with_style("width:1em;display:block;") }
                </DualListSelectorControl>
            </DualListSelectorControlsWrapper>
            <DualListSelectorPane<T>
                is_chosen=true
                title={props.chosen_options_title.clone()}
                status={chosen_options_status}
                options={state.chosen_options.clone()}
                onoptionselect={
                    let onoptionselect = onoptionselect.clone();
                    Callback::from(move |args: OnOptionSelectArgsNoChosen| onoptionselect.emit(args.with_chosen(true)))
                }
                selected_options={state.chosen_options_selected.clone()}
                disabled={props.disabled}
            />
        }
      </div>
    }
}
