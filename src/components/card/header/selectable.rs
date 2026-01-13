use super::*;

/// Selectable actions for a card. Note the hints about `clickable` and `selectable` in the containing [`Card`].
#[derive(Debug, Clone, PartialEq)]
pub enum CardSelectableActionsVariant {
    /// The entire card is clickable. Performs an action on click.
    /// Clicking the card will highlight the card.
    /// If you only want a single card out of a selection to be highlighted,
    /// then make sure that the `name` field of the [`CardHeaderSelectableActionsObjectBase`]
    /// has the same value in all [`Card`]s between which you want to differentiate.
    /// Requires setting `clickable` to `true` and `selectable` to `false` in the containing [`Card`].
    Click {
        onclick: Option<Callback<MouseEvent>>,
    },
    /// Uses radio selection for selecting the card.
    ///
    /// Requires setting `clickable` to `true` in the containing [`Card`].
    /// To make sure that only a single [`Card`] out of a group can be selected at once,
    /// make sure that the `name` field of the [`CardHeaderSelectableActionsObjectBase`]
    /// has the same value in all [`Card`]s between which you want to differentiate.
    /// If `clickable` is `false` in the containing [`Card`], then clicking anywhere within the card will toggle the state of the radio button.
    /// If `clickable is `true` in the containing` [`Card`], then only clicking the radio button itself will toggle the state of the radio button (to allow having other clickable content within the card such as links).
    SingleSelect { onchange: Option<Callback<()>> },
    /// Checkbox selection for selecting any amount of cards.
    /// If `clickable` is `false` in the containing [`Card`], then clicking anywhere within the card will toggle the state of the checkbox.
    /// If `clickable is `true` in the containing` [`Card`], then only clicking the radio button itself will toggle the state of the checkbox (to allow having other clickable content within the card such as links).
    MultiSelect {
        onchange: Callback<CheckboxState>,
        checked: CheckboxState,
    },
}

/// Interactions with a card through clicking on it.
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CardSelectableActionsObjectProperties {
    /// The actual action.
    pub action: CardSelectableActionsVariant,
    /// Meta information common to any kind of action.
    #[prop_or_default]
    pub base: CardSelectableActionsObjectBase,
}

/// Metadata for a selectable action.
#[derive(Debug, Clone, PartialEq, Properties, Default)]
pub struct CardSelectableActionsObjectBase {
    /// Remove the offset of the position of the actions to the header content.
    /// This looks better if using large card titles or tall header images, for example.
    #[prop_or_default]
    pub has_no_offset: bool,
    /// Additional classes to the selectable actions object.
    #[prop_or_default]
    pub class: Classes,
    /// HTML id
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// The name of the action. Use this field to group action across multiple cards.
    /// This is useful for single selections to describe which cards can be selected from.
    #[prop_or_default]
    pub name: Option<AttrValue>,
}

#[function_component(CardSelectableActionsObject)]
pub fn selectable_actions_object(props: &CardSelectableActionsObjectProperties) -> Html {
    type Variant = CardSelectableActionsVariant;
    match &props.action {
        Variant::SingleSelect { onchange } => html! {
            <SingleSelectActionRadio base={props.base.clone()} onchange={onchange.clone()} />
        },
        Variant::MultiSelect { onchange, checked } => html! {
            <MultiSelectActionCheckbox base={props.base.clone()} onchange={onchange.clone()} checked={*checked} />
        },
        Variant::Click { onclick } => html! {
            <ClickableInput base={props.base.clone()} onclick={onclick.clone()} />
        },
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct SingleSelectActionRadioProperties {
    base: CardSelectableActionsObjectBase,
    onchange: Option<Callback<()>>,
}

struct CommonProps {
    id: Option<String>,
    name: Option<AttrValue>,
    disabled: bool,
    base_class: &'static str,
}

fn get_common_props(base: &CardSelectableActionsObjectBase, context: &CardContext) -> CommonProps {
    CommonProps {
        base_class: "pf-m-standalone",
        id: base.id.as_ref().map(|s| s.to_string()),
        name: base.name.as_ref().cloned(),
        disabled: context.disabled,
    }
}

#[function_component(SingleSelectActionRadio)]
fn single_select_action_radio(props: &SingleSelectActionRadioProperties) -> Html {
    let context: CardContext = use_context().expect("Couldn't find card context");
    let onchange = {
        let onchange = props.onchange.clone();
        Callback::from(move |_| {
            if let Some(f) = onchange.clone() {
                f.emit(())
            }
        })
    };
    let common = get_common_props(&props.base, &context);
    html! {
        <Radio
            class={common.base_class}
            id={common.id}
            name={common.name}
            disabled={common.disabled}
            {onchange}
            force_label=true
        />
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct MultiSelectActionCheckboxProperties {
    base: CardSelectableActionsObjectBase,
    onchange: Callback<CheckboxState>,
    checked: CheckboxState,
}

#[function_component(MultiSelectActionCheckbox)]
fn multi_select_action_checkbox(props: &MultiSelectActionCheckboxProperties) -> Html {
    let context: CardContext = use_context().expect("Couldn't find card context");
    let common = get_common_props(&props.base, &context);
    html! {
        <Checkbox
            class={common.base_class}
            id={common.id}
            name={common.name}
            disabled={common.disabled}
            onchange={props.onchange.clone()}
            checked={props.checked}
            label={html!()}
        />
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct ClickableInputProperties {
    base: CardSelectableActionsObjectBase,
    onclick: Option<Callback<MouseEvent>>,
}

#[function_component(ClickableInput)]
fn clickable_input_action_radio(props: &ClickableInputProperties) -> Html {
    let context: CardContext = use_context().expect("Couldn't find card context");
    if context.selectable {
        log::warn!(
            "Using a click action for an entire tile in a selectable card doesn't work. Set `selectable` to `false` in card `{}`",
            context.card_id
        );
    }
    let common = get_common_props(&props.base, &context);
    html! {
        <Radio
            class={common.base_class}
            id={common.id}
            name={common.name}
            disabled={common.disabled}
            input_class="pf-v5-screen-reader"
            input_onclick={props.onclick.clone()}
            force_label=true
        />
    }
}
