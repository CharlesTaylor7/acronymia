pub mod context;
mod judging;
pub mod player_roster;
mod prompt;
mod results;
mod setup;
mod submission;
mod timer;
pub mod utils;
use self::context::*;
use self::judging::*;
use self::results::*;
use self::setup::*;
use self::submission::*;
use self::utils::state::*;
use crate::components::debug_view::*;
use crate::types::*;
use ::leptos::*;

#[component]
pub fn Game() -> impl IntoView {
    provide_game_context();
    let game_step = create_memo(move |_| expect_context().with(|g| g.step.clone()));

    view! {
        <div class="flex flex-row justify-center m-4">
            <div class="flex flex-col items-start gap-4">
                <h1 class="text-4xl font-bold tracking-wide">
                    "Acronymia"
                </h1>
                {move|| match game_step.get() {
                    GameStep::Setup => view! { <GameSetup /> },
                    GameStep::Submission => view! { <GameSubmission /> },
                    GameStep::Judging => view! { <GameJudging />},
                    GameStep::Results => view! { <GameResults />},
                }}
                <DebugView />
            </div>
        </div>
    }
}
