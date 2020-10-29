use yew::prelude::*;

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Copy, Debug)]
pub enum Level {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl Default for Level {
    fn default() -> Self {
        Level::H1
    }
}

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Copy, Debug)]
pub enum Size {
    Medium,
    Large,
    XLarge,
    XXLarge,
    XXXLarge,
    XXXXLarge,
}

impl Size {
    pub fn as_class(&self) -> &str {
        match self {
            Size::Medium => "pf-m-md",
            Size::Large => "pf-m-lg",
            Size::XLarge => "pf-m-xl",
            Size::XXLarge => "pf-m-2xl",
            Size::XXXLarge => "pf-m-3xl",
            Size::XXXXLarge => "pf-m-4xl",
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub level: Level,
    #[prop_or_default]
    pub size: Option<Size>,
}

pub struct Title {
    props: Props,
}

impl Component for Title {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
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
        let mut classes = Classes::from("pf-c-title");

        if let Some(size) = self.props.size {
            classes.push(size.as_class());
        }

        match self.props.level {
            Level::H1 => html! {<h1 class=classes>{ for self.props.children.iter() }</h1>},
            Level::H2 => html! {<h2 class=classes>{ for self.props.children.iter() }</h2>},
            Level::H3 => html! {<h3 class=classes>{ for self.props.children.iter() }</h3>},
            Level::H4 => html! {<h4 class=classes>{ for self.props.children.iter() }</h4>},
            Level::H5 => html! {<h5 class=classes>{ for self.props.children.iter() }</h5>},
            Level::H6 => html! {<h6 class=classes>{ for self.props.children.iter() }</h6>},
        }
    }
}
