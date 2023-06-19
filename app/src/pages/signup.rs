use {leptos::*, leptos_meta::*};

#[component]
pub fn Signup(cx: Scope) -> impl IntoView {
    // TODO: create a function to get langauge from outer scope
    // https://docs.rs/leptos/0.3.1/leptos/fn.use_context.html
    view! { cx,
        <Title text="Signup"/>
        <div class="main navbar-space">
            <div class="dialog">
                <div class="dialog-header">
                    <div class="dialog-header-title is-not-selectable">"Create an account"</div>
                </div>
                <form class="dialog-body">
                    <div class="dialog-image is-not-selectable" >
                        <img src="/assets/images/logo.svg" alt="Marzichat logo"/>
                    </div>
                </form>
            </div>
        </div>
    }
}
