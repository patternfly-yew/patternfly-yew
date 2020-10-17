#![recursion_limit = "256"]
mod button;
mod form;
mod page;
mod pagesection;
mod pagesidebar;

use button::*;
use form::*;
use page::*;
use pagesection::*;
use pagesidebar::*;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Variant {
    Primary,
    Secondary,
    Link,
}

impl Variant {
    pub fn as_class(&self) -> &str {
        match self {
            Variant::Primary => "pf-m-primary",
            Variant::Secondary => "pf-m-secondary",
            Variant::Link => "pf-m-link",
        }
    }
}

impl Default for Variant {
    fn default() -> Self {
        Variant::Primary
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Icon {
    PlusCircleIcon,
}

impl Icon {
    pub fn as_class(&self) -> &str {
        match self {
            Icon::PlusCircleIcon => "fas fa-plus-circle",
        }
    }
}

pub struct WeakComponentLink<COMP: Component>(Rc<RefCell<Option<ComponentLink<COMP>>>>);

impl<COMP: Component> Clone for WeakComponentLink<COMP> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<COMP: Component> Default for WeakComponentLink<COMP> {
    fn default() -> Self {
        Self(Rc::default())
    }
}

impl<COMP: Component> Deref for WeakComponentLink<COMP> {
    type Target = Rc<RefCell<Option<ComponentLink<COMP>>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<COMP: Component> PartialEq for WeakComponentLink<COMP> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let sidebar = html_nested! {
            <PageSidebar>
            </PageSidebar>
        };
        let header_tools = html! { {"Foo"} };

        html! {
            <Page sidebar=sidebar tools=header_tools>
                <PageSection variant=PageSectionVariant::Light>
                    <div>{"Foo"}</div>
                </PageSection>
                <PageSection fill=true>
                    <Form>
                        <Button label="Add One" icon=Some(Icon::PlusCircleIcon) variant=Variant::Link onclick=self.link.callback(|_| Msg::AddOne)/>
                        <p>{ self.value }</p>
                    </Form>
                </PageSection>
            </Page>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
