use {
    crate::{
        internationalization::Translations,
        routes::*,
        scroll_to_top,
        types::{email::Email, password::Password, validation::Validate, Username},
    },
    leptos::*,
    leptos_router::*,
    leptos_use::use_throttle_fn,
};

#[component]
pub fn Signup(cx: Scope) -> impl IntoView {
    let t = Translations::default();
    let (email, set_email) = create_signal(cx, String::default());
    let (username, set_username) = create_signal(cx, String::default());
    let (password, set_password) = create_signal(cx, String::default());
    let (password_again, set_password_again) = create_signal(cx, String::default());

    let (show_email_error, set_show_email_error) = create_signal(cx, false);

    let check_username_availability = use_throttle_fn(
        move || {
            let username: Username = username().into();
            if username.validate().is_err() {
                leptos::log!("username is invalid!");
            }
        },
        2000.0,
    );

    let on_click = move |_| {
        let email: Email = email().into();
        if email.validate().is_err() {
            set_show_email_error(true);
        }
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
                        <div class="form-group">
                            <div class="form-group-header">
                                <label for="email">{t.email()}</label>
                            </div>
                            <div class="form-group-body">
                                <input class="form-control width-full" type="text" id="email"
                                    on:input=move |ev| {  set_email(event_target_value(&ev)); set_show_email_error(false); }
                                    prop:value=email
                                />
                            </div>
                        </div>
                        <Show when=move || show_email_error() fallback=|_| ()>
                            <div class="flash flash-error">
                                <Show when=move || !email().is_empty() fallback=|_| view! { cx, "Please enter an email." } >
                                    <span class="text-bold">{email()}</span>
                                    " seems to be invalid or a temporary email. Please fix it or use another email."
                                </Show>
                            </div>
                        </Show>
                        <div class="form-group">
                            <div class="form-group-header">
                                <label for="username">{t.username()}</label>
                            </div>
                            <div class="form-group-body">
                                <input class="form-control width-full" type="text" id="username"
                                    on:input=move |ev| { set_username(event_target_value(&ev)); check_username_availability(); }
                                    prop:value=username
                                />
                            </div>
                        </div>
                        <div class="form-group">
                            <div class="form-group-header">
                                <label for="password">{t.password()}</label>
                            </div>
                            <div class="form-group-body">
                                <input class="form-control width-full" type="text" id="password"
                                    on:input=move |ev| set_password(event_target_value(&ev))
                                    prop:value=password
                                />
                            </div>
                        </div>
                        <div class="form-group">
                            <div class="form-group-header">
                                <label for="password_again">{t.retype_password()}</label>
                            </div>
                            <div class="form-group-body">
                                <input class="form-control width-full" type="text" id="password_again"
                                    on:input=move |ev| set_password_again(event_target_value(&ev))
                                    prop:value=password_again
                                />
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
                        <button class="btn btn-primary" on:click=on_click>{t.create_free_account()}</button>
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
