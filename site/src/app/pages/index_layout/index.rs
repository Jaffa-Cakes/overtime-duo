use super::*;

#[component]
pub fn Index() -> impl IntoView {
    view! {
        <div class="flex flex-row justify-center">
            <div class="max-w-6xl w-full">
                <Block />
                <Block />
                <Block />
                <Block />
                <Block />
                <Block />
                <Block />
            </div>
        </div>
    }
}

#[component]
pub fn Block() -> impl IntoView {
    view! {
        <div class="rounded-lg p-4 bg-black text-white my-8">
            <div class="flex flex-row">
                <div class="w-7/12 pr-2">
                    <img src="/images/erin-andrew-stage.jpg" class="rounded-lg w-full" />
                </div>

                <div class="w-5/12 pl-2">
                    <h1 class="text-2xl mb-6">"Hi And Welcome To Overtime Duo"</h1>

                    <p class="mb-2">"Andrew and I have been entertaining up and down the Murray River and beyond for a number of years now."</p>
                    <p class="mb-2">"Based in the historic town of Chiltern, we have performed at clubs, hotels, weddings, parties and corporate functions in and around this area."</p>
                    <p class="mb-2">"Over the years we have established a very strong client and fan base. Give us a call if you are looking for entertainment at your venue or function."</p>
                    <p class="mb-2">"It’s been quite a ride. Hope you enjoy looking around."</p>
                    <p class="mb-2">"Come and see us at a gig some time."</p>
                    <p class="mb-2">"Cheers,"</p>
                    <p class="mb-2">"Erin and Andrew"</p>
                </div>
            </div>
        </div>
    }
}
