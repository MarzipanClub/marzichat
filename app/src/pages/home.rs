use {
    crate::websocket::request,
    common::{
        api::{username, AppMessage},
        PRODUCT_NAME,
    },
    leptos::*,
    leptos_meta::*,
};

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let user = create_resource(
        cx,
        move || {},
        move |id| async move {
            request(cx, username::SuggestUsername);
        },
    );

    view! { cx,
        <Title text={PRODUCT_NAME}/>
        <Suspense fallback=|| view! { cx, "Loading..." }>
            {user.read(cx)}
        </Suspense>
        <h1>"Welcome to Marzichat"</h1>
        <h2>"Site under construction."</h2>
    }
}
