use super::*;
use yew::{
    prelude::*,
    virtual_dom::{VChild, VComp},
};

#[derive(PartialEq, Clone)]
pub struct ProgressStepperChildVariant {
    props: ProgressStepperChild,
}

impl<CHILD> From<VChild<CHILD>> for ProgressStepperChildVariant
where
    CHILD: BaseComponent,
    CHILD::Properties: Into<ProgressStepperChild> + Clone,
{
    fn from(vchild: VChild<CHILD>) -> Self {
        Self {
            props: (*vchild.props).clone().into(),
        }
    }
}

impl From<ProgressStepperChildVariant> for Html {
    fn from(value: ProgressStepperChildVariant) -> Self {
        match value.props {
            ProgressStepperChild::Step(props) => {
                VComp::new::<ProgressStepperStep>(props, None).into()
            }
        }
    }
}
