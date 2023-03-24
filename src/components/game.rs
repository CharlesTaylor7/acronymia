mod acronym;
pub mod context;
mod judging;
pub mod player_roster;
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

    let round_counter = create_memo(cx, move |_| {
        game_state(cx).with(|g| g.round_counter.clone())
    });

    view! {
        cx,
        <div class="flex flex-col items-start mx-20 my-4 gap-4">
            <h1 class="text-xl font-bold">"Acronymia"</h1>
            {move|| round_counter.with(|c| c.as_ref().map(|c| view! {cx,
                <h2 class="text-l font-bold">
                    {c}
                </h2>
            }))}
            {move|| match game_step() {
                GameStep::Setup => view! { cx, <><GameSetup /></> },
                GameStep::Submission => view! { cx, <><GameSubmission /></> },
                GameStep::Judging => view! { cx, <><GameJudging /></> },
                GameStep::Results => view! { cx, <><GameResults /></> },
            }}
            <DebugView />
        </div>
    }
}
