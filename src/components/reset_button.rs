use crate::components::game::utils::state::*;
use crate::components::styles::*;
use leptos::*;

#[component]
pub fn ResetButton(cx: Scope) -> impl IntoView {
    let reset = create_action(cx, move |_: &()| send(cx, ResetState));

    view! {
        cx,
        <button
            class=button_class(ButtonStyle::Danger, "")
            on:click=move |_| reset.dispatch(())
        >
            "Reset state"
        </button>
    }
}
