use crate::components::game::context::*;
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
    let prompt = create_memo(move |_| game_state.with(|g| g.prompt.clone()));
    view! {
        <p class="max-w-[205px]">
            <span>
                {move || prompt.with(|p| p.before.clone())}
            </span>
            <span class="inline font-bold text-emerald-600">
                {move ||
                    prompt.with(|p|
                        p
                        .acronym
                        .chars()
                        .flat_map(|c| c.to_uppercase().chain(once('.')))
                        .collect::<String>()
                    )
                }
            </span>
            <span>
                {move || prompt.with(|p| p.after.clone())}
            </span>
        </p>
    }
}
