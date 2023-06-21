use {common::internationalization::Translations, leptos::*, leptos_meta::*};

#[component]
pub fn Signup(cx: Scope) -> impl IntoView {
    let t = Translations::for_language(Default::default());
    view! { cx,
        <Title text={t.signup()}/>
        <div class="main navbar-space">
            <div class="dialog">
                <div class="dialog-header">
                    <div class="dialog-header-title is-not-selectable">{t.create_an_account()}</div>
                </div>
                <form class="dialog-body">
                    <div class="dialog-image is-not-selectable" >
                        <img src="/assets/images/logo.svg" alt=t.logo_of_the_letter_m()/>
                    </div>
                </form>
            </div>
        </div>
    }
}
