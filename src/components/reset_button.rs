use crate::components::game::utils::state::*;
use crate::components::styles::*;
use leptos::*;

#[component]
pub fn ResetButton() -> impl IntoView {
    let reset = create_action(move |_: &()| send(ResetState));

    view! {

        <button
            class=button_class(ButtonStyle::Danger, "")
            on:click=move|_| reset.dispatch(())
        >
            "Reset state"
        </button>
    }
}
