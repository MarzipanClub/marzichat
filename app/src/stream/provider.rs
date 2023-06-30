//! The websocket context.

use {
    super::backoff::Backoff,
    crate::stream::connection::Connection,
    gloo::timers::callback::Timeout,
    leptos::{create_effect, create_trigger, provide_context, use_context, ReadSignal, Scope},
    std::{cell::RefCell, rc::Rc},
    wasm_bindgen::UnwrapThrowExt,
};

/// Provides a websocket connection to the scope.
pub fn provide_connection(cx: Scope) {
    // This hook alerts that the webapp should reconnect with
    // backoff.
    let reconnect = create_trigger(cx);

    // This hook alerts that the webapp should reconnect now because of some
    // user interaction.
    let reconnect_now = create_trigger(cx);

    // This toggle hook is used to know when server responded with a pong meaning
    // the websocket is ready.
    let is_ready = create_trigger(cx);

    let backoff = Rc::new(RefCell::new(Backoff::new()));
    let connection = Rc::new(RefCell::new(Connection::uninitialized()));

    // reset the backoff when the is_ready signal is notified
    {
        let backoff = backoff.clone();
        create_effect(cx, move |_| {
            is_ready();
            backoff.borrow_mut().reset();
        });
    }

    {
        let connection = connection.clone();
        create_effect(cx, move |_| {
            reconnect();

            //  Reconnect after backoff time has elapsed
            let timeout = {
                let connection = connection.clone();
                Timeout::new(backoff.borrow().as_millis() as _, move || {
                    let queued_messages = connection.borrow().get_queued();
                    // create a new connection with the last queued messages
                    *connection.borrow_mut() =
                        Connection::new(reconnect, reconnect_now, is_ready, queued_messages);
                })
            };
            backoff.borrow_mut().increase();

            // we don't plan on cancelling the timeout so we can `forget`.
            timeout.forget();
        });
    }

    provide_context(cx, connection);
}

/// Obtains the websocket connection from the scope.
/// ### Panicking Behavior
/// Panics if the connection was not provided by the `provide_connection` higher
/// in the component tree.
pub fn use_connection(cx: Scope) -> ReadSignal<Rc<RefCell<Connection>>> {
    use_context(cx).unwrap_throw()
}
