pub mod context;
mod judging;
pub mod player_roster;
mod prompt;
mod results;
mod setup;
mod submission;
mod timer;
use self::context::*;
use self::judging::*;
use self::results::*;
use self::setup::*;
use self::submission::*;
use crate::components::debug_view::*;
use crate::components::state::*;
use crate::types::*;
use ::leptos::*;

#[component]
pub fn Game() -> impl IntoView {
    provide_game_context();
    let game_state = use_typed_context::<Signal_GameState>();
    let game_step = create_memo(move |_| game_state.with(|g| g.step.clone()));

    view! {
        {move|| match game_step.get() {
            GameStep::Setup => view! { <GameSetup /> },
            GameStep::Submission => view! { <GameSubmission /> },
            GameStep::Judging => view! { <GameJudging />},
            GameStep::Results => view! { <GameResults />},
        }}
        <DebugView />
    }
}
