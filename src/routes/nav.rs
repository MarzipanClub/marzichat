use {
    crate::{internationalization::Translations, Routes},
    leptos::{component, view, IntoView, Scope},
    leptos_router::*,
};

#[component]
pub fn Nav(cx: Scope) -> impl IntoView {
    let t = Translations::default();
    view! { cx,
         <nav class="UnderlineNav" aria-label="nav bar">
            <div class="UnderlineNav-body">
                <a class="UnderlineNav-item app-link" href="#home" aria-current="page">{"Home"}</a>
                <a class="UnderlineNav-item app-link" href="#about">{"About"}</a>
                <a class="UnderlineNav-item app-link" href="#newsletter">{"Newsletter"}</a>
            </div>
            <div class="UnderlineNav-actions">
                <A href=Routes::Signin class="btn btn-sm mx-2">{t.sign_in()}</A>
                <A href=Routes::Signup class="btn btn-sm btn-primary">{t.sign_up()}</A>
            </div>
        </nav>
    }
}
