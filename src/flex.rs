use crate::{AsClasses, SpaceItems, Spacer, WithBreakpoint};
use std::fmt::Debug;
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
    pub modifiers: Vec<WithBreakpoint<FlexModifier>>,
    #[prop_or_default]
    pub spacer: Vec<WithBreakpoint<Spacer>>,
    #[prop_or_default]
    pub space_items: Vec<WithBreakpoint<SpaceItems>>,
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
        classes = classes.extend(self.props.space_items.as_classes());
        classes = classes.extend(self.props.spacer.as_classes());

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
    #[prop_or_default]
    pub spacer: Vec<WithBreakpoint<Spacer>>,
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
        classes = classes.extend(self.props.spacer.as_classes());

        return html! {
            <div class=classes>
                { for self.props.children.iter() }
            </div>
        };
    }
}
