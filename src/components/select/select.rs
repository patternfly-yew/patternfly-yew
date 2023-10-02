use crate::prelude::*;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct SimpleSelectProperties<T>
where
    T: Clone + Eq + SelectItemRenderer,
{
    #[prop_or_default]
    pub placeholder: Option<String>,

    #[prop_or_default]
    pub entries: Vec<T>,

    #[prop_or_default]
    pub selected: Option<T>,

    #[prop_or_default]
    pub onselect: Callback<T>,
}

pub trait SelectItemRenderer {
    type Item;

    fn label(&self) -> String;
}

impl<T> SelectItemRenderer for T
where
    T: std::fmt::Display,
{
    type Item = T;

    fn label(&self) -> String {
        self.to_string()
    }
}

#[function_component(SimpleSelect)]
pub fn simple_select<T>(props: &SimpleSelectProperties<T>) -> Html
where
    T: Clone + Eq + SelectItemRenderer + 'static,
{
    let text = props
        .selected
        .as_ref()
        .map(|s| s.label())
        .or_else(|| props.placeholder.clone());

    html!(
        <Dropdown
            text={text.clone()}
        >
            { for props.entries.iter().map(|entry| {
                html_nested!(
                    <Raw>
                        <SimpleSelectItem<T>
                            entry={entry.clone()}
                            selected={props.selected.as_ref() == Some(entry)}
                            onselect={props.onselect.clone()}
                        />
                    </Raw>
                )
            }) }
        </Dropdown>
    )
}

#[derive(PartialEq, Properties)]
struct SimpleSelectItemProperties<T>
where
    T: Eq + SelectItemRenderer + 'static,
{
    entry: T,
    selected: bool,
    onselect: Callback<T>,
}

#[function_component(SimpleSelectItem)]
fn simple_select_item<T>(props: &SimpleSelectItemProperties<T>) -> Html
where
    T: Clone + Eq + SelectItemRenderer + 'static,
{
    let onclick = use_callback(
        (props.entry.clone(), props.onselect.clone()),
        |_, (entry, onselect)| {
            log::info!("Emit: {}", entry.label());
            onselect.emit(entry.clone());
        },
    );

    html!(
        <MenuAction
            {onclick}
            selected={props.selected}
        >
            { props.entry.label() }
        </MenuAction>)
}
