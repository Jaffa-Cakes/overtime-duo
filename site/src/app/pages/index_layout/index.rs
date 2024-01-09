use super::*;

#[component]
pub fn Index() -> impl IntoView {
    view! {
        <components::Intro />
        <components::DemoSongs />
    }
}
