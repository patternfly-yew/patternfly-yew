use crate::{SelectVariant, ValidationContext};
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::rc::Rc;
use std::str::FromStr;
use wasm_bindgen::JsCast;
use web_sys::{HtmlOptionElement, HtmlSelectElement};
use yew::{
    html::ChildrenRenderer,
    prelude::*,
    virtual_dom::{VChild, VComp},
};

/// Properties for [`FormSelect`]
#[derive(Clone, PartialEq, Properties)]
pub struct FormSelectProperties<K: 'static + Clone + PartialEq + Display + FromStr> {
    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or_default]
    pub name: AttrValue,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub required: bool,

    #[prop_or_default]
    pub placeholder: String,

    #[prop_or_default]
    pub onchange: Callback<Option<K>>,

    /// The selected value.
    #[prop_or_default]
    pub value: Option<K>,

    #[prop_or_default]
    pub children: ChildrenRenderer<FormSelectChildVariant<K>>,

    #[prop_or_default]
    pub onvalidate: Callback<ValidationContext<Vec<K>>>,
}

/// A select component in a [`Form`](crate::prelude::Form)
#[function_component(FormSelect)]
pub fn form_select<K>(props: &FormSelectProperties<K>) -> Html
where
    K: 'static + Clone + PartialEq + Display + FromStr,
{
    let class = Classes::from("pf-v5-c-form-control");

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

        use_effect_with_deps(
            move |value| {
                if let Some(ele) = node_ref.cast::<HtmlSelectElement>() {
                    ele.set_value(value);
                }
            },
            props
                .value
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_default(),
        );
    }

    html! (
        <select
            {class}
            {oninput}
            name={&props.name}
            id={&props.id}
            ref={node_ref}
            required={props.required}
            >
            if !props.placeholder.is_empty() {
                <option value="">{ &props.placeholder }</option>
            }
            { for props.children.iter() }
        </select>
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

impl<K> Into<Html> for FormSelectChildVariant<K>
where
    K: 'static + Clone + PartialEq + Display,
{
    fn into(self) -> Html {
        match self.props {
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
