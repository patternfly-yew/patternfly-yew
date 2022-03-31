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

#[derive(Clone, PartialEq, Properties)]
pub struct Props<K: 'static + Clone + PartialEq + Display + Debug + FromStr> {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub placeholder: String,

    #[prop_or_default]
    pub variant: SelectVariant<K>,

    #[prop_or_default]
    pub children: ChildrenRenderer<FormSelectChildVariant<K>>,

    #[prop_or_default]
    pub onvalidate: Callback<ValidationContext<Vec<K>>>,
}

pub struct FormSelect<K>
where
    K: 'static + Clone + PartialEq + Display + Debug + FromStr,
{
    _marker: PhantomData<K>,
    node_ref: NodeRef,
}

#[derive(Clone, Debug)]
pub enum Msg {
    Changed,
}

impl<K> Component for FormSelect<K>
where
    K: 'static + Clone + PartialEq + Display + Debug + FromStr,
{
    type Message = Msg;
    type Properties = Props<K>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            _marker: Default::default(),
            node_ref: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Changed => {
                self.input_changed(ctx);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let classes = Classes::from("pf-c-form-control");

        let multiple = !matches!(ctx.props().variant, SelectVariant::Single(_));

        let oninput = ctx.link().callback(|_| Msg::Changed);

        html! (
            <select
                class={classes}
                multiple={multiple}
                {oninput}
                ref={self.node_ref.clone()}
                >
                { for ctx.props().children.iter() }
            </select>
        )
    }
}

impl<K> FormSelect<K>
where
    K: 'static + Clone + PartialEq + Display + Debug + FromStr,
{
    fn input_changed(&self, ctx: &Context<Self>) {
        if let Some(ele) = self.node_ref.cast::<HtmlSelectElement>() {
            match &ctx.props().variant {
                SelectVariant::Single(callback) => {
                    let value = ele.value();
                    if let Ok(value) = K::from_str(&value) {
                        callback.emit(value);
                    }
                }
                SelectVariant::Checkbox(callback) | SelectVariant::Multiple(callback) => {
                    let opts = ele.selected_options();
                    let mut values = Vec::new();
                    for i in 0..opts.length() {
                        if let Some(opt) = opts.item(i) {
                            if let Some(ele) = opt.dyn_ref::<HtmlOptionElement>() {
                                if let Ok(value) = K::from_str(&ele.value()) {
                                    values.push(value);
                                }
                            }
                        }
                    }
                    callback.emit(values);
                }
            }
        }
    }
}

// child

#[derive(Clone, PartialEq)]
pub enum FormSelectChild<K>
where
    K: 'static + Clone + PartialEq + Display + Debug,
{
    Option(Rc<<FormSelectOption<K> as Component>::Properties>),
    Group(Rc<<FormSelectGroup<K> as Component>::Properties>),
}

impl<K> From<FormSelectOptionProps<K>> for FormSelectChild<K>
where
    K: Clone + PartialEq + Display + Debug,
{
    fn from(props: FormSelectOptionProps<K>) -> Self {
        FormSelectChild::Option(Rc::new(props))
    }
}

impl<K> From<FormSelectGroupProps<K>> for FormSelectChild<K>
where
    K: Clone + PartialEq + Display + Debug,
{
    fn from(props: FormSelectGroupProps<K>) -> Self {
        FormSelectChild::Group(Rc::new(props))
    }
}

// variant

#[derive(PartialEq, Clone)]
pub struct FormSelectChildVariant<K>
where
    K: 'static + Clone + PartialEq + Display + Debug,
{
    props: FormSelectChild<K>,
}

impl<K, CHILD> From<VChild<CHILD>> for FormSelectChildVariant<K>
where
    CHILD: Component,
    CHILD::Properties: Into<FormSelectChild<K>> + Clone,
    K: 'static + Clone + PartialEq + Display + Debug,
{
    fn from(vchild: VChild<CHILD>) -> Self {
        Self {
            props: (*vchild.props).clone().into(),
        }
    }
}

impl<K> Into<Html> for FormSelectChildVariant<K>
where
    K: 'static + Clone + PartialEq + Display + Debug,
{
    fn into(self) -> Html {
        match self.props {
            FormSelectChild::Option(props) => {
                VComp::new::<FormSelectOption<K>>(props, NodeRef::default(), None).into()
            }
            FormSelectChild::Group(props) => {
                VComp::new::<FormSelectGroup<K>>(props, NodeRef::default(), None).into()
            }
        }
    }
}

// Item

#[derive(Clone, PartialEq, Properties)]
pub struct FormSelectOptionProps<K>
where
    K: Clone + PartialEq + Display + Debug,
{
    pub value: K,

    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub description: Option<String>,

    #[prop_or_default]
    pub selected: bool,

    #[prop_or_default]
    pub onclick: Option<Callback<K>>,

    #[prop_or_default]
    pub(crate) want_clicked: Callback<K>,
}

pub struct FormSelectOption<K>
where
    K: 'static + Clone + PartialEq + Display + Debug,
{
    _marker: PhantomData<K>,
}

impl<K> Component for FormSelectOption<K>
where
    K: 'static + Clone + PartialEq + Display + Debug,
{
    type Message = ();
    type Properties = FormSelectOptionProps<K>;

    fn create(_: &Context<Self>) -> Self {
        Self {
            _marker: Default::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! (
            <option
                id={ctx.props().id.clone()}
                selected={ctx.props().selected}
                value={ctx.props().value.to_string()}
            >
                { if let Some(description) = &ctx.props().description {
                    html!{ &description }
                } else {
                    html!{ &ctx.props().value }
                }}
            </option>
        )
    }
}

// Group

#[derive(Clone, PartialEq, Properties)]
pub struct FormSelectGroupProps<K>
where
    K: 'static + Clone + PartialEq + Display + Debug,
{
    pub label: String,
    #[prop_or_default]
    pub children: ChildrenRenderer<FormSelectChildVariant<K>>,
}

#[derive(Clone)]
pub struct FormSelectGroup<K>
where
    K: 'static + Clone + PartialEq + Display + Debug,
{
    _marker: PhantomData<K>,
}

impl<K> Component for FormSelectGroup<K>
where
    K: Clone + PartialEq + Display + Debug,
{
    type Message = ();
    type Properties = FormSelectGroupProps<K>;

    fn create(_: &Context<Self>) -> Self {
        Self {
            _marker: Default::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <optgroup label={ctx.props().label.clone()}>
                    { for ctx.props().children.iter() }
                </optgroup>
            </>
        }
    }
}
