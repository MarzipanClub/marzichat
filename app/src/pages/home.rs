use {
    crate::stream::{request_deferred, RequestState},
    common::{api::username::CheckAvailability, types::Username, PRODUCT_NAME},
    leptos::*,
    leptos_meta::*,
    std::rc::Rc,
};

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    // let res = (Username("foobar".into()), true);
    let rws = create_rw_signal(cx, RequestState::<Rc<(Username, bool)>>::Pending);
    provide_context(cx, rws);
    // RequestState<Rc<T::Response>>
    let r = request_deferred(cx);
    r.send(CheckAvailability("foobar".into()));

    view! { cx,
        <Title text={PRODUCT_NAME}/>
        <h1>"Welcome to Marzichat"</h1>
        <h2>"Site under construction."</h2>
    }
}
