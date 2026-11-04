//! Run with `trunk serve examples/alerts.html`.
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_nested_router::{components::Link, prelude::*};

#[derive(Clone, Debug, PartialEq, Eq, Target)]
enum Route {
    #[target(index)]
    Home,
    Status,
}

#[function_component(Showcase)]
fn showcase() -> Html {
    let count = use_state_eq(|| 0);
    let refresh = use_callback(count.clone(), |_: (), count| count.set(**count + 1));
    html! {
        <Router<Route>>
            <main style="padding: 2rem; display: grid; gap: 1rem">
                <Title level={Level::H1}>{"Alerts and banners"}</Title>
                <Button label="Update status" onclick={refresh.clone().reform(|_| ())} />
                <Alert r#type={AlertType::Info} inline=true live_region=true
                    title={html!(<>{"Processed "}<strong>{*count}</strong>{" items"}</>)}
                    title_level={Level::H2} onclose={Callback::noop()}
                    actions={vec![Action::new("Refresh", refresh)]}
                    action_links={html! {
                        <>
                            <AlertActionLink>
                                <Link<Route> to={Route::Status} class="pf-v6-c-button pf-m-link pf-m-inline">
                                    {"View processing status"}
                                </Link<Route>>
                            </AlertActionLink>
                            <AlertActionLink href="https://www.patternfly.org/components/alert/">{"Documentation"}</AlertActionLink>
                        </>
                    }}>
                    {"Updates to this alert are announced politely."}
                </Alert>
                <Alert inline=true plain=true title="Plain inline alert" />
                <Alert title="Expandable alert" expandable=true toggle_aria_label="Show processing details"
                    variant_label="Information" custom_icon={html!({Icon::InfoCircle})}>
                    {"More details appear when expanded."}
                </Alert>
                <Alert title="A long status message that can span several lines before it is truncated. Resize the window to try it."
                    truncate_title=2 tooltip_position={TooltipPosition::Bottom} />
                <Alert title={String::from("Legacy String title and single-line truncation")} truncate=true />
                <AlertGroup live_region=true>
                    <Alert title={format!("Update {}", *count)} />
                </AlertGroup>
                {for [BannerColor::Red, BannerColor::OrangeRed, BannerColor::Orange, BannerColor::Yellow,
                    BannerColor::Green, BannerColor::Teal, BannerColor::Blue, BannerColor::Purple].into_iter().map(|color| html! {
                    <Banner {color}>{format!("{color:?} banner")}</Banner>
                })}
                {for [BannerStatus::Success, BannerStatus::Warning, BannerStatus::Danger,
                    BannerStatus::Info, BannerStatus::Custom].into_iter().map(|status| html! {
                    <Banner {status} screen_reader_text={format!("{status:?}: ")}>
                        <Flex modifiers={[FlexModifier::Justify(Justify::SpaceBetween)]}>
                            <FlexItem>{Icon::Bell}{" Left content"}</FlexItem>
                            <FlexItem>{"Center content"}</FlexItem>
                            <FlexItem><a href="/status">{"Right link"}</a></FlexItem>
                        </Flex>
                    </Banner>
                })}
                <Banner color={BannerColor::Purple} sticky=true>{"Sticky environment banner"}</Banner>
                <Banner status={BannerStatus::Success} screen_reader_text="Success: " pill=true>
                    {Icon::CheckCircle}{" All items processed"}
                </Banner>
            </main>
        </Router<Route>>
    }
}

fn main() {
    yew::Renderer::<Showcase>::new().render();
}
