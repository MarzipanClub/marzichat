use {
    crate::{internationalization::Translations, routes::HOME},
    leptos::*,
    leptos_router::*,
};

#[component]
pub fn NotFound(cx: Scope) -> impl IntoView {
    let t = Translations::default();
    view! { cx,
        <div class="mx-auto my-auto">
            <div class="blankslate blankslate-narrow-blued">
                // there's a bug in leptos where going back to a page changes the viewBox in an svg.
                // <svg class="octicon octicon-octoface blankslate-icon" aria-hidden="true" viewBox="0 0 24 24" width="24" height="24" clip-rule="evenodd" fill-rule="evenodd" height="512" stroke-linejoin="round" stroke-miterlimit="2" viewBox="0 0 32 32" width="512" xmlns="http://www.w3.org/2000/svg"><g id="OUTLINE"><path d="m31 6.2c0-.583-.232-1.143-.644-1.556-.413-.412-.973-.644-1.556-.644-4.591 0-21.009 0-25.6 0-.583 0-1.143.232-1.556.644-.412.413-.644.973-.644 1.556v19.6c0 .583.232 1.143.644 1.556.413.412.973.644 1.556.644h25.6c.583 0 1.143-.232 1.556-.644.412-.413.644-.973.644-1.556zm-28 3.8v15.8c0 .053.021.104.059.141.037.038.088.059.141.059h25.6c.053 0 .104-.021.141-.059.038-.037.059-.088.059-.141v-14.8h-6.717c-.341 0-.678-.08-.984-.232l-1.493-.747c-.028-.014-.058-.021-.089-.021zm26-1v-2.8c0-.053-.021-.104-.059-.141-.037-.038-.088-.059-.141-.059h-25.6c-.053 0-.104.021-.141.059-.038.037-.059.088-.059.141v1.8h16.717c.341 0 .678.08.984.232l1.493.747c.028.014.058.021.089.021z"/><path d="m9.086 16.5-.793.793c-.39.39-.39 1.024 0 1.414s1.024.39 1.414 0l.793-.793.793.793c.39.39 1.024.39 1.414 0s.39-1.024 0-1.414l-.793-.793.793-.793c.39-.39.39-1.024 0-1.414s-1.024-.39-1.414 0l-.793.793-.793-.793c-.39-.39-1.024-.39-1.414 0s-.39 1.024 0 1.414z"/><path d="m20.086 16.5-.793.793c-.39.39-.39 1.024 0 1.414s1.024.39 1.414 0l.793-.793.793.793c.39.39 1.024.39 1.414 0s.39-1.024 0-1.414l-.793-.793.793-.793c.39-.39.39-1.024 0-1.414s-1.024-.39-1.414 0l-.793.793-.793-.793c-.39-.39-1.024-.39-1.414 0s-.39 1.024 0 1.414z"/><g transform="matrix(1 0 0 -1 0 44)"><path d="m11.553 21.106s.871-.436 1.463-.732c.619-.31 1.349-.31 1.968 0l.927.463c.056.028.122.028.178 0l.927-.463c.619-.31 1.349-.31 1.968 0 .592.296 1.463.732 1.463.732.494.246.694.848.447 1.341-.246.494-.848.694-1.341.447l-1.464-.731c-.056-.028-.122-.028-.178 0l-.927.463c-.619.31-1.349.31-1.968 0l-.927-.463c-.056-.028-.122-.028-.178 0l-1.464.731c-.493.247-1.095.047-1.341-.447-.247-.493-.047-1.095.447-1.341z"/></g></g></svg>
                <h3 class="blankslate-heading">{t.not_found()}</h3>
                <p>{t.page_not_found_desc()}</p>
                <div class="blankslate-action">
                    <A href=HOME class="btn btn-primary">{t.home()}</A>
                </div>
            </div>
        </div>
    }
}
