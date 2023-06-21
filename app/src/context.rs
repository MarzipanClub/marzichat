use {
    crate::pages::Signup, common::internationalization::Language, const_format::formatcp,
    leptos::*, leptos_meta::*, leptos_router::*,
};

struct User;

#[cfg(feature = "ssr")]
fn get_user() -> User {
    User
}

#[cfg(not(feature = "ssr"))]
fn get_user() -> User {
    User
}
