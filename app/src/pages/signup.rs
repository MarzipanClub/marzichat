use {common::internationalization::Translations, leptos::*, leptos_meta::*};

#[component]
pub fn Signup(cx: Scope) -> impl IntoView {
    let t = Translations::for_language(Default::default());

    let username_to_check = String::from("foobar");

    // let posts = create_resource(cx, || (), |_| async { list_post_metadata().await
    // }); let posts_view = move || {
    //     posts.with(cx, |posts| posts
    //         .clone()
    //         .map(|posts| {
    //             posts.iter()
    //             .map(|post| view! { cx, <li><a href=format!("/post/{}",
    // post.id)>{&post.title}</a></li>})             .collect_view(cx)
    //         })
    //     )
    // };

    view! { cx,
        <Title text={t.signup()}/>
        <h1>{"Under construction"}</h1>
    }
}
