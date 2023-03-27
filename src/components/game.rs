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
use ::std::mem::{discriminant, Discriminant};

type D<T> = Discriminant<T>;

#[component]
pub fn Game(cx: Scope) -> impl IntoView {
    provide_game_context(cx);
    let d = create_memo(cx, move |_| game_state(cx).with(|g| discriminant(&g.step)));

    let step = move || {
        let _ = d.with(|_| ());
        game_state(cx).with_untracked(|g| g.step.clone())
    };

    view! {
        cx,
        <div class="flex flex-row justify-center mt-4">
            <div class="flex flex-col items-start gap-4">
                <h1 class="text-xl font-bold">"Acronymia"</h1>
                {move|| match step() {
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
