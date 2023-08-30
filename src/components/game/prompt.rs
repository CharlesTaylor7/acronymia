use crate::components::game::context::*;
use crate::components::state::*;
use ::leptos::*;
use core::iter::once;

/// Shows prompt with acronym.
/// Acronym is stylized by:
/// - bold
/// - green
/// - capitalized letters
/// - insert periods
#[component]
pub fn Prompt() -> impl IntoView {
    let game_state = use_typed_context::<Signal_GameState>();
    view! {
        <p class="max-w-[205px]">
            <span>
                {game_state.with(|g| g.prompt.before.clone())}
            </span>
            <span class="inline font-bold text-emerald-600">
            {
                game_state.with(|g| g.prompt.acronym
                    .chars()
                    .flat_map(|c| c.to_uppercase().chain(once('.')))
                    .collect::<String>()
                )
            }
            </span>
            <span>
                {game_state.with(|g| g.prompt.after.clone())}
            </span>
        </p>
    }
}
