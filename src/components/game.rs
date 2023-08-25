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
pub fn Game(cx: Scope) -> impl IntoView {
    provide_game_context(cx);
    let game_step = create_memo(cx, move |_| game_state(cx).with(|g| g.step.clone()));

    view! {
        cx,
        <div class="flex flex-row justify-center m-4">
            <div class="flex flex-col items-start gap-4">
                <h1 class="text-4xl font-bold tracking-wide">
                    "Acronymia"
                </h1>
                {move|| match game_step.get() {
                    GameStep::Setup => view! { cx, <><GameSetup /></> },
                    GameStep::Submission => view! { cx, <><GameSubmission /></> },
                    GameStep::Judging => view! { cx, <><GameJudging /></> },
                    GameStep::Results => view! { cx, <><GameResults /></> },
                }}
                <DebugView />
            </div>
        </div>
    }
}
