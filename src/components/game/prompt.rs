use crate::components::game::utils::state::*;
use ::leptos::*;

/// Shows prompt with acronym.
/// Acronym is stylized by:
/// - bold
/// - green
/// - capitalized letters
/// - insert periods
#[component]
pub fn Prompt() -> impl IntoView {
    use core::iter::once;
    view! {
        <p class="max-w-[205px]">
            <span>
                {expect_context().with(|g| g.prompt.before.clone())}
            </span>
            <span class="inline font-bold text-emerald-600">
            {
                expect_context().with(|g| g.prompt.acronym
                    .chars()
                    .flat_map(|c| c.to_uppercase().chain(once('.')))
                    .collect::<String>()
                    )
            }
            </span>
            <span>
                {expect_context().with(|g| g.prompt.after.clone())}
            </span>
        </p>
    }
}
