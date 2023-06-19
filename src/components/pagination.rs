//! Pagination controls
use crate::{
    next::TextInput, on_enter, Button, ButtonVariant, Icon, InputState, ValidationContext,
    Validator,
};
use yew::prelude::*;
use yew_hooks::use_click_away;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PaginationPosition {
    Top,
    Bottom,
}

impl PaginationPosition {
    fn menu_classes(&self) -> Classes {
        match self {
            PaginationPosition::Top => classes!("pf-c-options-menu"),
            PaginationPosition::Bottom => classes!("pf-c-options-menu", "pf-m-top"),
        }
    }

    fn toggle_icon(&self, expanded: bool) -> Icon {
        match (self, expanded) {
            (PaginationPosition::Bottom, true) => Icon::CaretUp,
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

    /// Callback for change in limit
    #[prop_or_default]
    pub onlimit: Callback<usize>,

    #[prop_or_default]
    pub id: AttrValue,
    /// additional styles
    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or(PaginationPosition::Top)]
    pub position: PaginationPosition,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Navigation {
    First,
    Previous,
    Next,
    Last,
    Page(usize),
}

/// Pagination component.
///
/// > A **pagination** component gives users more navigational capability on pages with content views.
///
/// See: <https://www.patternfly.org/v4/components/pagination>
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
    let mut menu_classes = props.position.menu_classes();
    if *expanded {
        menu_classes.push("pf-m-expanded");
    }

    // The default rust div operator does floor(), we need ceil, so we cast to float before doing the operation
    let max_page = props
        .total_entries
        .map(|m| (m as f64 / props.selected_choice as f64).ceil() as usize);
    let current_page = (props.offset as f64 / props.selected_choice as f64).ceil() as usize;

    let is_last_page = if let Some(max) = props.total_entries {
        props.offset + props.selected_choice > max
    } else {
        false
    };

    let total_entries = props
        .total_entries
        .map(|m| format!("{}", m))
        .unwrap_or_else(|| String::from("unknown"));
    // +1 because humans don't count from 0 :)
    let showing = format!(
        "{} - {}",
        props.offset + 1,
        props.offset + props.selected_choice
    );

    let limit_choices = props.entries_per_page_choices.clone();

    // todo also add max page
    let page_number_field_validator =
        Validator::from(
            |ctx: ValidationContext<String>| match ctx.value.parse::<usize>() {
                Ok(value) => {
                    if value > 0 {
                        InputState::Default
                    } else {
                        InputState::Error
                    }
                }
                Err(_) => InputState::Error,
            },
        );

    // toggle

    let ontoggle = {
        let expanded = expanded.clone();
        Callback::from(move |_| {
            expanded.set(!*expanded);
        })
    };

    let node = use_node_ref();
    {
        let expanded = expanded.clone();
        use_click_away(node.clone(), move |_| {
            expanded.set(false);
        });
    }

    // page input field

    // the state of the input
    let input_state = use_state_eq(InputState::default);
    // the parsed input
    let input = use_state_eq(|| 0);
    // the raw input of the page number field
    let input_text = use_state_eq(|| Some((current_page + 1).to_string()));
    if input_text.is_none() {
        input_text.set(Some((current_page + 1).to_string()));
    }

    let onkeydown = {
        let input = input.clone();
        let onnavigation = props.onnavigation.clone();
        on_enter(move || {
            onnavigation.emit(Navigation::Page(*input));
        })
    };

    let oninput = {
        let input = input.clone();
        let input_text = input_text.clone();
        let page_number_field_validator = page_number_field_validator.clone();
        let input_state = input_state.clone();
        Callback::from(move |text: String| {
            input_text.set(Some(text.clone()));

            let state = page_number_field_validator
                .run(ValidationContext::from(text.clone()))
                .unwrap_or_default();

            if let InputState::Default = &state {
                input.set(text.parse().unwrap_or_default());
            }

            input_state.set(state);
        })
    };

    let onnavigation = {
        let onnavigation = props.onnavigation.clone();
        let input_text = input_text.clone();
        Callback::from(move |nav| {
            input_text.set(None);
            onnavigation.emit(nav);
        })
    };

    // Page number can be changed through props, therefore input_text should watch props
    {
        let input_text = input_text.clone();
        use_effect_with_deps(
            move |tuple| {
                let r = (tuple.0 as f64 / tuple.1 as f64).ceil() as usize;
                input_text.set(Some((r + 1).to_string()));
            },
            (props.offset, props.selected_choice),
        );
    }

    // on limit change

    let onlimit = {
        let onlimit = props.onlimit.clone();
        let input_text = input_text.clone();
        Callback::from(move |limit| {
            input_text.set(None);
            onlimit.emit(limit);
        })
    };

    // render

    html! (

        <div
            id={&props.id}
            class="pf-c-pagination"
            style={&props.style}
            ref={node}
        >

            // the selector of how many entries per page to display
            <div class="pf-c-pagination__total-items">
                <b>{ showing.clone() }</b> {"\u{00a0}of\u{00a0}"}
                <b>{ total_entries.clone() }</b>
            </div>

            <div class={ menu_classes }>
                <div class="pf-c-options-menu__toggle pf-m-text pf-m-plain">
                    <Button
                        class="pf-c-options-menu__toggle-button"
                        aria_haspopup="listbox"
                        aria_label="Items per page"
                        onclick={ontoggle}
                    >
                        <span class="pf-c-options-menu__toggle-text">
                            <b>{ showing }</b>{"\u{00a0}of\u{00a0}"}
                            <b>{ total_entries }</b>
                        </span>
                        <span class="pf-c-options-menu__toggle-icon">
                            { props.position.toggle_icon(*expanded)}
                        </span>
                    </Button>
                </div>

            if *expanded {
                <ul class="pf-c-options-menu__menu" >
                    { for limit_choices.into_iter().map(|limit|  {
                        let expanded = expanded.clone();
                        let onlimit = onlimit.clone();
                        let onclick = Callback::from(move |_|{
                            onlimit.emit(limit);
                            expanded.set(false);
                        });
                        html!(
                            <li>
                                <Button
                                    class="pf-c-options-menu__menu-item"
                                    {onclick}
                                >
                                    {limit} {" per page"}
                                    if props.selected_choice == limit {
                                        <div class="pf-c-options-menu__menu-item-icon">
                                            { Icon::Check }
                                        </div>
                                    }
                                </Button>
                            </li>
                    )})}
                </ul>
            }
            </div>

            // the navigation buttons

            <nav class="pf-c-pagination__nav" aria-label="Pagination">
                <div class="pf-c-pagination__nav-control pf-m-first">
                    <Button
                        variant={ButtonVariant::Plain}
                        onclick={onnavigation.reform(|_|Navigation::First)}
                        disabled={ props.offset == 0 }
                        aria_label="Go to first page"
                    >
                      { Icon::AngleDoubleLeft }
                    </Button>
                </div>
                <div class="pf-c-pagination__nav-control pf-m-prev">
                    <Button
                        aria_label="Go to previous page"
                        variant={ButtonVariant::Plain}
                        onclick={onnavigation.reform(|_|Navigation::Previous)}
                        disabled={ props.offset == 0 }
                    >
                       { Icon::AngleLeft }
                    </Button>
                </div>
                <div class="pf-c-pagination__nav-page-select">
                    <TextInput
                        r#type="number"
                        {oninput}
                        {onkeydown}
                        state={(*input_state).clone()}
                        value={(*input_text).clone().unwrap_or_default()}
                    />
                if let Some(max_page) = max_page {
                    <span aria-hidden="true">{ "of "} { max_page }</span>
                }
                </div>

                <div class="pf-c-pagination__nav-control pf-m-next">
                    <Button
                        aria_label="Go to next page"
                        variant={ButtonVariant::Plain}
                        onclick={onnavigation.reform(|_|Navigation::Next)}
                        disabled={max_page.map_or_else(|| false, |m| current_page >= m)}
                    >
                        { Icon::AngleRight }
                    </Button>
                </div>
                <div class="pf-c-pagination__nav-control pf-m-last">
                    <Button
                        aria_label="Go to last page"
                        variant={ButtonVariant::Plain}
                        onclick={onnavigation.reform(|_|Navigation::Last)}
                        disabled={is_last_page}
                    >
                        { Icon::AngleDoubleRight }
                    </Button>
                </div>
            </nav>
        </div>
    )
}
