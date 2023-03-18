use crate::components::game::utils::state::*;
use crate::components::styles::*;
use crate::types::ClientMessage::*;
use leptos::*;

#[component]
pub fn ResetButton(cx: Scope) -> impl IntoView {
    let reset = create_action(cx, move |_: &()| send(cx, ResetState));

    view! {
        cx,
        <button
            class=button_class("bg-blue-300")
            on:click=move |_| reset.dispatch(())
        >
            "Reset state"
        </button>
    }
}
