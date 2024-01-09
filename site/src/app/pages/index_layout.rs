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
        <div class="min-h-screen flex flex-col justify-between bg-slate-800">
            <components::Navbar />

            <div class="flex flex-row justify-center">
                <div class="max-w-6xl w-full">
                    <Outlet />
                </div>
            </div>

            <components::Footer />
        </div>
    }
}
