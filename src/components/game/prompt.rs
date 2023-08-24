use crate::components::game::utils::state::*;
use ::leptos::*;

/// Shows prompt with acronym.
/// Acronym is stylized by:
/// - bold
/// - green
/// - capitalized letters
/// - insert periods
#[component]
pub fn Prompt(cx: Scope) -> impl IntoView {
    use core::iter::once;
    view! { cx,
        <p class="max-w-[205px]">
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
        </p>
    }
}