use yew::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PageSectionVariant {
    Default,
    Darker,
    Dark,
    Light,
}

impl Default for PageSectionVariant {
    fn default() -> Self {
        Self::Default
    }
}

impl PageSectionVariant {
    pub fn as_class(&self) -> Vec<String> {
        match self {
            Self::Default => vec![],
            Self::Darker => vec!["pf-m-dark-100".into()],
            Self::Dark => vec!["pf-m-dark-200".into()],
            Self::Light => vec!["pf-m-light".into()],
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub variant: PageSectionVariant,
    #[prop_or_default]
    pub fill: bool,
    #[prop_or_default]
    pub limit_width: bool,
}

#[derive(Clone, PartialEq)]
pub struct PageSection {}

impl Component for PageSection {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <section class={self.collect_classes(ctx)}>
                { self.children(ctx) }
            </section>
        }
    }
}

impl PageSection {
    fn collect_classes(&self, ctx: &Context<Self>) -> Classes {
        let mut classes = Classes::from("pf-c-page__main-section");
        classes.extend(ctx.props().variant.as_class());

        if ctx.props().fill {
            classes.push("pf-m-fill");
        }

        if ctx.props().limit_width {
            classes.push("pf-m-limit-width");
        }

        classes
    }

    fn children(&self, ctx: &Context<Self>) -> Html {
        let c = html! {
            <>
            { for ctx.props().children.iter() }
            </>
        };

        match ctx.props().limit_width {
            true => {
                html! {
                    <div class="pf-c-page__main-body">
                        { c }
                    </div>
                }
            }
            false => c,
        }
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PageTabsProperties {
    #[prop_or_default]
    pub limit_width: bool,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(PageTabs)]
pub fn page_tabs(props: &PageTabsProperties) -> Html {
    let mut classes = Classes::from("pf-c-page__main-tabs");

    if props.limit_width {
        classes.push("pf-m-limit-width");
    }

    html!(
        <section class={classes}>
            <div class="pf-c-page__main-body">
                { for props.children.iter() }
            </div>
        </section>
    )
}
