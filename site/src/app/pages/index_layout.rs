use super::*;

pub mod about_us;
pub mod contact_us;
pub mod gig_guide;
pub mod index;
pub mod venues;

pub use about_us::AboutUs;
pub use contact_us::ContactUs;
pub use gig_guide::GigGuide;
pub use index::Index;
pub use venues::Venues;

#[component]
pub fn IndexLayout() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            <components::Navbar />

            <Outlet />

            <components::Footer />
        </div>
    }
}
