use crate::{AsClasses, ToolbarElementModifier, WithBreakpoint};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarGroupProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub modifiers: Vec<WithBreakpoint<ToolbarElementModifier>>,
}

pub struct ToolbarGroup {
    props: ToolbarGroupProps,
}

impl Component for ToolbarGroup {
    type Message = ();
    type Properties = ToolbarGroupProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
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
        let mut classes = Classes::from("pf-c-toolbar__group");

        classes = classes.extend(self.props.modifiers.as_classes());

        return html! {
            <div class=classes>
                { for self.props.children.iter() }
            </div>
        };
    }
}
