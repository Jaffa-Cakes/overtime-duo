use super::*;

#[component]
pub fn SongName(#[prop(into)] name: String) -> impl IntoView {
    view! {
        <div class="flex flex-col justify-center place-items-center place-content-center justify-items-center  h-8">
            <h4>{name}</h4>
        </div>
    }
}
