use super::*;

#[component]
pub fn SongBar(#[prop(into)] _href: String) -> impl IntoView {
    view! {
        <div class="flex flex-col justify-center place-items-center place-content-center justify-items-center h-8 flex-grow">
            <div class="p-1 bg-blue-900 w-full rounded">
                <div class="h-2 bg-blue-700 rounded" />
            </div>
        </div>
    }
}
