use crate::WithBreakpoints;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub gutter: bool,
    #[prop_or_default]
    pub cols: WithBreakpoints<usize>,
}

pub struct Grid {
    props: Props,
}

impl Component for Grid {
    type Message = ();
    type Properties = Props;

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
        let mut classes = Classes::from("pf-l-grid");

        if self.props.gutter {
            classes.push("pf-m-gutter");
        }

        classes = classes.extend(
            self.props
                .cols
                .mapped(|cols| format!("pf-m-all-{}-col", cols)),
        );

        return html! {
            <div class=classes>
                { for self.props.children.iter() }
            </div>
        };
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct GridItemProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub cols: WithBreakpoints<u16>,
    #[prop_or_default]
    pub rows: WithBreakpoints<u16>,
    #[prop_or_default]
    pub offset: WithBreakpoints<u16>,
}

pub struct GridItem {
    props: GridItemProps,
}

impl Component for GridItem {
    type Message = ();
    type Properties = GridItemProps;

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
        let mut classes = Classes::from("pf-l-grid__item");

        classes = classes.extend(self.props.cols.mapped(|cols| format!("pf-m-{}-col", cols)));
        classes = classes.extend(self.props.rows.mapped(|cols| format!("pf-m-{}-row", cols)));
        classes = classes.extend(
            self.props
                .offset
                .mapped(|cols| format!("pf-m-offset-{}-col", cols)),
        );

        return html! {
            <div class=classes>
                { for self.props.children.iter() }
            </div>
        };
    }
}
