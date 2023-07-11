use {
    crate::{internationalization::Translations, on_click_outside, routes::*, PRODUCT_NAME},
    leptos::*,
    leptos_router::*,
};

#[component]
pub fn Nav(cx: Scope) -> impl IntoView {
    let t = Translations::default();
    let (show_modal, set_show_modal) = create_signal(cx, false);
    let modal_ref = create_node_ref::<html::Div>(cx);

    on_click_outside(cx, modal_ref, move |_| set_show_modal.set(false));

    let account_button =
        move || view! { cx, <div class="p-2 d-inline" aria-haspopup="true">{t.account()}</div> };

    view! { cx,
        <nav class="Header">
            <div class="Header-Container">
                <div class="Header-item Header-item--full">
                    <A href=HOME class="Header-link f3">{PRODUCT_NAME}</A>
                </div>
                <div class="Header-item mr-0">
                    <div style="cursor: pointer;" class="dropdown details-reset details-overlay d-inline-block" on:click=move |_| set_show_modal.set(true)>
                        <Show when=move || show_modal.get() fallback=move |_| account_button() >
                            {account_button()}
                            <div class="SelectMenu right-0" on:click=move |_| set_show_modal.set(false)>
                                <div node_ref=modal_ref class="SelectMenu-modal">
                                    <header class="SelectMenu-header">
                                        <h3 class="SelectMenu-title color-fg-default">{t.not_signed_in()}</h3>
                                    </header>
                                    <div class="SelectMenu-list">
                                        <A href=SIGNUP class="SelectMenu-item d-block">
                                            <h5>{t.create_a_free_account()}</h5>
                                            <span>{t.join_the_discussion_by_signing_up()}</span>
                                        </A>
                                        <A href=SIGNIN class="SelectMenu-item">
                                            <svg aria-hidden="true" height="16" viewBox="0 0 16 16" version="1.1" width="16" data-view-component="true" class="octicon octicon-person">
                                                <path
                                                    d="M10.561 8.073a6.005 6.005 0 0 1 3.432 5.142.75.75 0 1 1-1.498.07 4.5 4.5 0 0 0-8.99 0 .75.75 0 0 1-1.498-.07 6.004 6.004 0 0 1 3.431-5.142 3.999 3.999 0 1 1 5.123 0ZM10.5 5a2.5 2.5 0 1 0-5 0 2.5 2.5 0 0 0 5 0Z"
                                                ></path>
                                            </svg>
                                            <span class="text-semibold ml-1">{t.sign_in()}</span>
                                        </A>
                                        <hr class="SelectMenu-divider" />
                                        <A href=ABOUT class="SelectMenu-item color-fg-muted">{t.about_marzichat()}</A>
                                        <A href=HELP_AND_SAFETY class="SelectMenu-item color-fg-muted">{t.help_and_safety()}</A>
                                    </div>
                                </div>
                            </div>
                        </Show>
                    </div>
                </div>
            </div>
        </nav>
    }
}
