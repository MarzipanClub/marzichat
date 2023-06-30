use {
    common::{internationalization::Translations, PRODUCT_NAME},
    leptos::*,
    leptos_meta::*,
};

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let t = Translations::for_language(Default::default());
    view! { cx,
        <Title text={PRODUCT_NAME}/>
        <h1>"Welcome to Marzichat"</h1>
        <h2>"Site under construction."</h2>
    }
}
