use crate::{Badge, Button, Divider, GlobalClose, Icon, Variant};
use std::cell::Cell;
use std::fmt::{Debug, Display};
use uuid::Uuid;
use yew::{
    html::ChildrenRenderer,
    prelude::*,
    virtual_dom::{VChild, VComp},
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

#[derive(Clone, PartialEq, Properties)]
pub struct Props<K: 'static + Clone + PartialEq + Display + Debug> {
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
    pub placeholder: String,

    #[prop_or_default]
    pub variant: SelectVariant<K>,

    #[prop_or_default]
    pub children: ChildrenRenderer<SelectChildVariant<K>>,
}

pub struct Select<K>
where
    K: 'static + Clone + PartialEq + Display + Debug,
{
    props: Props<K>,
    link: ComponentLink<Self>,

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
    type Properties = Props<K>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            expanded: false,
            props,
            global_close: GlobalClose::new(NodeRef::default(), link.callback(|_| Msg::Close)),
            link,
            selection: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle => {
                self.expanded = !self.expanded;
            }
            Msg::Close => self.expanded = false,
            Msg::Clicked(k) => self.clicked(k),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("pf-c-select");
        if self.expanded {
            classes.push("pf-m-expanded");
        }
        if self.props.top {
            classes.push("pf-m-top");
        }

        let menu_classes = Classes::from("pf-c-select__menu");

        let onclick = self.link.callback(|_| Msg::Toggle);

        let variant = match self.props.plain {
            true => Variant::Plain,
            false => Variant::None,
        };

        return html! {
            <div class=classes
                ref=self.global_close.clone()>
                <Button
                    class="pf-c-select__toggle"
                    variant=variant
                    r#type="button"
                    disabled=self.props.disabled
                    onclick=onclick
                    id=&self.props.id
                    >
                    <div class="pf-c-select__toggle-wrapper">
                        { self.render_selection() }
                    </div>
                    <div class="pf-c-select__toggle-arrow">
                        { Icon::CaretDown }
                    </div>
                </Button>
                <div
                    class=menu_classes
                    hidden=!self.expanded
                >
                    {
                        match self.props.variant {
                            SelectVariant::Single(_) => self.render_button(),
                            SelectVariant::Multiple(_) => self.render_button(),
                            SelectVariant::Checkbox(_) => self.render_checkbox(),
                        }
                    }
                </div>
            </div>
        };
    }
}

impl<K> Select<K>
where
    K: 'static + Clone + PartialEq + Display + Debug,
{
    fn render_selection(&self) -> Html {
        let selection = self.selection.as_slice();
        if selection.is_empty() {
            return html! {<span class="pf-c-select__toggle-text">{&self.props.placeholder}</span>};
        }

        match &self.props.variant {
            SelectVariant::Single(_) => {
                return html! {<span class="pf-c-select__toggle-text">{ &selection[0] }</span>};
            }
            SelectVariant::Checkbox(_) | SelectVariant::Multiple(_) => {
                return html! {
                    <>
                        <span class="pf-c-select__toggle-text">{&self.props.placeholder}</span>
                        <div class="pf-c-select__toggle_badge">
                            <Badge read=true>{selection.len()}</Badge>
                        </div>
                    </>
                };
            }
        }
    }

    fn render_button(&self) -> Html {
        return html! {
            <ul>
                { for self.props.children.iter().map(|mut c|{
                    // request a close callback from the item
                    c.set_need_close(self.link.callback(|_|Msg::Close));
                    c.set_need_clicked(self.link.callback(|k|Msg::Clicked(k)));
                    c.set_variant(self.props.variant.clone());
                    c
                }) }
            </ul>
        };
    }

    fn render_checkbox(&self) -> Html {
        return html! {
            <fieldset class="pf-c-select__menu-fieldset" aria-label="Select input">
                { for self.props.children.iter().map(|mut c|{
                    // request a close callback from the item
                    c.set_need_close(self.link.callback(|_|Msg::Close));
                    c.set_need_clicked(self.link.callback(|k|Msg::Clicked(k)));
                    c.set_variant(self.props.variant.clone());
                    c
                }) }
            </fieldset>
        };
    }

    fn clicked(&mut self, key: K) {
        log::info!("Clicked: {}", key);
        match &self.props.variant {
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
    K: 'static + Clone + PartialEq + Display + Debug,
{
    Option(<SelectOption<K> as Component>::Properties),
    Divider(<Divider as Component>::Properties),
    Group(<SelectGroup<K> as Component>::Properties),
}

impl<K> From<SelectOptionProps<K>> for SelectChild<K>
where
    K: Clone + PartialEq + Display + Debug,
{
    fn from(props: SelectOptionProps<K>) -> Self {
        SelectChild::Option(props)
    }
}

impl<K> From<()> for SelectChild<K>
where
    K: Clone + PartialEq + Display + Debug,
{
    fn from(_: ()) -> Self {
        SelectChild::Divider(())
    }
}

impl<K> From<SelectGroupProps<K>> for SelectChild<K>
where
    K: Clone + PartialEq + Display + Debug,
{
    fn from(props: SelectGroupProps<K>) -> Self {
        SelectChild::Group(props)
    }
}

// variant

#[derive(PartialEq, Clone)]
pub struct SelectChildVariant<K>
where
    K: 'static + Clone + PartialEq + Display + Debug,
{
    props: SelectChild<K>,
}

impl<K> SelectChildVariant<K>
where
    K: Clone + PartialEq + Display + Debug,
{
    /// Forward the need to get a close callback to the actual item
    fn set_need_close(&mut self, callback: Callback<()>) {
        match self.props {
            SelectChild::Option(ref mut props) => {
                props.want_close = callback;
            }
            SelectChild::Group(ref mut props) => {
                props.want_close = callback;
            }
            _ => {}
        }
    }

    fn set_need_clicked(&mut self, callback: Callback<K>) {
        match self.props {
            SelectChild::Option(ref mut props) => {
                props.want_clicked = callback;
            }
            SelectChild::Group(ref mut props) => {
                props.want_clicked = callback;
            }
            _ => {}
        }
    }

    fn set_variant(&mut self, variant: SelectVariant<K>) {
        match self.props {
            SelectChild::Option(ref mut props) => {
                props.variant = variant;
            }
            SelectChild::Group(ref mut props) => {
                props.variant = variant;
            }
            _ => {}
        }
    }
}

impl<K, CHILD> From<VChild<CHILD>> for SelectChildVariant<K>
where
    CHILD: Component,
    CHILD::Properties: Into<SelectChild<K>>,
    K: 'static + Clone + PartialEq + Display + Debug,
{
    fn from(vchild: VChild<CHILD>) -> Self {
        Self {
            props: vchild.props.into(),
        }
    }
}

impl<K> Into<Html> for SelectChildVariant<K>
where
    K: 'static + Clone + PartialEq + Display + Debug,
{
    fn into(self) -> Html {
        match self.props {
            SelectChild::Option(props) => {
                VComp::new::<SelectOption<K>>(props, NodeRef::default(), None).into()
            }
            SelectChild::Divider(props) => {
                VComp::new::<Divider>(props, NodeRef::default(), None).into()
            }
            SelectChild::Group(props) => {
                VComp::new::<SelectGroup<K>>(props, NodeRef::default(), None).into()
            }
        }
    }
}

// Item

#[derive(Clone, PartialEq, Properties)]
pub struct SelectOptionProps<K>
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
    pub(crate) want_close: Callback<()>,

    #[prop_or_default]
    pub(crate) want_clicked: Callback<K>,

    #[prop_or_default]
    pub(crate) variant: SelectVariant<K>,
}

#[derive(Clone, Copy, Debug)]
pub enum SelectOptionMsg {
    Clicked,
}

pub struct SelectOption<K>
where
    K: 'static + Clone + PartialEq + Display + Debug,
{
    props: SelectOptionProps<K>,
    link: ComponentLink<Self>,
    default_id: Cell<Option<String>>,
}

impl<K> Component for SelectOption<K>
where
    K: 'static + Clone + PartialEq + Display + Debug,
{
    type Message = SelectOptionMsg;
    type Properties = SelectOptionProps<K>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            default_id: Default::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Self::Message::Clicked => {
                log::info!("Clicked on: {:?}", self.props.value);
                if let Some(onclick) = &self.props.onclick {
                    // if we have a click handler, we don't send the default handling
                    onclick.emit(self.props.value.clone());
                } else {
                    // default is to report clicked, if we have a key
                    self.props.want_clicked.emit(self.props.value.clone());
                }
                if matches!(self.props.variant, SelectVariant::Single(_)) {
                    // request close from our parent, only when we are neither multi nor checkbox
                    self.props.want_close.emit(());
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        match self.props.variant {
            SelectVariant::Single(_) => self.render_button(),
            SelectVariant::Multiple(_) => self.render_button(),
            SelectVariant::Checkbox(_) => self.render_checkbox(),
        }
    }
}

impl<K> SelectOption<K>
where
    K: Clone + PartialEq + Display + Debug,
{
    fn render_button(&self) -> Html {
        let mut classes = Classes::from("pf-c-select__menu-item");

        if self.props.selected {
            classes.push("pf-m-selected");
        }

        if self.props.description.is_some() {
            classes.push("pf-m-description");
        }

        return html! {
            <li role="presentation">
                <button
                    class=classes
                    role="option"
                    onclick=self.link.callback(|_|SelectOptionMsg::Clicked)
                    >
                { if let Some(description) = &self.props.description {
                    html!{
                        <>
                        <span class="pf-c-select__menu-item-main">{ &self.props.value }</span>
                        <span class="pf-c-select__menu-item-description">{ &description }</span>
                        { self.render_selected() }
                        </>
                    }
                } else {
                    html!{
                        <>
                        { &self.props.value }
                        { self.render_selected() }
                        </>
                    }
                }}
                </button>
            </li>
        };
    }

    fn render_checkbox(&self) -> Html {
        let mut classes = Classes::from("pf-c-check pf-c-select__menu-item");

        if self.props.selected {
            classes.push("pf-m-selected");
        }

        if self.props.description.is_some() {
            classes.push("pf-m-description");
        }

        let id = self.props.id.clone().unwrap_or_else(|| {
            let id = self
                .default_id
                .take()
                .unwrap_or_else(|| Uuid::new_v4().to_string());
            self.default_id.set(Some(id.clone()));
            id
        });

        return html! {
            <label
                class=classes
                for=id
            >
                <input
                    id=id
                    class="pf-c-check__input"
                    type="checkbox"
                    checked=self.props.selected
                    onclick=self.link.callback(|_|SelectOptionMsg::Clicked)
                    />
                <span class="pf-c-check__label">{ &self.props.value }</span>
            </label>
        };
    }

    fn render_selected(&self) -> Html {
        if self.props.selected {
            return html! {
                <span class="pf-c-select__menu-item-icon">{ Icon::Check }</span>
            };
        } else {
            return html! {};
        }
    }
}

// Group

#[derive(Clone, PartialEq, Properties)]
pub struct SelectGroupProps<K>
where
    K: 'static + Clone + PartialEq + Display + Debug,
{
    pub label: String,
    #[prop_or_default]
    pub children: ChildrenRenderer<SelectChildVariant<K>>,
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
    K: 'static + Clone + PartialEq + Display + Debug,
{
    props: SelectGroupProps<K>,
    link: ComponentLink<Self>,
}

#[derive(Clone, Debug)]
pub enum SelectGroupMsg<K> {
    Close,
    Clicked(K),
}

impl<K> Component for SelectGroup<K>
where
    K: Clone + PartialEq + Display + Debug,
{
    type Message = SelectGroupMsg<K>;
    type Properties = SelectGroupProps<K>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Self::Message::Close => self.props.want_close.emit(()),
            Self::Message::Clicked(k) => self.props.want_clicked.emit(k),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        return html! {
            <>
                <div class="pf-c-select__menu-group">
                    <div class="pf-c-select__menu-group-title" aria-hidden="true">
                        { &self.props.label }
                    </div>
                    { for self.props.children.iter().map(|mut c|{
                        c.set_need_close(self.link.callback(|_|Self::Message::Close));
                        c.set_need_clicked(self.link.callback(|k|Self::Message::Clicked(k)));
                        c.set_variant(self.props.variant.clone());
                        c
                    })}
                </div>
            </>
        };
    }
}
