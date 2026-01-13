//! Pagination controls

mod simple;
pub use simple::*;

use crate::prelude::{
    AsClasses, Button, ButtonVariant, ExtendClasses, Icon, TextInput, TextInputType, use_on_enter,
};
use yew::prelude::*;
use yew_hooks::use_click_away;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PaginationPosition {
    Top,
    Bottom,
}

impl AsClasses for PaginationPosition {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Top => {}
            Self::Bottom => classes.push(classes!("pf-m-top")),
        }
    }
}

impl PaginationPosition {
    fn toggle_icon(&self, expanded: bool) -> Icon {
        match (self, expanded) {
            (Self::Bottom, true) => Icon::CaretUp,
            _ => Icon::CaretDown,
        }
    }
}

/// Properties for [`Pagination`]
#[derive(Clone, PartialEq, Properties)]
pub struct PaginationProperties {
    #[prop_or_default]
    pub total_entries: Option<usize>,
    #[prop_or_default]
    pub offset: usize,
    #[prop_or(vec![10,25,50])]
    pub entries_per_page_choices: Vec<usize>,
    #[prop_or(25)]
    pub selected_choice: usize,

    /// Callback for navigation
    #[prop_or_default]
    pub onnavigation: Callback<Navigation>,

    /// Callback for change in limit (page size, per page)
    #[prop_or_default]
    pub onlimit: Callback<usize>,

    /// Element ID
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Additional styles
    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or(PaginationPosition::Top)]
    pub position: PaginationPosition,

    /// Disable the full control
    #[prop_or_default]
    pub disabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Navigation {
    First,
    Previous,
    Next,
    Last,
    /// navigate to a specific page (zero based)
    Page(usize),
}

/// Pagination component.
///
/// > A **pagination** component gives users more navigational capability on pages with content views.
///
/// See: <https://www.patternfly.org/components/pagination>
///
/// ## Properties
///
/// Defined by [`PaginationProperties`].
///
/// ## Example
///
/// See the [PatternFly Quickstart](https://github.com/ctron/patternfly-yew-quickstart) for a complete example.
#[function_component(Pagination)]
pub fn pagination(props: &PaginationProperties) -> Html {
    let expanded = use_state_eq(|| false);

    // The pagination menu : "1-20 of nnn"
    let mut menu_classes = classes!("pf-v5-c-options-menu");
    menu_classes.extend_from(&props.position);

    if *expanded {
        menu_classes.push("pf-m-expanded");
    }

    // if the dataset is empty
    let empty = props
        .total_entries
        .map(|total| total == 0)
        .unwrap_or_default();

    // The default rust div operator does floor(), we need ceil, so we cast to float before doing the operation
    let max_page = props
        .total_entries
        .map(|m| (m as f64 / props.selected_choice as f64).ceil() as usize);

    // the current page
    let current_page = match empty {
        true => 0,
        false => (props.offset as f64 / props.selected_choice as f64).ceil() as usize,
    };

    // if this is the  last page
    let is_last_page = if let Some(max) = props.total_entries {
        props.offset + props.selected_choice >= max
    } else {
        false
    };

    // total entries string
    let total_entries = props
        .total_entries
        .map(|m| format!("{}", m))
        .unwrap_or_else(|| String::from("many"));

    // first entry number (one based)
    let start = match empty {
        true => 0,
        // +1 because humans don't count from 0 :)
        false => props.offset + 1,
    };

    let mut end = props.offset + props.selected_choice;
    if let Some(total) = props.total_entries {
        end = end.min(total);
    }
    let showing = format!("{start} - {end}",);

    let limit_choices = props.entries_per_page_choices.clone();

    // toggle
    let ontoggle = use_callback(expanded.clone(), |_, expanded| {
        expanded.set(!**expanded);
    });

    let node = use_node_ref();
    {
        let expanded = expanded.clone();
        use_click_away(node.clone(), move |_| {
            expanded.set(false);
        });
    }

    // page input field

    // the parsed input (zero based)
    let input = use_state_eq(|| 0);
    // the raw input of the page number field
    let input_text = use_state_eq(|| Some((current_page + 1).to_string()));

    if input_text.is_none() {
        input_text.set(Some((current_page + 1).to_string()));
    }

    let onkeydown = use_on_enter(
        (input.clone(), props.onnavigation.clone(), max_page),
        |(input, onnavigation, max_page)| {
            let mut page: usize = **input;
            if let Some(max_page) = max_page {
                if page > *max_page {
                    page = *max_page;
                }
            }
            // humans start with 1, we use 0.
            page = page.saturating_sub(1);
            log::debug!("Emit page change: {page}");
            onnavigation.emit(Navigation::Page(page));
        },
    );

    let onchange = use_callback(
        (input.clone(), input_text.clone(), max_page, current_page),
        |text: String, (input, input_text, max_page, current_page)| {
            input_text.set(Some(text.clone()));

            let value = match text.parse::<usize>() {
                Ok(value) => {
                    let max_page = max_page.unwrap_or(usize::MAX);
                    if value > 0 && value <= max_page {
                        Some(value)
                    } else {
                        None
                    }
                }
                Err(_) => None,
            };

            if let Some(value) = value {
                input.set(value);
            } else {
                // +1 because humans
                input.set(current_page.saturating_add(1));
            }

            log::debug!("New prepared page value: {:?} / {}", **input_text, **input);
        },
    );

    let onblur = use_callback(input_text.clone(), |_, input_text| {
        input_text.set(None);
    });

    let onnavigation = use_callback(
        (props.onnavigation.clone(), input_text.clone()),
        |nav, (onnavigation, input_text)| {
            input_text.set(None);
            onnavigation.emit(nav);
        },
    );

    // Page number can be changed through props, therefore input_text should watch props
    {
        let input_text = input_text.clone();
        use_effect_with(
            (props.offset, props.selected_choice, props.total_entries),
            move |(offset, selected, total)| {
                let r = (*offset as f64 / *selected as f64).ceil() as usize;

                if *total == Some(0) {
                    input_text.set(Some("0".to_string()));
                } else {
                    input_text.set(Some((r + 1).to_string()));
                }
            },
        );
    }

    // on limit change
    let onlimit = use_callback(
        (props.onlimit.clone(), input_text.clone()),
        |limit, (onlimit, input_text)| {
            input_text.set(None);
            onlimit.emit(limit);
        },
    );

    // The main div
    let pagination_classes = match &props.position {
        PaginationPosition::Top => classes!("pf-v5-c-pagination"),
        PaginationPosition::Bottom => classes!("pf-v5-c-pagination", "pf-m-bottom"),
    };

    let pagination_styles = format!(
        "--pf-v5-c-pagination__nav-page-select--c-form-control--width-chars: {};",
        max_page.unwrap_or_default().to_string().len().clamp(2, 10)
    );

    // render

    let unbound = props.total_entries.is_none();

    html! (

        <div
            id={&props.id}
            class={pagination_classes}
            style={[pagination_styles, props.style.to_string()].join(" ")}
            ref={node}
        >

            // the selector of how many entries per page to display
            <div class="pf-v5-c-pagination__total-items">
                <b>{ showing.clone() }</b> {"\u{00a0}of\u{00a0}"}
                <b>{ total_entries.clone() }</b>
            </div>

            <div class={ menu_classes }>
                <button
                    class="pf-v5-c-options-menu__toggle pf-m-text pf-m-plain"
                    type="button"
                    aria-haspopup="listbox"
                    aria-expanded="true"
                    onclick={ontoggle}
                    disabled={props.disabled}
                >
                    <span class="pf-v5-c-options-menu__toggle-text">
                        <b>{ showing }</b>{"\u{00a0}of\u{00a0}"}
                        <b>{ total_entries }</b>
                    </span>
                    <div class="pf-v5-c-options-menu__toggle-icon">
                        { props.position.toggle_icon(*expanded)}
                    </div>
                </button>

            if *expanded {
                <ul class="pf-v5-c-options-menu__menu" >
                    { for limit_choices.into_iter().map(|limit|  {
                        let expanded = expanded.clone();
                        let onlimit = onlimit.clone();
                        let onclick = Callback::from(move |_|{
                            onlimit.emit(limit);
                            expanded.set(false);
                        });
                        html!(
                            <li>
                                <button
                                    class="pf-v5-c-options-menu__menu-item"
                                    type="button"
                                    {onclick}
                                >
                                    {limit} {" per page"}
                                    if props.selected_choice == limit {
                                        <div class="pf-v5-c-options-menu__menu-item-icon">
                                            { Icon::Check }
                                        </div>
                                    }
                                </button>
                            </li>
                    )})}
                </ul>
            }
            </div>

            // the navigation buttons
            <nav class="pf-v5-c-pagination__nav" aria-label="Pagination">
                <div class="pf-v5-c-pagination__nav-control pf-m-first">
                    <Button
                        variant={ButtonVariant::Plain}
                        onclick={onnavigation.reform(|_|Navigation::First)}
                        disabled={ props.disabled || props.offset == 0 }
                        aria_label="Go to first page"
                    >
                      { Icon::AngleDoubleLeft }
                    </Button>
                </div>
                <div class="pf-v5-c-pagination__nav-control pf-m-prev">
                    <Button
                        aria_label="Go to previous page"
                        variant={ButtonVariant::Plain}
                        onclick={onnavigation.reform(|_|Navigation::Previous)}
                        disabled={ props.disabled || props.offset == 0 }
                    >
                       { Icon::AngleLeft }
                    </Button>
                </div>
                <div class="pf-v5-c-pagination__nav-page-select">
                    <TextInput
                        r#type={TextInputType::Number}
                        inputmode="number"
                        {onchange}
                        {onkeydown}
                        {onblur}
                        value={(*input_text).clone().unwrap_or_else(|| (current_page+1).to_string()) }
                        disabled={ props.disabled || empty }
                    />
                if let Some(max_page) = max_page {
                    <span aria-hidden="true">{ "of "} { max_page }</span>
                }
                </div>

                <div class="pf-v5-c-pagination__nav-control pf-m-next">
                    <Button
                        aria_label="Go to next page"
                        variant={ButtonVariant::Plain}
                        onclick={onnavigation.reform(|_|Navigation::Next)}
                        disabled={ props.disabled || is_last_page }
                    >
                        { Icon::AngleRight }
                    </Button>
                </div>
                <div class="pf-v5-c-pagination__nav-control pf-m-last">
                    <Button
                        aria_label="Go to last page"
                        variant={ButtonVariant::Plain}
                        onclick={onnavigation.reform(|_|Navigation::Last)}
                        disabled={ props.disabled || unbound || is_last_page }
                    >
                        { Icon::AngleDoubleRight }
                    </Button>
                </div>
            </nav>
        </div>
    )
}
