use {common::internationalization::Translations, leptos::*, leptos_meta::*};

#[component]
pub fn Signup(cx: Scope) -> impl IntoView {
    let t = Translations::for_language(Default::default());

    view! { cx,
        <Title text={t.signup()}/>
        <h1>{"Under construction"}</h1>
    }
}
