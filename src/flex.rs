use crate::{AsClasses, ExtendClasses, SpaceItems, Spacer, WithBreakpoints};
use std::fmt::Debug;
use std::rc::Rc;
use yew::{
    html::ChildrenRenderer,
    prelude::*,
    virtual_dom::{VChild, VComp},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FlexModifier {
    Grow,
    Shrink,
    Flex1,
    Flex2,
    Flex3,
    Flex4,
    FullWidth,
    Default,
    None,
    Column,
    Justify(Justify),
    Align(Alignment),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Justify {
    Start,
    End,
    SpaceBetween,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Alignment {
    Right,
    Left,
    Start,
    Center,
    End,
    Baseline,
    Stretch,
}

impl AsClasses for FlexModifier {
    fn extend(&self, classes: &mut Classes) {
        match self {
            FlexModifier::Grow => classes.push("pf-m-grow"),
            FlexModifier::Shrink => classes.push("pf-m-shrink"),
            FlexModifier::Flex1 => classes.push("pf-m-flex-1"),
            FlexModifier::Flex2 => classes.push("pf-m-flex-2"),
            FlexModifier::Flex3 => classes.push("pf-m-flex-3"),
            FlexModifier::Flex4 => classes.push("pf-m-flex-4"),
            FlexModifier::FullWidth => classes.push("pf-m-full-width"),
            FlexModifier::Default => classes.push("pf-m-default"),
            FlexModifier::None => classes.push("pf-m-none"),
            FlexModifier::Column => classes.push("pf-m-column"),
            FlexModifier::Justify(layout) => match layout {
                Justify::Start => classes.push("pf-m-justify-content-flex-start"),
                Justify::End => classes.push("pf-m-justify-content-flex-end"),
                Justify::SpaceBetween => classes.push("pf-m-justify-content-space-between"),
            },
            FlexModifier::Align(alignement) => match alignement {
                Alignment::Right => classes.push("pf-m-align-right"),
                Alignment::Left => classes.push("pf-m-align-left"),
                Alignment::Start => classes.push("pf-m-align-self-flex-start"),
                Alignment::Center => classes.push("pf-m-align-self-flex-center"),
                Alignment::End => classes.push("pf-m-align-self-flex-end"),
                Alignment::Baseline => classes.push("pf-m-align-self-flex-baseline"),
                Alignment::Stretch => classes.push("pf-m-align-self-flex-stretch"),
            },
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum FlexChild {
    Flex(Rc<<Flex as BaseComponent>::Properties>),
    FlexItem(Rc<<FlexItem as BaseComponent>::Properties>),
}

impl From<FlexProps> for FlexChild {
    fn from(props: FlexProps) -> Self {
        FlexChild::Flex(Rc::new(props))
    }
}

impl From<FlexItemProps> for FlexChild {
    fn from(props: FlexItemProps) -> Self {
        FlexChild::FlexItem(Rc::new(props))
    }
}

#[derive(PartialEq, Clone)]
pub struct FlexChildVariant {
    props: FlexChild,
}

impl<CHILD> From<VChild<CHILD>> for FlexChildVariant
where
    CHILD: BaseComponent,
    CHILD::Properties: Into<FlexChild> + Clone,
{
    fn from(vchild: VChild<CHILD>) -> Self {
        Self {
            props: (*vchild.props).clone().into(),
        }
    }
}

impl Into<Html> for FlexChildVariant {
    fn into(self) -> Html {
        match self.props {
            FlexChild::Flex(props) => VComp::new::<Flex>(props, None).into(),
            FlexChild::FlexItem(props) => VComp::new::<FlexItem>(props, None).into(),
        }
    }
}

pub trait ToFlexItems {
    fn into_flex_items(self) -> Vec<VChild<FlexItem>>;
}

impl ToFlexItems for Vec<Html> {
    fn into_flex_items(self) -> Vec<VChild<FlexItem>> {
        self.into_iter()
            .map(|html| html_nested! {<FlexItem> { html }</FlexItem>})
            .collect()
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct FlexProps {
    #[prop_or_default]
    pub children: ChildrenRenderer<FlexChildVariant>,
    #[prop_or_default]
    pub modifiers: WithBreakpoints<FlexModifier>,
    #[prop_or_default]
    pub spacer: WithBreakpoints<Spacer>,
    #[prop_or_default]
    pub space_items: WithBreakpoints<SpaceItems>,
}

#[function_component(Flex)]
pub fn flex(props: &FlexProps) -> Html {
    let mut classes = Classes::from("pf-l-flex");

    classes.extend_from(&props.modifiers);
    classes.extend_from(&props.space_items);
    classes.extend_from(&props.spacer);

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}

// flex item

#[derive(Clone, PartialEq, Properties)]
pub struct FlexItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub modifiers: WithBreakpoints<FlexModifier>,
    #[prop_or_default]
    pub spacer: WithBreakpoints<Spacer>,
}

#[function_component(FlexItem)]
pub fn flex_item(props: &FlexItemProps) -> Html {
    let mut classes = Classes::from("pf-l-flex__item");

    classes.extend_from(&props.modifiers);
    classes.extend_from(&props.spacer);

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
