use super::*;

#[component]
pub fn Item(#[prop(into)] name: String, #[prop(into)] href: String) -> impl IntoView {
    let location = use_location();
    let (active, set_active) = create_signal(false);

    {
        let href = href.clone();

        create_effect(move |_| {
            let current_path = location.pathname.get();
            set_active.set(*current_path == href);
        });
    }

    let styles = move || {
        if active.get() {
            "nav-item bg-slate-700"
        } else {
            "nav-item bg-slate-600 hover:bg-slate-700"
        }
    };

    view! {
        <A href={href} class={styles}>
            {name}
        </A>
    }
}
