use crate::ChildrenProperties;
use yew::prelude::*;

#[function_component(Raw)]
pub fn raw(props: &ChildrenProperties) -> Html {
    props.children.iter().collect()
}
