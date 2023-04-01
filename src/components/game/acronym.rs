use crate::components::game::utils::state::*;
use ::leptos::*;

/// Applies bold, capitalizes letters, inserts periods.
#[component]
pub fn Acronym(cx: Scope) -> impl IntoView {
    use core::iter::once;
    view! { cx,
        <span>
            {game_state(cx).with(|g| g.prompt.before.clone())}
        </span>
        <span class="inline font-bold text-emerald-600">
        {
            game_state(cx).with(|g| g.prompt.acronym
                .chars()
                .flat_map(|c| c.to_uppercase().chain(once('.')))
                .collect::<String>()
                )
        }
        </span>
        <span>
            {game_state(cx).with(|g| g.prompt.after.clone())}
        </span>
    }
}
