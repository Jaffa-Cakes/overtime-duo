use super::*;

#[component]
pub fn Erin() -> impl IntoView {
    view! {
        <div class="flex flex-row">
            <img class="w-3/12 h-auto" src="/images/erin.jpg" />

            <div class="w-9/12">
                <h1 class="text-1xl mb-6">"Erin"</h1>

                <p class="mb-2">"Undisputedly one of the border's most talented female vocalists, Erin's voice has been likened to such vocalists as Karen Capenter, Suzie Quatro, Melissa Etheridge, K D Lang and Alannah Miles. This illustrates the wide range of music Erin covers, and, combined with her guitar playing and crowd-pleasing personality, ensures a successful function."</p>
                <p class="mb-2 font-bold">"Erin is a unique entertainer!"</p>
            </div>
        </div>
    }
}
