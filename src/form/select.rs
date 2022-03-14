use crate::{Divider, SelectVariant};
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::rc::Rc;
use yew::{
    html::ChildrenRenderer,
    prelude::*,
    virtual_dom::{VChild, VComp},
};

#[derive(Clone, PartialEq, Properties)]
pub struct Props<K: 'static + Clone + PartialEq + Display + Debug> {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub multiple: bool,

    #[prop_or_default]
    pub placeholder: String,

    #[prop_or_default]
    pub variant: SelectVariant<K>,

    #[prop_or_default]
    pub children: ChildrenRenderer<FormSelectChildVariant<K>>,
}

pub struct FormSelect<K>
where
    K: 'static + Clone + PartialEq + Display + Debug,
{
    selection: Vec<K>,
}

#[derive(Clone, Debug)]
pub enum Msg<K> {
    Clicked(K),
}

impl<K> Component for FormSelect<K>
where
    K: 'static + Clone + PartialEq + Display + Debug,
{
    type Message = Msg<K>;
    type Properties = Props<K>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            selection: Vec::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked(k) => self.clicked(ctx, k),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let classes = Classes::from("pf-c-form-control");

        html! (
            <select class={classes}>
                { for ctx.props().children.iter().map(|mut c|{
                    c.set_need_clicked(ctx.link().callback(|k|Msg::Clicked(k)));
                    c.set_variant(ctx.props().variant.clone());
                    c
                }) }
            </select>
        )
    }
}

impl<K> FormSelect<K>
where
    K: 'static + Clone + PartialEq + Display + Debug,
{
    fn clicked(&mut self, ctx: &Context<Self>, key: K) {
        log::info!("Clicked: {}", key);
        match &ctx.props().variant {
            SelectVariant::Single(on) => {
                self.selection = vec![key.clone()];
                on.emit(key);
            }
            SelectVariant::Multiple(on) | SelectVariant::Checkbox(on) => {
                match self.selection.iter().position(|x| *x == key) {
                    Some(idx) => {
                        // remove
                        self.selection.remove(idx);
                    }
                    None => {
                        // add
                        self.selection.push(key);
                    }
                }

                on.emit(self.selection.clone());
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
    Divider(Rc<<Divider as Component>::Properties>),
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

impl<K> From<()> for FormSelectChild<K>
where
    K: Clone + PartialEq + Display + Debug,
{
    fn from(_: ()) -> Self {
        FormSelectChild::Divider(Rc::new(()))
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

impl<K> FormSelectChildVariant<K>
where
    K: Clone + PartialEq + Display + Debug,
{
    fn set_need_clicked(&mut self, callback: Callback<K>) {
        match self.props {
            FormSelectChild::Option(ref mut props) => {
                let props = Rc::make_mut(props);
                props.want_clicked = callback;
            }
            FormSelectChild::Group(ref mut props) => {
                let props = Rc::make_mut(props);
                props.want_clicked = callback;
            }
            _ => {}
        }
    }

    fn set_variant(&mut self, variant: SelectVariant<K>) {
        match self.props {
            FormSelectChild::Option(ref mut props) => {
                let props = Rc::make_mut(props);
                props.variant = variant;
            }
            FormSelectChild::Group(ref mut props) => {
                let props = Rc::make_mut(props);
                props.variant = variant;
            }
            _ => {}
        }
    }
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
            FormSelectChild::Divider(props) => {
                VComp::new::<Divider>(props, NodeRef::default(), None).into()
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

    #[prop_or_default]
    pub(crate) variant: SelectVariant<K>,
}

#[derive(Clone, Copy, Debug)]
pub enum FormSelectOptionMsg {
    Clicked,
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
    type Message = FormSelectOptionMsg;
    type Properties = FormSelectOptionProps<K>;

    fn create(_: &Context<Self>) -> Self {
        Self {
            _marker: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Clicked => {
                log::info!("Clicked on: {:?}", ctx.props().value);
                if let Some(onclick) = &ctx.props().onclick {
                    // if we have a click handler, we don't send the default handling
                    onclick.emit(ctx.props().value.clone());
                } else {
                    // default is to report clicked, if we have a key
                    ctx.props().want_clicked.emit(ctx.props().value.clone());
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-c-select__menu-item");

        if ctx.props().selected {
            classes.push("pf-m-selected");
        }

        if ctx.props().description.is_some() {
            classes.push("pf-m-description");
        }

        return html! {
            <option
                id={ctx.props().id.clone()}
                class={classes}
                selected={ctx.props().selected}
                value={ctx.props().value.to_string()}
                onclick={ctx.link().callback(|_|FormSelectOptionMsg::Clicked)}
            >
                { if let Some(description) = &ctx.props().description {
                    html!{ &description }
                } else {
                    html!{ &ctx.props().value }
                }}
            </option>
        };
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
    #[prop_or_default]
    pub(crate) want_clicked: Callback<K>,
    #[prop_or_default]
    pub(crate) variant: SelectVariant<K>,
}

#[derive(Clone)]
pub struct FormSelectGroup<K>
where
    K: 'static + Clone + PartialEq + Display + Debug,
{
    _marker: PhantomData<K>,
}

#[derive(Clone, Debug)]
pub enum FormSelectGroupMsg<K> {
    Clicked(K),
}

impl<K> Component for FormSelectGroup<K>
where
    K: Clone + PartialEq + Display + Debug,
{
    type Message = FormSelectGroupMsg<K>;
    type Properties = FormSelectGroupProps<K>;

    fn create(_: &Context<Self>) -> Self {
        Self {
            _marker: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Clicked(k) => ctx.props().want_clicked.emit(k),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <optgroup label={ctx.props().label.clone()}>
                    { for ctx.props().children.iter().map(|mut c|{
                        c.set_need_clicked(ctx.link().callback(|k|Self::Message::Clicked(k)));
                        c.set_variant(ctx.props().variant.clone());
                        c
                    })}
                </optgroup>
            </>
        }
    }
}
