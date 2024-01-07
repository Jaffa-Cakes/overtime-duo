use super::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="bg-black text-white p-8 text-center">
            <p>"I am the footer"</p>
        </div>
    }
}
