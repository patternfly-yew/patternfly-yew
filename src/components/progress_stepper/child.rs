use super::{ProgressStepperStep, ProgressStepperStepProperties};
use std::rc::Rc;
use yew::BaseComponent;

#[derive(Clone, PartialEq)]
pub enum ProgressStepperChild {
    Step(Rc<<ProgressStepperStep as BaseComponent>::Properties>),
}

impl From<ProgressStepperStepProperties> for ProgressStepperChild {
    fn from(props: ProgressStepperStepProperties) -> Self {
        ProgressStepperChild::Step(Rc::new(props))
    }
}
