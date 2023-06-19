//! Select control
use crate::prelude::{Chip, GlobalClose, Icon, ListDivider};
use std::{
    cell::Cell,
    fmt::{Debug, Display},
    marker::PhantomData,
    rc::Rc,
};
use uuid::Uuid;
use yew::{
    html::ChildrenRenderer,
    prelude::*,
    virtual_dom::{VChild, VComp},
    BaseComponent,
};

#[derive(Clone, Debug, PartialEq)]
pub enum SelectVariant<K> {
    Single(Callback<K>),
    Multiple(Callback<Vec<K>>),
    Checkbox(Callback<Vec<K>>),
}

impl<K> Default for SelectVariant<K> {
    fn default() -> Self {
        Self::Single(Default::default())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ChipVariant {
    #[default]
    None,
    Count,
    Values,
}

/// Properties for [`Select`]
#[derive(Clone, PartialEq, Properties)]
pub struct SelectProperties<K>
where
    K: Clone + PartialEq + Display + 'static,
{
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub plain: bool,

    #[prop_or_default]
    pub top: bool,

    #[prop_or_default]
    pub multiple: bool,

    #[prop_or_default]
    pub icon: Option<Html>,

    #[prop_or_default]
    pub placeholder: String,

    #[prop_or_default]
    pub variant: SelectVariant<K>,

    #[prop_or_default]
    pub chip: ChipVariant,

    #[prop_or_default]
    pub children: ChildrenRenderer<SelectChildVariant<K>>,

    #[prop_or_default]
    pub initial_selection: Vec<K>,
}

/// Select component
///
/// > A **select** list enables users to select one or more items from a list. Use a select list when options are dynamic or variable.
///
/// See: <https://www.patternfly.org/v4/components/select>
///
/// ## Properties
///
/// Defined by [`SelectProperties`].
pub struct Select<K>
where
    K: 'static + Clone + PartialEq + Display + Debug,
{
    selection: Vec<K>,

    expanded: bool,
    global_close: GlobalClose,
}

#[derive(Clone, Debug)]
pub enum Msg<K> {
    Toggle,
    Close,
    Clicked(K),
}

impl<K> Component for Select<K>
where
    K: 'static + Clone + PartialEq + Display + Debug,
{
    type Message = Msg<K>;
    type Properties = SelectProperties<K>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            expanded: false,
            global_close: GlobalClose::new(NodeRef::default(), ctx.link().callback(|_| Msg::Close)),
            selection: ctx.props().initial_selection.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Toggle => {
                self.expanded = !self.expanded;
            }
            Msg::Close => self.expanded = false,
            Msg::Clicked(k) => self.clicked(ctx, k),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-v5-c-select");
        if self.expanded {
            classes.push("pf-m-expanded");
        }
        if ctx.props().top {
            classes.push("pf-m-top");
        }

        let menu_classes = Classes::from("pf-v5-c-select__menu");

        let onclick = ctx.link().callback(|_| Msg::Toggle);

        html! (
            <div class={classes}
                ref={self.global_close.clone()}
            >
                <button
                    class="pf-v5-c-select__toggle"
                    aria-haspopup="true"
                    aria-expanded="false"
                    disabled={ctx.props().disabled}
                    onclick={onclick}
                    id={ctx.props().id.clone()}
                >
                    <div class="pf-v5-c-select__toggle-wrapper">
                        if let Some(icon) = &ctx.props().icon {
                            <span class="pf-v5-c-select__toggle-icon">
                                { icon.clone() }
                            </span>
                        }
                        { self.render_selection(ctx) }
                    </div>
                    <div class="pf-v5-c-select__toggle-arrow">
                        { Icon::CaretDown }
                    </div>
                </button>
                <div
                    class={menu_classes}
                    hidden={!self.expanded}
                >
                    {
                        match ctx.props().variant {
                            SelectVariant::Single(_) => self.render_button(ctx),
                            SelectVariant::Multiple(_) => self.render_button(ctx),
                            SelectVariant::Checkbox(_) => self.render_checkbox(ctx),
                        }
                    }
                </div>
            </div>
        )
    }
}

impl<K> Select<K>
where
    K: 'static + Clone + PartialEq + Display + Debug,
{
    fn render_selection(&self, ctx: &Context<Self>) -> Html {
        let selection = self.selection.as_slice();
        if selection.is_empty() {
            return html! {<span class="pf-v5-c-select__toggle-text">{&ctx.props().placeholder}</span>};
        }

        match &ctx.props().variant {
            SelectVariant::Single(_) => {
                html! (<span class="pf-v5-c-select__toggle-text">{ &selection[0] }</span>)
            }
            SelectVariant::Checkbox(_) | SelectVariant::Multiple(_) => {
                html! (
                    <>
                        <span class="pf-v5-c-select__toggle-text">{&ctx.props().placeholder}</span>
                        { match &ctx.props().chip {
                            ChipVariant::None => html!(),
                            ChipVariant::Count => {
                                html! (
                                    <div class="pf-v5-c-select__toggle_badge">
                                        <Chip text={selection.len().to_string()} />
                                    </div>
                                )
                            },
                            ChipVariant::Values => {
                                selection.iter().map(|b| {
                                    let link = {
                                        let b = b.clone();
                                        ctx.link().callback(move |()|Msg::Clicked(b.clone()))
                                    };
                                    html!(<Chip text={b.to_string()} onclose={link} />)
                                })
                                .collect()
                           }
                        }}
                    </>
                )
            }
        }
    }

    fn render_button(&self, ctx: &Context<Self>) -> Html {
        html! (
            <ul>
                { for ctx.props().children.iter().map(|mut c|{
                    // request a close callback from the item
                    c.set_need_close(ctx.link().callback(|_|Msg::Close));
                    c.set_need_clicked(ctx.link().callback(|k|Msg::Clicked(k)));
                    c.set_variant(ctx.props().variant.clone());
                    c.set_selection(&self.selection);
                    c
                }) }
            </ul>
        )
    }

    fn render_checkbox(&self, ctx: &Context<Self>) -> Html {
        html! (
            <fieldset class="pf-v5-c-select__menu-fieldset" aria-label="Select input">
                { for ctx.props().children.iter().map(|mut c|{
                    // request a close callback from the item
                    c.set_need_close(ctx.link().callback(|_|Msg::Close));
                    c.set_need_clicked(ctx.link().callback(|k|Msg::Clicked(k)));
                    c.set_variant(ctx.props().variant.clone());
                    c.set_selection(&self.selection);
                    c
                }) }
            </fieldset>
        )
    }

    fn clicked(&mut self, ctx: &Context<Self>, key: K) {
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
pub enum SelectChild<K>
where
    K: 'static + Clone + PartialEq + Display,
{
    Option(Rc<<SelectOption<K> as Component>::Properties>),
    Divider(Rc<<ListDivider as BaseComponent>::Properties>),
    Group(Rc<<SelectGroup<K> as Component>::Properties>),
}

impl<K> From<SelectOptionProperties<K>> for SelectChild<K>
where
    K: Clone + PartialEq + Display,
{
    fn from(props: SelectOptionProperties<K>) -> Self {
        SelectChild::Option(Rc::new(props))
    }
}

impl<K> From<()> for SelectChild<K>
where
    K: Clone + PartialEq + Display,
{
    fn from(_: ()) -> Self {
        SelectChild::Divider(Rc::new(()))
    }
}

impl<K> From<SelectGroupProperties<K>> for SelectChild<K>
where
    K: Clone + PartialEq + Display,
{
    fn from(props: SelectGroupProperties<K>) -> Self {
        SelectChild::Group(Rc::new(props))
    }
}

// variant

#[derive(PartialEq, Clone)]
pub struct SelectChildVariant<K>
where
    K: 'static + Clone + PartialEq + Display,
{
    props: SelectChild<K>,
}

impl<K> SelectChildVariant<K>
where
    K: Clone + PartialEq + Display,
{
    /// Forward the need to get a close callback to the actual item
    fn set_need_close(&mut self, callback: Callback<()>) {
        match self.props {
            SelectChild::Option(ref mut props) => {
                let props = Rc::make_mut(props);
                props.want_close = callback;
            }
            SelectChild::Group(ref mut props) => {
                let props = Rc::make_mut(props);
                props.want_close = callback;
            }
            SelectChild::Divider(_) => {}
        }
    }

    fn set_need_clicked(&mut self, callback: Callback<K>) {
        match self.props {
            SelectChild::Option(ref mut props) => {
                let props = Rc::make_mut(props);
                props.want_clicked = callback;
            }
            SelectChild::Group(ref mut props) => {
                let props = Rc::make_mut(props);
                props.want_clicked = callback;
            }
            SelectChild::Divider(_) => {}
        }
    }

    fn set_variant(&mut self, variant: SelectVariant<K>) {
        match self.props {
            SelectChild::Option(ref mut props) => {
                let props = Rc::make_mut(props);
                props.variant = variant;
            }
            SelectChild::Group(ref mut props) => {
                let props = Rc::make_mut(props);
                props.variant = variant;
            }
            SelectChild::Divider(_) => {}
        }
    }

    fn set_selection(&mut self, selection: &[K]) {
        match &mut self.props {
            SelectChild::Option(props) => {
                let props = Rc::make_mut(props);
                props.selected = selection.contains(&props.value);
            }
            SelectChild::Group(props) => {
                let props = Rc::make_mut(props);
                props.selection = selection.to_vec();
            }
            SelectChild::Divider(_) => {}
        }
    }
}

impl<K, CHILD> From<VChild<CHILD>> for SelectChildVariant<K>
where
    CHILD: BaseComponent,
    CHILD::Properties: Into<SelectChild<K>> + Clone,
    K: 'static + Clone + PartialEq + Display,
{
    fn from(vchild: VChild<CHILD>) -> Self {
        Self {
            props: (*vchild.props).clone().into(),
        }
    }
}

impl<K> Into<Html> for SelectChildVariant<K>
where
    K: 'static + Clone + PartialEq + Display,
{
    fn into(self) -> Html {
        match self.props {
            SelectChild::Option(props) => VComp::new::<SelectOption<K>>(props, None).into(),
            SelectChild::Divider(props) => VComp::new::<ListDivider>(props, None).into(),
            SelectChild::Group(props) => VComp::new::<SelectGroup<K>>(props, None).into(),
        }
    }
}

// Item

#[derive(Clone, PartialEq, Properties)]
pub struct SelectOptionProperties<K>
where
    K: Clone + PartialEq + Display,
{
    pub value: K,

    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub description: Option<String>,

    #[prop_or_default]
    pub onclick: Option<Callback<K>>,

    #[prop_or_default]
    pub(crate) want_close: Callback<()>,

    #[prop_or_default]
    pub(crate) want_clicked: Callback<K>,

    #[prop_or_default]
    pub(crate) variant: SelectVariant<K>,

    #[prop_or_default]
    pub(crate) selected: bool,
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug)]
pub enum SelectOptionMsg {
    Clicked,
}

pub struct SelectOption<K>
where
    K: 'static + Clone + PartialEq + Display,
{
    default_id: Cell<Option<String>>,
    _marker: PhantomData<K>,
}

impl<K> Component for SelectOption<K>
where
    K: 'static + Clone + PartialEq + Display,
{
    type Message = SelectOptionMsg;
    type Properties = SelectOptionProperties<K>;

    fn create(_: &Context<Self>) -> Self {
        Self {
            default_id: Default::default(),
            _marker: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Clicked => {
                if let Some(onclick) = &ctx.props().onclick {
                    // if we have a click handler, we don't send the default handling
                    onclick.emit(ctx.props().value.clone());
                } else {
                    // default is to report clicked, if we have a key
                    ctx.props().want_clicked.emit(ctx.props().value.clone());
                }
                if matches!(ctx.props().variant, SelectVariant::Single(_)) {
                    // request close from our parent, only when we are neither multi nor checkbox
                    ctx.props().want_close.emit(());
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match ctx.props().variant {
            SelectVariant::Single(_) => self.render_button(ctx),
            SelectVariant::Multiple(_) => self.render_button(ctx),
            SelectVariant::Checkbox(_) => self.render_checkbox(ctx),
        }
    }
}

impl<K> SelectOption<K>
where
    K: Clone + PartialEq + Display,
{
    fn render_button(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-v5-c-select__menu-item");

        if ctx.props().selected {
            classes.push("pf-m-selected");
        }

        if ctx.props().description.is_some() {
            classes.push("pf-m-description");
        }

        html! (
            <li role="presentation">
                <button
                    class={classes}
                    role="option"
                    onclick={ctx.link().callback(|_|SelectOptionMsg::Clicked)}
                    >
                { if let Some(description) = &ctx.props().description {
                    html!{
                        <>
                        <span class="pf-v5-c-select__menu-item-main">{ &ctx.props().value }</span>
                        <span class="pf-v5-c-select__menu-item-description">{ &description }</span>
                        { self.render_selected(ctx) }
                        </>
                    }
                } else {
                    html!{
                        <>
                        { &ctx.props().value }
                        { self.render_selected(ctx) }
                        </>
                    }
                }}
                </button>
            </li>
        )
    }

    fn render_checkbox(&self, ctx: &Context<Self>) -> Html {
        let classes = Classes::from("pf-v5-c-check pf-v5-c-select__menu-item");

        let id = ctx.props().id.clone().unwrap_or_else(|| {
            let id = self
                .default_id
                .take()
                .unwrap_or_else(|| Uuid::new_v4().to_string());
            self.default_id.set(Some(id.clone()));
            id
        });

        html! (
            <label
                class={classes}
                for={id.clone()}
            >
                <input
                    id={id}
                    class="pf-v5-c-check__input"
                    type="checkbox"
                    checked={ctx.props().selected}
                    onclick={ctx.link().callback(|_|SelectOptionMsg::Clicked)}
                    />
                <span class="pf-v5-c-check__label">{ &ctx.props().value }</span>

                {if let Some(description) = &ctx.props().description {
                        html!{
                            <>
                            <span class="pf-v5-c-check__description">{&description}</span>
                            </>
                        }
                    }
                    else {
                        html! {}
                }}
            </label>
        )
    }

    fn render_selected(&self, ctx: &Context<Self>) -> Html {
        html! (
            if ctx.props().selected {
                <span class="pf-v5-c-select__menu-item-icon">{ Icon::Check }</span>
            }
        )
    }
}

// Group

#[derive(Clone, PartialEq, Properties)]
pub struct SelectGroupProperties<K>
where
    K: 'static + Clone + PartialEq + Display,
{
    pub label: String,
    #[prop_or_default]
    pub children: ChildrenRenderer<SelectChildVariant<K>>,
    #[prop_or_default]
    pub(crate) selection: Vec<K>,
    #[prop_or_default]
    pub(crate) want_close: Callback<()>,
    #[prop_or_default]
    pub(crate) want_clicked: Callback<K>,
    #[prop_or_default]
    pub(crate) variant: SelectVariant<K>,
}

#[derive(Clone)]
pub struct SelectGroup<K>
where
    K: 'static + Clone + PartialEq + Display,
{
    _marker: PhantomData<K>,
}

#[derive(Clone, Debug)]
pub enum SelectGroupMsg<K> {
    Close,
    Clicked(K),
}

impl<K> Component for SelectGroup<K>
where
    K: Clone + PartialEq + Display,
{
    type Message = SelectGroupMsg<K>;
    type Properties = SelectGroupProperties<K>;

    fn create(_: &Context<Self>) -> Self {
        Self {
            _marker: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Close => ctx.props().want_close.emit(()),
            Self::Message::Clicked(k) => ctx.props().want_clicked.emit(k),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! (
            <>
                <div class="pf-v5-c-select__menu-group">
                    <div class="pf-v5-c-select__menu-group-title" aria-hidden="true">
                        { &ctx.props().label }
                    </div>
                    { for ctx.props().children.iter().map(|mut c|{
                        c.set_need_close(ctx.link().callback(|_|Self::Message::Close));
                        c.set_need_clicked(ctx.link().callback(|k|Self::Message::Clicked(k)));
                        c.set_variant(ctx.props().variant.clone());
                        c.set_selection(&ctx.props().selection);
                        c
                    })}
                </div>
            </>
        )
    }
}
