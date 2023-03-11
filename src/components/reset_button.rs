use crate::*;
use leptos::*;

#[component]
pub fn ResetButton(cx: Scope) -> impl IntoView {
    let reset = create_action(cx, move |_: &()| {
        sse::game_state(cx).update(|g| *g = Default::default());
        api::reset_state()
    });

    view! {
        cx,
        <button
            class="border rounded p-2 bg-blue-300 border-slate-200"
            on:click=move |_| reset.dispatch(())
        >
            "Reset state"
        </button>
    }
}
