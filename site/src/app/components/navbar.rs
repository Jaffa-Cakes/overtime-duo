use super::*;

mod item;

use item::Item;

#[component]
pub fn Navbar() -> impl IntoView {
    let items = vec![
        ("Home", "/"),
        ("About Us", "/about-us"),
        ("Gig Guide", "/gig-guide"),
        ("Venues", "/venues"),
        ("Contact Us", "/contact-us"),
    ];

    view! {
        <div class="sticky top-0 bg-slate-700 flex flex-row justify-center">
            <div class="flex flex-row justify-between place-content-center justify-items-center place-items-center max-w-6xl w-full">
                <A href="/">
                    <img src="/images/logo.png" class="w-auto h-12" />
                </A>

                <div class="flex flex-row justify-center">
                    {items.into_iter()
                        .map(|(name, href)| view! { <Item name={name} href={href} /> })
                        .collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
