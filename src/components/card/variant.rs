use super::{CardBody, CardBodyProperties, CardDivider};
use crate::{ChildrenProperties, Raw};
use std::rc::Rc;
use yew::virtual_dom::VComp;
use yew::{prelude::*, virtual_dom::VChild};

#[derive(PartialEq, Clone)]
pub struct CardBodyVariant {
    props: CardBodyChild,
}

impl<CHILD> From<VChild<CHILD>> for CardBodyVariant
where
    CHILD: BaseComponent,
    CHILD::Properties: Into<CardBodyChild> + Clone,
{
    fn from(vchild: VChild<CHILD>) -> Self {
        Self {
            props: (*vchild.props).clone().into(),
        }
    }
}

impl Into<Html> for CardBodyVariant {
    fn into(self) -> Html {
        match self.props {
            CardBodyChild::Body(props) => VComp::new::<CardBody>(props, None).into(),
            CardBodyChild::Divider(props) => VComp::new::<CardDivider>(props, None).into(),
            CardBodyChild::Raw(props) => VComp::new::<Raw>(props, None).into(),
        }
    }
}

/// Child for an [`AppLauncher`]
#[derive(Clone, PartialEq)]
pub enum CardBodyChild {
    Body(Rc<<CardBody as BaseComponent>::Properties>),
    Divider(Rc<<CardDivider as BaseComponent>::Properties>),
    Raw(Rc<<Raw as BaseComponent>::Properties>),
}

impl From<CardBodyProperties> for CardBodyChild {
    fn from(props: CardBodyProperties) -> Self {
        CardBodyChild::Body(Rc::new(props))
    }
}

impl From<()> for CardBodyChild {
    fn from(_: ()) -> Self {
        CardBodyChild::Divider(Rc::new(()))
    }
}

impl From<ChildrenProperties> for CardBodyChild {
    fn from(props: ChildrenProperties) -> Self {
        CardBodyChild::Raw(Rc::new(props))
    }
}
