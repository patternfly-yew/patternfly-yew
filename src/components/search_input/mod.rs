use std::ops::Deref;

use crate::components::badge::Badge;
use crate::components::button::*;
use crate::components::input_group::*;
use crate::components::text_input_group::*;
use crate::icon::Icon;
use crate::utils::HtmlElementSupport;
use yew::html::IntoPropValue;
use yew::prelude::*;
use yew_hooks::use_event_with_window;

/// The number of search results returned. Either a total number of results,
/// or an index of the current result in relation to total results.
#[derive(Debug, Clone, PartialEq)]
pub enum ResultsCount {
    Absolute(usize),
    /// For an index out of total such as "1/5"
    Fraction(usize, usize),
}

impl IntoPropValue<Html> for ResultsCount {
    fn into_prop_value(self) -> Html {
        match self {
            Self::Absolute(i) => html!(i),
            Self::Fraction(i, j) => html!(format!("{i}/{j}")),
        }
    }
}

pub enum OnSearchEvent {
    Mouse(MouseEvent),
    Keyboard(KeyboardEvent),
}

impl Deref for OnSearchEvent {
    type Target = Event;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Mouse(e) => e.deref(),
            Self::Keyboard(e) => e.deref(),
        }
    }
}

impl From<MouseEvent> for OnSearchEvent {
    fn from(value: MouseEvent) -> Self {
        Self::Mouse(value)
    }
}

impl From<KeyboardEvent> for OnSearchEvent {
    fn from(value: KeyboardEvent) -> Self {
        Self::Keyboard(value)
    }
}

/// The main search input component.
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct SearchInputProperties {
    /// Id of the outermost element
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// An accessible label for the search input.
    #[prop_or_default]
    pub aria_label: AttrValue,
    /// Additional classes added to the search input.
    #[prop_or_default]
    pub class: Classes,
    /// Object that makes the search input expandable/collapsable.
    #[prop_or_default]
    pub expandable: Option<SearchInputExpandableProperties>,
    /// A suggestion for autocompleting
    #[prop_or_default]
    pub hint: Option<AttrValue>,
    /// A reference object to attach to the input box.
    #[prop_or_default]
    pub inner_ref: Option<NodeRef>,
    /// Flag indicating if searchinput is disabled.
    #[prop_or_default]
    pub disabled: bool,
    /// Placeholder text of the search input.
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    /// Label for the button which resets the advanced search form and clears the search input.
    #[prop_or(AttrValue::from("Reset"))]
    pub reset_button_label: AttrValue,
    /// Label for the button which calls the onSearch event handler.
    #[prop_or(AttrValue::from("Search"))]
    pub submit_search_button_label: AttrValue,
    /// Flag to indicate utilities should be displayed.
    /// By default, utilities will only be displayed when the search input has a value.
    #[prop_or_default]
    pub utilities_displayed: bool,
    /// Value of the search input.
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub autofocus: bool,

    // Navigable results
    /// The number of search results returned. View `[ResultsCount]`.
    #[prop_or_default]
    pub results_count: Option<ResultsCount>,
    /// Accessible label for the button to navigate to previous result.
    #[prop_or(AttrValue::from("Previous"))]
    pub previous_navigation_button_aria_label: AttrValue,
    /// Flag indicating if the previous navigation button is disabled.
    #[prop_or_default]
    pub previous_navigation_button_disabled: bool,
    /// Accessible label for the button to navigate to next result.
    #[prop_or(AttrValue::from("Next"))]
    pub next_navigation_button_aria_label: AttrValue,
    /// Flag indicating if the next navigation button is disabled.
    #[prop_or_default]
    pub next_navigation_button_disabled: bool,

    // Callbacks
    /// A callback for when the input value changes.
    #[prop_or_default]
    pub onchange: Option<Callback<String>>,
    /// A callback for when the user clicks the clear button.
    #[prop_or_default]
    pub onclear: Option<Callback<MouseEvent>>,
    /// A callback for when the user clicks to navigate to next result.
    #[prop_or_default]
    pub onnextclick: Option<Callback<MouseEvent>>,
    /// A callback for when the user clicks to navigate to previous result.
    #[prop_or_default]
    pub onpreviousclick: Option<Callback<MouseEvent>>,
    /// A callback for when the search button is clicked.
    #[prop_or_default]
    pub onsearch: Option<Callback<(OnSearchEvent, String)>>,
}

/// Properties for creating an expandable search input. These properties should be passed into
/// the search input component's expandableInput property.
///
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct SearchInputExpandableProperties {
    /// Flag to indicate if the search input is expanded.
    #[prop_or_default]
    pub expanded: bool,
    /// Callback function to toggle the expandable search input.
    #[prop_or_default]
    pub ontoggleexpand: Callback<(MouseEvent, bool)>,
    /// An accessible label for the expandable search input toggle.
    #[prop_or_default]
    pub toggle_aria_label: AttrValue,
}

#[function_component(SearchInput)]
pub fn search_input(props: &SearchInputProperties) -> Html {
    let search_value = use_state(|| props.value.clone());
    use_effect_with(
        (props.value.clone(), search_value.clone()),
        move |(prop_val, search_value)| search_value.set(prop_val.clone()),
    );
    let focus_after_expand_change = use_state(|| false);
    let is_search_menu_open = use_state(|| false);
    let node_ref = use_node_ref();
    let input_ref = props.inner_ref.clone().unwrap_or(node_ref);
    let expandable_toggle_ref = use_node_ref();

    use_effect_with(
        (
            focus_after_expand_change.clone(),
            props.expandable.clone(),
            input_ref.clone(),
            expandable_toggle_ref.clone(),
        ),
        |(focus, expandable, input_ref, toggle_ref)| {
            if !**focus {
                return;
            }
            if expandable.as_ref().is_some_and(|e| e.expanded) {
                input_ref.focus();
            } else {
                toggle_ref.focus();
            }
        },
    );

    let ontoggle = use_callback(is_search_menu_open.clone(), |_, is_search_menu_open| {
        is_search_menu_open.set(!**is_search_menu_open);
    });
    let expand_toggle = if let Some(expandable) = &props.expandable {
        let onclick = {
            let value = search_value.clone();
            let ontoggleexpand = expandable.ontoggleexpand.clone();
            let focus_after_expand_change = focus_after_expand_change.clone();
            let expanded = expandable.expanded;
            Callback::from(move |e| {
                value.set(String::new());
                ontoggleexpand.emit((e, expanded));
                focus_after_expand_change.set(true);
            })
        };
        html! {
            <Button
                variant={ButtonVariant::Plain}
                aria_label={expandable.toggle_aria_label.clone()}
                aria_expanded={expandable.expanded.to_string()}
                icon={if expandable.expanded { Icon::Times} else { Icon::Search }}
                {onclick}
            />
        }
    } else {
        html! {}
    };

    if let Some(SearchInputExpandableProperties {
        expanded: false, ..
    }) = props.expandable
    {
        html! {
            <InputGroup class={props.class.clone()}>
                <InputGroupItem>{expand_toggle}</InputGroupItem>
            </InputGroup>
        }
    } else if props.onsearch.is_some() {
        html! {
            <TextInputGroupWithExtraButtons
                search_value={search_value.clone()}
                focus_after_expand_change={focus_after_expand_change.clone()}
                is_search_menu_open={is_search_menu_open.clone()}
                ontoggle={ontoggle.clone()}
                expand_toggle={expand_toggle.clone()}
                {input_ref}
                props={props.clone()}
            />
        }
    } else if props.expandable.is_some() {
        html! {
            <ExpandableInputGroup
                search_value={search_value.clone()}
                expand_toggle={expand_toggle.clone()}
                {input_ref}
                props={props.clone()}
            />
        }
    } else {
        html! {
            <InnerTextInputGroup
                search_value={search_value.clone()}
                {input_ref}
                props={props.clone()}
            />
        }
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct ExpandableInputGroupProps {
    search_value: UseStateHandle<String>,
    expand_toggle: Html,
    input_ref: NodeRef,
    props: SearchInputProperties,
}

#[function_component(ExpandableInputGroup)]
fn expandable_input_group(props: &ExpandableInputGroupProps) -> Html {
    html! {
        <InputGroup
            id={&props.props.id}
            class={props.props.class.clone()}
        >
            <InputGroupItem fill=true>
                <InnerTextInputGroup
                    props={props.props.clone()}
                    search_value={props.search_value.clone()}
                    input_ref={props.input_ref.clone()}
                />
            </InputGroupItem>
            <InputGroupItem plain=true>{props.expand_toggle.clone()}</InputGroupItem>
        </InputGroup>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct InnerTextInputGroupProps {
    search_value: UseStateHandle<String>,
    input_ref: NodeRef,
    props: SearchInputProperties,
}

#[function_component(InnerTextInputGroup)]
fn inner_text_input_group(props: &InnerTextInputGroupProps) -> Html {
    let onchange = use_callback(
        (props.search_value.clone(), props.props.onchange.clone()),
        |value: String, (search_value, onchange)| {
            if let Some(f) = onchange.as_ref() {
                f.emit(value.clone())
            }
            search_value.set(value)
        },
    );

    let render_utilities = !props.props.value.is_empty()
        && (props.props.results_count.is_some()
            || (props.props.onnextclick.is_some() && props.props.onpreviousclick.is_some())
            || (props.props.onclear.is_some() && props.props.expandable.is_none()));
    let badge = if let Some(results_count) = &props.props.results_count {
        html! { <Badge read=true>{results_count.clone()}</Badge> }
    } else {
        html! {}
    };

    let mut clicknav = html! {};
    if let Some(onnextclick) = &props.props.onnextclick {
        if let Some(onprevclick) = &props.props.onpreviousclick {
            clicknav = html! {
                <div class={classes!["pf-v5-c-text-input-group__group"]}>
                    <Button
                        variant={ButtonVariant::Plain}
                        aria_label={props.props.previous_navigation_button_aria_label.clone()}
                        disabled={props.props.disabled || props.props.previous_navigation_button_disabled}
                        onclick={onprevclick}
                    >
                        {Icon::AngleUp}
                    </Button>
                    <Button
                        variant={ButtonVariant::Plain}
                        aria_label={props.props.next_navigation_button_aria_label.clone()}
                        disabled={props.props.disabled || props.props.next_navigation_button_disabled}
                        onclick={onnextclick.clone()}
                    >
                        {Icon::AngleDown}
                    </Button>
                </div>
            };
        }
    }
    let onclearinput = use_callback(
        (props.props.onclear.clone(), props.input_ref.clone()),
        |e, (onclear, input_ref)| {
            if let Some(f) = onclear.as_ref() {
                f.emit(e)
            }
            input_ref.focus();
        },
    );
    let mut clearnav = html! {};
    if props.props.onclear.is_some() && props.props.expandable.is_none() {
        clearnav = html! {
            <Button
                variant={ButtonVariant::Plain}
                disabled={props.props.disabled}
                aria_label={props.props.reset_button_label.clone()}
                onclick={onclearinput}
            >
                {Icon::Times}
            </Button>
        };
    };
    html! {
        <TextInputGroup
            id={&props.props.id}
            class={props.props.class.clone()}
            disabled={props.props.disabled}
        >
            <TextInputGroupMain
                hint={props.props.hint.clone()}
                icon={Icon::Search}
                value={(*props.search_value).clone()}
                placeholder={props.props.placeholder.clone()}
                aria_label={props.props.aria_label.clone()}
                {onchange}
                inner_ref={props.input_ref.clone()}
                autofocus={props.props.autofocus}
            />
            if render_utilities || props.props.utilities_displayed {
                <TextInputGroupUtilities>
                    {badge}
                    {clicknav}
                    {clearnav}
                </TextInputGroupUtilities>
            }
        </TextInputGroup>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct TextInputGroupWithExtraButtonsProps {
    search_value: UseStateHandle<String>,
    focus_after_expand_change: UseStateHandle<bool>,
    is_search_menu_open: UseStateHandle<bool>,
    input_ref: NodeRef,
    ontoggle: Callback<MouseEvent>,
    expand_toggle: Html,
    props: SearchInputProperties,
}

#[function_component(TextInputGroupWithExtraButtons)]
fn text_input_group_with_extra_buttons(props: &TextInputGroupWithExtraButtonsProps) -> Html {
    let onsearchhandler = use_callback(
        (
            props.props.onsearch.clone(),
            props.props.value.clone(),
            props.is_search_menu_open.clone(),
        ),
        |e: OnSearchEvent, (onsearch, value, is_search_menu_open)| {
            e.prevent_default();
            if let Some(f) = onsearch.as_ref() {
                f.emit((e, value.clone()))
            }
            is_search_menu_open.set(false);
        },
    );
    {
        let onsearchhandler = onsearchhandler.clone();
        use_event_with_window("keydown", move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                onsearchhandler.emit(e.into());
            }
        });
    }

    let submit_button = if props.props.onsearch.is_some() {
        let onsearchhandler = onsearchhandler.clone();
        let onclick = Callback::from(move |e: MouseEvent| onsearchhandler.emit(e.into()));
        html! {
            <InputGroupItem>
                <Button
                    r#type={ButtonType::Submit}
                    variant={ButtonVariant::Control}
                    aria_label={props.props.submit_search_button_label.clone()}
                    {onclick}
                    disabled={props.props.disabled}
                >
                    {Icon::ArrowRight}
                </Button>
            </InputGroupItem>
        }
    } else {
        html! {}
    };

    html! (
        <InputGroup
            id={&props.props.id}
            class={props.props.class.clone()}
        >
            <InputGroupItem fill=true>
                <InnerTextInputGroup
                    props={props.props.clone()}
                    search_value={props.search_value.clone()}
                    input_ref={props.input_ref.clone()}
                />
                {submit_button}
            </InputGroupItem>
            if props.props.expandable.is_some() {
                {props.expand_toggle.clone()}
            }
        </InputGroup>
    )
}
