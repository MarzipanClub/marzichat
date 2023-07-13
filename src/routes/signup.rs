use {
    crate::{
        internationalization::Translations,
        routes::*,
        scroll_to_top,
        types::{self, password},
    },
    leptos::*,
    leptos_router::*,
    leptos_use::use_throttle_fn,
};

#[component]
pub fn Signup(cx: Scope) -> impl IntoView {
    let t = Translations::default();

    let (username, set_username) = create_signal(cx, String::new());
    let (username_violations, set_username_violations) = create_signal(cx, Ok(()));
    let on_username_change = move |e| {
        let u = event_target_value(&e);
        set_username_violations(types::username::validate(&u));
        set_username(u);
    };

    let email_input = create_node_ref::<html::Input>(cx);
    let password_input = create_node_ref::<html::Input>(cx);
    let password_again_input = create_node_ref::<html::Input>(cx);

    let is_password_warning = || false;
    let is_password_errored = || false;
    let is_email_errored = || false;

    let check_username_availability = use_throttle_fn(
        move || {
            //
        },
        2000.0,
    );

    let submit = move |_| {
        //
    };

    scroll_to_top();
    view! { cx,
        <main class="container-sm my-4">
            <div class="Box Box--spacious">
                <div class="Box-header">
                    <h1 class="Box-title">
                        {"Create an account"}
                    </h1>
                </div>
                <div class="Box-body">
                    <form>
                        // username
                        <div class="form-group">
                            <div class="form-group-header">
                                <label for="username">{t.username()}</label>
                            </div>
                            <div class="form-group-body">
                                <input class="form-control width-full" type="text" id="username"
                                    on:input=on_username_change
                                    prop:value=username
                                />
                            </div>
                        </div>

                        // email
                        <div class="form-group" class:errored=is_email_errored >
                            <div class="form-group-header">
                                <label for="email">{t.email()}</label>
                            </div>
                            <div class="form-group-body">
                                <input class="form-control width-full" type="text" id="email" node_ref=email_input />
                                <p class="note error">
                                    // <Show when=move || !email().is_empty() fallback=move |_| t.please_enter_an_email() >
                                    //     <span class="text-bold">{email()}</span>
                                    //     {t.email_seems_invalid_description()}
                                    // </Show>
                                </p>
                            </div>
                        </div>

                        // password
                        <div class="form-group" class:warn=is_password_warning class:errored=is_password_errored >
                            <div class="form-group-header">
                                <label for="password">{t.password()}</label>
                            </div>
                            <div class="form-group-body">
                                <input class="form-control width-full" type="text" id="password" node_ref=password_input />
                                <p class="note" class:warning=is_password_warning class:error=is_password_errored>
                                    // <Show when=move || !password().is_empty() fallback=move |_| t.please_enter_a_password() >
                                    //     {"password has issues"}
                                    // </Show>
                                </p>
                            </div>
                        </div>

                        // password again
                        <div class="form-group">
                            <div class="form-group-header">
                                <label for="password_again">{t.retype_password()}</label>
                            </div>
                            <div class="form-group-body">
                                <input class="form-control width-full" type="text" id="password_again" node_ref=password_again_input />
                            </div>
                        </div>
                    </form>
                    <p class="color-fg-default">{t.terms_and_privacy_disclaimer_1()}
                        <A href=TERMS_AND_CONDITIONS>{t.terms_and_conditions()}</A>
                        {t.terms_and_privacy_disclaimer_2()}
                        <A href=PRIVACY_POLICY>{t.privacy_policy()}</A>
                        {t.terms_and_privacy_disclaimer_3()}
                    </p>
                    <div class=" text-right mt-4">
                        <button class="btn btn-primary" on:click=submit>{t.create_free_account()}</button>
                    </div>
                </div>
            </div>
            <div class="Box mt-4">
                <div class="Box-body text-center">
                    <div class="blankslate color-fg-default">
                        {t.already_have_an_account()}{" "}<A href=SIGNIN>{t.sign_in()}</A>{"."}
                    </div>
                </div>
            </div>
            <div class="Box mt-4">
                <div class="Box-body text-center color-fg-subtle">
                    {crate::copyright()}
                </div>
            </div>
        </main>
    }
}

#[server(CheckUsernameAvailability, "/api")]
pub async fn check_username_availability() -> Result<bool, ServerFnError> {
    // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    Ok(true)
}
