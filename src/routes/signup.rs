use {
    crate::{internationalization::Translations, routes::*, scroll_to_top},
    leptos::*,
    leptos_router::*,
};

#[component]
pub fn Signup(cx: Scope) -> impl IntoView {
    let t = Translations::default();
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
                                <input class="form-control width-full" type="text"
                                    id="email" />
                            </div>
                        </div>
                        <div class="flash flash-success">"Flash error inside a Box."</div>
                        <div class="form-group">
                            <div class="form-group-header">
                                <label for="username">{t.username()}</label>
                            </div>
                            <div class="form-group-body">
                                <input class="form-control  width-full" type="text" id="username" />
                            </div>
                        </div>
                        <div class="form-group">
                            <div class="form-group-header">
                                <label for="password">{t.password()}</label>
                            </div>
                            <div class="form-group-body">
                                <input class="form-control  width-full" type="text" id="password" />
                            </div>
                        </div>
                        <div class="form-group">
                            <div class="form-group-header">
                                <label for="password_again">{t.retype_password()}</label>
                            </div>
                            <div class="form-group-body">
                                <input class="form-control  width-full" type="text" id="password_again" />
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
                        <button class="btn btn-primary">{"Create free account"}</button>
                    </div>
                </div>
            </div>
            <div class="Box mt-4">
                <div class="Box-body text-center">
                    <div class="blankslate color-fg-default">
                        {"Already have an account? "}<A href=SIGNIN>{"Sign in"}</A>{"."}
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
