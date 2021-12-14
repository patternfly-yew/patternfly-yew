use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub filter: Option<String>,
}

pub struct Background {}

impl Component for Background {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(filter) = &ctx.props().filter {
            let styles = format!("--pf-c-background-image--Filter: {};", filter);
            html! {
                <div class="pf-c-background-image" style={styles}></div>
            }
        } else {
            // FIXME: something is still wrong here, the filter gets applied, but seems to have no effect
            html! {
                <div class="pf-c-background-image">
                    <svg xmlns="http://www.w3.org/2000/svg" class="pf-c-background-image__filter" width="0" height="0">
                        <filter id="image_overlay">
                            <feColorMatrix type="matrix" values="1 0 0 0 0 1 0 0 0 0 1 0 0 0 0 0 0 0 1 0"></feColorMatrix>
                            <feComponentTransfer color-interpolation-filters="sRGB" result="duotone">
                                <feFuncR type="table" tableValues="0.086274509803922 0.43921568627451"></feFuncR>
                                <feFuncG type="table" tableValues="0.086274509803922 0.43921568627451"></feFuncG>
                                <feFuncB type="table" tableValues="0.086274509803922 0.43921568627451"></feFuncB>
                                <feFuncA type="table" tableValues="0 1"></feFuncA>
                            </feComponentTransfer>
                        </filter>
                    </svg>
                </div>
            }
        }
    }
}
