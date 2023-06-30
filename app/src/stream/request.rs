use {
    super::{connection::Error, provider::use_connection},
    common::api::Request,
    leptos::{use_context, ReadSignal, RwSignal, Scope, SignalGet},
    std::{fmt::Debug, ops::Deref, rc::Rc},
    wasm_bindgen::UnwrapThrowExt,
};

#[derive(Debug, PartialEq, Eq)]
pub enum RequestState<T> {
    Pending,
    Data(T),
    Offline,
    Error,
}

impl<T> RequestState<T> {
    pub fn is_pending(&self) -> bool {
        matches!(self, Self::Pending)
    }

    /// Returns the data if available.
    pub fn data(&self) -> Option<&T> {
        match self {
            Self::Data(t) => Some(t),
            Self::Offline => None,
            Self::Error => None,
            Self::Pending => None,
        }
    }
}

/// Send a message and stream the response(s) back.
pub fn request<T>(cx: Scope, message: T) -> ReadSignal<RequestState<Rc<<T as Request>::Response>>>
where
    T: Request + 'static,
{
    let (state, set_state) = use_context::<RwSignal<RequestState<Rc<T::Response>>>>(cx)
        .unwrap_throw()
        .split();

    let connection = use_connection(cx);

    leptos::spawn_local(async move {
        match connection().borrow_mut().send(message.into()).await {
            Ok(()) => (),
            Err(Error::Queued) => set_state(RequestState::Offline),
            Err(Error::SendFailed) => set_state(RequestState::Error),
        }
    });

    state
}

#[derive(Clone)]
pub struct RequestHandle<T>
where
    T: Request + PartialEq + Eq + 'static,
{
    state: ReadSignal<RequestState<Rc<<T as Request>::Response>>>,
    send: Rc<dyn Fn(T)>,
}

impl<T> RequestHandle<T>
where
    T: Request + PartialEq + Eq + 'static,
{
    /// Send the message.
    pub fn send(&self, message: T) {
        (self.send)(message);
    }
}

impl<T> Deref for RequestHandle<T>
where
    T: Request + PartialEq + Eq + 'static,
{
    type Target = ReadSignal<RequestState<Rc<<T as Request>::Response>>>;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<T> PartialEq for RequestHandle<T>
where
    T: Request + PartialEq + Eq + 'static,
{
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

/// Send a message and stream the responses back.
/// Send the message with `.send(message)`
#[cfg(not(feature = "ssr"))]
pub fn request_deferred<T>(cx: Scope) -> RequestHandle<T>
where
    T: Request + PartialEq + Eq + 'static,
{
    let (state, set_state) = use_context::<RwSignal<RequestState<Rc<T::Response>>>>(cx)
        .unwrap_throw()
        .split();

    // send the message
    let send = Rc::new(move |message: T| {
        log::debug!("calling request_deferred from frontend");
        leptos::spawn_local(async move {
            match use_connection(cx).get().borrow().send(message.into()).await {
                Ok(()) => (),
                Err(Error::Queued) => set_state(RequestState::Offline),
                Err(Error::SendFailed) => set_state(RequestState::Error),
            }
        });
    });

    RequestHandle { state, send }
}

/// Send a message and stream the responses back.
/// Send the message with `.send(message)`
#[cfg(feature = "ssr")]
pub fn request_deferred<T>(cx: Scope) -> RequestHandle<T>
where
    T: Request + PartialEq + Eq + 'static,
{
    // when send is called, we need to just process the message

    let (state, set_state) = use_context::<RwSignal<RequestState<Rc<T::Response>>>>(cx)
        .unwrap_throw()
        .split();

    // send the message
    // let send = Rc::new(move |message: T| {
    //     log::debug!("calling spawn_local");
    //     leptos::spawn_local(async move {
    //         match use_connection(cx).get().borrow().send(message.into()).await {
    //             Ok(()) => (),
    //             Err(Error::Queued) => set_state(RequestState::Offline),
    //             Err(Error::SendFailed) => set_state(RequestState::Error),
    //         }
    //     });
    // });

    let send = Rc::new(|_| {
        log::debug!("calling request_deferred from backend");
    });

    RequestHandle { state, send }
}
