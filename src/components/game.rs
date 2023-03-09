use ::leptos::*;

use crate::sse::*;
use crate::types::*;

mod judging;
mod setup;
mod results;
mod submission;
mod context;
use self::judging::*;
use self::setup::*;
use self::results::*;
use self::submission::*;
use self::context::*;


#[component]
pub fn Game(cx: Scope) -> impl IntoView {
    provide_game_context(cx);
    let game_step = create_sse_signal::<GameStep>(cx);
    let game_step = create_memo(cx, move |_| game_step());
    view! {
        cx,

        <Transition
            fallback=move || view! { cx, "Loading" }
        >
            { move || match game_step() {
                None => view! {cx, <><GameNotFound /></>},
                Some(GameStep::Setup) => view! { cx, <><GameSetup /></> },
                Some(GameStep::Submission) => view! { cx, <><GameSubmission /></> },
                Some(GameStep::Judging) => view! { cx, <><GameJudging /></> },
                Some(GameStep::Results) => view! { cx, <><GameResults /></> },
            }}
        </Transition>
    }
}

#[component]
fn GameNotFound(_cx: Scope) -> impl IntoView {
    view! {
        cx,
        "Game not found!"
    }
}
