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

#[function_component(Grid)]
pub fn grid(props: &Props) -> Html {
    let mut classes = Classes::from("pf-l-grid");

    if props.gutter {
        classes.push("pf-m-gutter");
    }

    classes.extend(props.cols.mapped(|cols| format!("pf-m-all-{}-col", cols)));

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
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

#[function_component(GridItem)]
pub fn grid_item(props: &GridItemProps) -> Html {
    let mut classes = Classes::from("pf-l-grid__item");

    classes.extend(props.cols.mapped(|cols| format!("pf-m-{}-col", cols)));
    classes.extend(props.rows.mapped(|cols| format!("pf-m-{}-row", cols)));
    classes.extend(
        props
            .offset
            .mapped(|cols| format!("pf-m-offset-{}-col", cols)),
    );

    html! {
            <div class={classes}>
                { for props.children.iter() }
            </div>
    }
}
