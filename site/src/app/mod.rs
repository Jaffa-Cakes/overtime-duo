#[allow(unused_imports)]
use super::*;

use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod api;
mod components;
mod pages;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/site.css"/>

        <Title text="Welcome to Leptos!!!"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <Routes>
                <Route path="/" view=pages::IndexLayout>
                    <Route path="/about-us" view=pages::index_layout::AboutUs />
                    <Route path="/contact-us" view=pages::index_layout::ContactUs />
                    <Route path="/gig-guide" view=pages::index_layout::GigGuide />
                    <Route path="/venues" view=pages::index_layout::Venues />
                    <Route path="" view=pages::index_layout::Index />
                </Route>
            </Routes>
        </Router>
    }
}
