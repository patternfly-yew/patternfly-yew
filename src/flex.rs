use std::fmt::Debug;
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew::virtual_dom::{VChild, VComp};

/*
 * Grow.on(Lg)
 */

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Breakpoint {
    None,
    Small,
    Medium,
    Large,
    XLarge,
    XXLarge,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WithBreakpoint<T>
where
    T: Clone + Debug + PartialEq,
{
    modifier: T,
    on: Breakpoint,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FlexModifier {
    Grow,
    Shrink,
    Flex1,
    Flex2,
    Flex3,
    Flex4,
    Default,
    None,
    Column,
}

impl ToString for FlexModifier {
    fn to_string(&self) -> String {
        match self {
            FlexModifier::Grow => "pf-m-grow",
            FlexModifier::Shrink => "pf-m-shrink",
            FlexModifier::Flex1 => "pf-m-flex-1",
            FlexModifier::Flex2 => "pf-m-flex-2",
            FlexModifier::Flex3 => "pf-m-flex-3",
            FlexModifier::Flex4 => "pf-m-flex-4",
            FlexModifier::Default => "pf-m-default",
            FlexModifier::None => "pf-m-none",
            FlexModifier::Column => "pf-m-column",
        }
        .to_string()
    }
}

impl ToString for Breakpoint {
    fn to_string(&self) -> String {
        match self {
            Breakpoint::None => "",
            Breakpoint::Small => "-on-sm",
            Breakpoint::Medium => "-on-md",
            Breakpoint::Large => "-on-lg",
            Breakpoint::XLarge => "-on-xl",
            Breakpoint::XXLarge => "-on-2xl",
        }
        .to_string()
    }
}

pub trait WithBreakpointExt<T>
where
    T: Clone + Debug + PartialEq + ToString,
{
    fn on(&self, breakpoint: Breakpoint) -> WithBreakpoint<T>;

    fn all(&self) -> WithBreakpoint<T> {
        self.on(Breakpoint::None)
    }

    fn sm(&self) -> WithBreakpoint<T> {
        self.on(Breakpoint::Small)
    }
    fn md(&self) -> WithBreakpoint<T> {
        self.on(Breakpoint::Medium)
    }
    fn lg(&self) -> WithBreakpoint<T> {
        self.on(Breakpoint::Large)
    }
    fn xl(&self) -> WithBreakpoint<T> {
        self.on(Breakpoint::XLarge)
    }
    fn xxl(&self) -> WithBreakpoint<T> {
        self.on(Breakpoint::XXLarge)
    }
}

impl<T> WithBreakpointExt<T> for T
where
    T: Clone + Debug + PartialEq + ToString,
{
    fn on(&self, breakpoint: Breakpoint) -> WithBreakpoint<T> {
        WithBreakpoint {
            modifier: self.clone(),
            on: breakpoint,
        }
    }
}

impl<T> ToString for WithBreakpoint<T>
where
    T: Clone + Debug + PartialEq + ToString,
{
    fn to_string(&self) -> String {
        format!("{}{}", self.modifier.to_string(), self.on.to_string())
    }
}

impl<T> Into<Classes> for WithBreakpoint<T>
where
    T: Clone + Debug + PartialEq + ToString,
{
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
}

trait AsClasses {
    fn as_classes(&self) -> Classes;
}

impl<T> AsClasses for Vec<WithBreakpoint<T>>
where
    T: Clone + Debug + PartialEq + ToString,
{
    fn as_classes(&self) -> Classes {
        self.iter()
            .map(|b| b.to_string())
            .collect::<String>()
            .into()
    }
}

impl<T> From<T> for WithBreakpoint<T>
where
    T: Clone + Debug + PartialEq + ToString,
{
    fn from(modifier: T) -> Self {
        Self {
            on: Breakpoint::None,
            modifier,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum FlexChild {
    Flex(<Flex as Component>::Properties),
    FlexItem(<FlexItem as Component>::Properties),
}

impl From<FlexProps> for FlexChild {
    fn from(props: FlexProps) -> Self {
        FlexChild::Flex(props)
    }
}

impl From<FlexItemProps> for FlexChild {
    fn from(props: FlexItemProps) -> Self {
        FlexChild::FlexItem(props)
    }
}

#[derive(PartialEq, Clone)]
pub struct FlexChildVariant {
    props: FlexChild,
}

impl<CHILD> From<VChild<CHILD>> for FlexChildVariant
where
    CHILD: Component,
    CHILD::Properties: Into<FlexChild>,
{
    fn from(vchild: VChild<CHILD>) -> Self {
        Self {
            props: vchild.props.into(),
        }
    }
}

impl Into<Html> for FlexChildVariant {
    fn into(self) -> Html {
        match self.props {
            FlexChild::Flex(props) => VComp::new::<Flex>(props, NodeRef::default(), None).into(),
            FlexChild::FlexItem(props) => {
                VComp::new::<FlexItem>(props, NodeRef::default(), None).into()
            }
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct FlexProps {
    #[prop_or_default]
    pub children: ChildrenRenderer<FlexChildVariant>,
    #[prop_or_default]
    pub modifiers: Vec<WithBreakpoint<FlexModifier>>,
}

#[derive(Clone, PartialEq)]
pub struct Flex {
    props: FlexProps,
}

impl Component for Flex {
    type Message = ();
    type Properties = FlexProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("pf-l-flex");

        classes = classes.extend(self.props.modifiers.as_classes());

        return html! {
            <div class=classes>
                { for self.props.children.iter() }
            </div>
        };
    }
}

// flex item

#[derive(Clone, PartialEq, Properties)]
pub struct FlexItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub modifiers: Vec<WithBreakpoint<FlexModifier>>,
}

#[derive(Clone, PartialEq)]
pub struct FlexItem {
    props: FlexItemProps,
}

impl Component for FlexItem {
    type Message = ();
    type Properties = FlexItemProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("pf-l-flex__item");

        classes = classes.extend(self.props.modifiers.as_classes());

        return html! {
            <div class=classes>
            { for self.props.children.iter() }
            </div>
        };
    }
}
