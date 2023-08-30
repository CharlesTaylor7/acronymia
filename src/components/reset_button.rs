use crate::components::game::utils::state::*;
use crate::components::styles::*;
use leptos::*;

#[component]
pub fn ResetButton() -> impl IntoView {
    let action = create_ws_action();
    view! {
        <button
            class=button_class(ButtonStyle::Danger, "")
            on:click=move|_| action.dispatch(ResetState)
        >
            "Reset state"
        </button>
    }
}
