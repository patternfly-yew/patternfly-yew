use crate::ouia;
use crate::prelude::{Icon, OuiaComponentType, ValidationContext};
use crate::utils::{Ouia, OuiaSafe};
use std::fmt::Display;
use std::rc::Rc;
use std::str::FromStr;
use web_sys::HtmlSelectElement;
use yew::{
    html::ChildrenRenderer,
    prelude::*,
    virtual_dom::{VChild, VComp},
};

const OUIA: Ouia = ouia!("FormSelect");

/// Properties for [`FormSelect`]
#[derive(Clone, PartialEq, Properties)]
pub struct FormSelectProperties<K: 'static + Clone + PartialEq + Display + FromStr> {
    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub name: Option<AttrValue>,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub required: bool,

    #[prop_or_default]
    pub placeholder: Option<AttrValue>,

    #[prop_or_default]
    pub onchange: Callback<Option<K>>,

    /// The selected value.
    #[prop_or_default]
    pub value: Option<K>,

    #[prop_or_default]
    pub children: ChildrenRenderer<FormSelectChildVariant<K>>,

    #[prop_or_default]
    pub onvalidate: Callback<ValidationContext<Vec<K>>>,

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

/// A select component in a [`Form`](crate::prelude::Form)
#[function_component(FormSelect)]
pub fn form_select<K>(props: &FormSelectProperties<K>) -> Html
where
    K: 'static + Clone + PartialEq + Display + FromStr,
{
    let ouia_id = use_memo(props.ouia_id.clone(), |id| {
        id.clone().unwrap_or(OUIA.generated_id())
    });
    let node_ref = use_node_ref();

    let oninput = {
        let node_ref = node_ref.clone();
        let onchange = props.onchange.clone();
        Callback::from(move |_evt: InputEvent| {
            if let Some(ele) = node_ref.cast::<HtmlSelectElement>() {
                let value = ele.value();
                if value.is_empty() {
                    onchange.emit(None);
                } else {
                    onchange.emit(K::from_str(&value).ok());
                }
            }
        })
    };

    {
        let node_ref = node_ref.clone();

        use_effect_with(
            props
                .value
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_default(),
            move |value| {
                if let Some(ele) = node_ref.cast::<HtmlSelectElement>() {
                    ele.set_value(value);
                }
            },
        );
    }

    html! (
        <div class="pf-v6-c-form-control">
            <select
                {oninput}
                name={&props.name}
                id={&props.id}
                ref={node_ref}
                required={props.required}
                data-ouia-component-id={(*ouia_id).clone()}
                data-ouia-component-type={props.ouia_type}
                data-ouia-safe={props.ouia_safe}
            >
                if let Some(placeholder) = &props.placeholder {
                    <option value="">{ placeholder }</option>
                }
                { for props.children.iter() }
            </select>
            <div class="pf-v6-c-form-control__utilities">
                <div class="pf-v6-c-form-control__toggle-icon">
                  {Icon::CaretDown}
                </div>
            </div>
        </div>
    )
}

#[derive(Clone, PartialEq)]
pub enum FormSelectChild<K>
where
    K: 'static + Clone + PartialEq + Display,
{
    Option(Rc<<FormSelectOption<K> as BaseComponent>::Properties>),
    Group(Rc<<FormSelectGroup<K> as BaseComponent>::Properties>),
}

impl<K> From<FormSelectOptionProperties<K>> for FormSelectChild<K>
where
    K: Clone + PartialEq + Display,
{
    fn from(props: FormSelectOptionProperties<K>) -> Self {
        FormSelectChild::Option(Rc::new(props))
    }
}

impl<K> From<FormSelectGroupProperties<K>> for FormSelectChild<K>
where
    K: Clone + PartialEq + Display,
{
    fn from(props: FormSelectGroupProperties<K>) -> Self {
        FormSelectChild::Group(Rc::new(props))
    }
}

// variant

#[derive(PartialEq, Clone)]
pub struct FormSelectChildVariant<K>
where
    K: 'static + Clone + PartialEq + Display,
{
    props: FormSelectChild<K>,
}

impl<K, CHILD> From<VChild<CHILD>> for FormSelectChildVariant<K>
where
    CHILD: BaseComponent,
    CHILD::Properties: Into<FormSelectChild<K>> + Clone,
    K: 'static + Clone + PartialEq + Display,
{
    fn from(vchild: VChild<CHILD>) -> Self {
        Self {
            props: (*vchild.props).clone().into(),
        }
    }
}

impl<K> From<FormSelectChildVariant<K>> for Html
where
    K: 'static + Clone + PartialEq + Display,
{
    fn from(value: FormSelectChildVariant<K>) -> Self {
        match value.props {
            FormSelectChild::Option(props) => VComp::new::<FormSelectOption<K>>(props, None).into(),
            FormSelectChild::Group(props) => VComp::new::<FormSelectGroup<K>>(props, None).into(),
        }
    }
}

// Item

#[derive(Clone, PartialEq, Properties)]
pub struct FormSelectOptionProperties<K>
where
    K: Clone + PartialEq + Display,
{
    pub value: K,

    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub description: Option<String>,

    #[prop_or_default]
    pub selected: bool,
}

#[function_component(FormSelectOption)]
pub fn form_select_option<K>(props: &FormSelectOptionProperties<K>) -> Html
where
    K: 'static + Clone + PartialEq + Display,
{
    html! (
        <option
            id={props.id.clone()}
            selected={props.selected}
            value={props.value.to_string()}
        >
            if let Some(description) = &props.description {
                { &description }
            } else {
                { &props.value }
            }
        </option>
    )
}

#[derive(Clone, PartialEq, Properties)]
pub struct FormSelectGroupProperties<K>
where
    K: 'static + Clone + PartialEq + Display,
{
    pub label: String,
    #[prop_or_default]
    pub children: ChildrenRenderer<FormSelectChildVariant<K>>,
}

#[function_component(FormSelectGroup)]
pub fn form_select_group<K>(props: &FormSelectGroupProperties<K>) -> Html
where
    K: 'static + Clone + PartialEq + Display,
{
    html! (
        <optgroup label={props.label.clone()}>
            { for props.children.iter() }
        </optgroup>
    )
}
