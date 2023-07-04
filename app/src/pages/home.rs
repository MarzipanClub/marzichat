use {
    crate::stream::{provider::use_connection, request_deferred, RequestState},
    common::{api::username::CheckAvailability, types::Username, PRODUCT_NAME},
    leptos::*,
    leptos_meta::*,
    std::{cell::RefCell, rc::Rc},
    wasm_bindgen::UnwrapThrowExt,
};

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let rws = create_rw_signal(cx, RequestState::<Rc<(Username, bool)>>::Pending);
    provide_context(cx, rws);

    // let r = request_deferred(cx);
    // r.send(CheckAvailability("foobar".into()));

    let (state, set_state) = use_context::<RwSignal<RequestState<Rc<(Username, bool)>>>>(cx)
        .unwrap_throw()
        .split();

    let message = CheckAvailability("foobar".into());

    // leptos::spawn_local(async move {
    //     // use_connection(cx).send(message.into()).await;
    //     // match use_connection(cx).borrow().send(message.into()).await {
    //     //     Ok(()) => (),
    //     //     Err(crate::stream::connection::Error::Queued) =>
    //     // set_state(RequestState::Offline),
    //     //     Err(crate::stream::connection::Error::SendFailed) =>
    //     // set_state(RequestState::Error), }
    // });

    #[cfg(feature = "hydrate")]
    let ws = use_context::<web_sys::WebSocket>(cx).unwrap_throw();

    #[cfg(feature = "hydrate")]
    create_effect(cx, move |_| {
        leptos::spawn_local(async move {
            leptos::log!("inside spawn local");
        });
        use {
            js_sys::ArrayBuffer,
            leptos::{create_effect, use_context, SignalUpdate},
            wasm_bindgen::{prelude::Closure, JsCast},
        };

        let callback = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
            let blob = event.data().dyn_into::<ArrayBuffer>().unwrap();

            // if let Ok(update_signal) =
            // serde_json::from_str::<ServerSignalUpdate>(&ws_string) {
            //     if update_signal.name == name {
            //         json_set.update(|doc| {
            //             json_patch::patch(doc,
            // &update_signal.patch).unwrap();         });
            //         let new_value =
            // serde_json::from_value(json_get()).unwrap();
            //         set(new_value);
            //     }
            // }
        }) as Box<dyn FnMut(_)>);
        let function: &js_sys::Function = callback.as_ref().unchecked_ref();
        ws.set_onmessage(Some(function));

        // Keep the closure alive for the lifetime of the program
        callback.forget();
    });

    let is_available = rws.get();
    view! { cx,
        <Title text={PRODUCT_NAME}/>
        {format!("is avail 2: {is_available:?}")}
        <h1>Welcome to Marzichat</h1>
        <h2>Site under construction.</h2>
    }
}
