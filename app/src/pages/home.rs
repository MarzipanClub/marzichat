use {
    crate::websocket::request,
    common::{api::AppMessage, PRODUCT_NAME},
    leptos::*,
    leptos_meta::*,
};

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    request(cx, AppMessage::SuggestUsername);
    view! { cx,
        <Title text={PRODUCT_NAME}/>


        <h1>"Welcome to Marzichat"</h1>
        <h2>"Site under construction."</h2>
    }
}
