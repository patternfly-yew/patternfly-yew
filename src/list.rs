use yew::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ListType {
    Basic,
    Inline,
    Ordered(ListOrder),
}

impl Default for ListType {
    fn default() -> Self {
        Self::Basic
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ListOrder {
    Number,
    LowercaseLetter,
    UppercaseLetter,
    LowercaseRomanNumber,
    UppercaseRomanNumber,
}

impl Default for ListOrder {
    fn default() -> Self {
        Self::Number
    }
}

impl ToString for ListOrder {
    fn to_string(&self) -> String {
        match self {
            Self::Number => "1",
            Self::LowercaseLetter => "a",
            Self::UppercaseLetter => "A",
            Self::LowercaseRomanNumber => "i",
            Self::UppercaseRomanNumber => "I",
        }
        .into()
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub r#type: ListType,
}

#[derive(Clone, PartialEq)]
pub struct List {
    props: Props,
}

impl Component for List {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
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
        let mut classes = Classes::from("pf-c-list");

        if let ListType::Inline = self.props.r#type {
            classes.push("pf-m-inline");
        }

        let l: Box<dyn FnOnce(Html) -> Html> = match self.props.r#type {
            ListType::Basic | ListType::Inline => {
                Box::new(|items| html! {<ul class=classes>{ items }</ul>})
            }
            ListType::Ordered(n) => {
                Box::new(move |items| html! {<ol r#type=n class=classes>{ items }</ol>})
            }
        };

        l(html! {
            {
             for self.props.children.iter()
                .map(|item|html!{<li>{item}</li>})
            }
        })
    }
}
