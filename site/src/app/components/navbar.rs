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
        <div class="sticky top-0 bg-slate-600 flex flex-row justify-center">
            {items.into_iter()
                .map(|(name, href)| view! { <Item name={name} href={href} /> })
                .collect::<Vec<_>>()}
        </div>
    }
}
