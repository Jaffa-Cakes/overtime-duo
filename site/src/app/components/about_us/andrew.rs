use super::*;

#[component]
pub fn Andrew() -> impl IntoView {
    view! {
        <div class="flex flex-row">
            <div class="w-9/12">
                <h1 class="text-1xl mb-6">"Andrew"</h1>

                <p class="mb-2">"The backbone of the Overtime Duo, Andrew is a talented lead and rhythm guitarist, and has achieved certificates in classical and modern guitar. Andrew is also responsible for all the behind-the-scenes sequencing, for example, drums, keyboard and bass, as well as other instruments. Andrew is currently learning the banjo, and is hoping to master the instrument and add it to the show."</p>
                <p class="mb-2 font-bold">"Andrew is conscientious and professional!"</p>
            </div>

            <img class="w-3/12 h-auto" src="/images/andrew.jpg" />
        </div>
    }
}
