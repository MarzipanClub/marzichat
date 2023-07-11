use {
    crate::{routes::*, scroll_to_top},
    leptos::*,
    leptos_router::*,
};

#[component]
pub fn Signup(cx: Scope) -> impl IntoView {
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
                                <label for="example-text">{"Email"}</label>
                            </div>
                            <div class="form-group-body">
                                <input class="form-control width-full" type="text" value="Example Value"
                                    id="example-text" />
                            </div>
                        </div>
                        <div class="flash flash-success">Flash error inside a Box.</div>
                        <div class="form-group">
                            <div class="form-group-header">
                                <label for="example-text">{"Username"}</label>
                            </div>
                            <div class="form-group-body">
                                <input class="form-control  width-full" type="text" value="Example Value" id="example-text" />
                            </div>
                        </div>
                        <div class="form-group">
                            <div class="form-group-header">
                                <label for="example-text">{"Password"}</label>
                            </div>
                            <div class="form-group-body">
                                <input class="form-control  width-full" type="text" value="Example Value" id="example-text" />
                            </div>
                        </div>
                    </form>
                    <p class="color-fg-default">{"By continuing, you agree to the Marzichat "}
                        <A href=TERMS_AND_CONDITIONS>{"Terms and Conditions"}</A>
                        {" and "}
                        <A href=PRIVACY_POLICY>{"Privacy Policy"}</A>
                        {"."}
                    </p>
                    <div class=" text-right mt-4">
                        <button class="btn btn-primary">{"Create free account"}</button>
                    </div>
                </div>
            </div>
            <div class="Box mt-4">
                <div class="Box-body text-center">
                    <div class="blankslate color-fg-default">
                        {"Already have an account? "}<A href=SIGNIN>{"Sign in."}</A>
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
