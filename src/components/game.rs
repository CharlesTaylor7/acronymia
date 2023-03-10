use ::leptos::*;

use crate::components::reset_button::*;
use crate::components::text_input::*;
use crate::components::utils::*;
use crate::typed_context::*;
use crate::types::*;
use crate::sse;
use crate::sse::*;

mod context;
mod judging;
mod results;
mod setup;
mod submission;
use self::context::*;
use self::judging::*;
use self::results::*;
use self::setup::*;
use self::submission::*;


#[component]
pub fn Game(cx: Scope) -> impl IntoView {
    provide_game_context(cx);
    let player_id = use_typed_context::<Signal_PlayerId>(cx);
    let game_step = create_memo(cx, move |_| game_state(cx).map(|s| s.step.clone()));
    let debug_region_collapsed = create_rw_signal(cx, true);
    view! {
        cx,
        <div class="flex flex-col items-start mx-20 my-4 gap-4">
            <Debug>
                <button
                    class="border rounded p-2 bg-slate-200"
                    on:click=move |_| debug_region_collapsed.update(|b| *b = !*b)
                >
                    "Toggle Debug View"
                </button>
                <div
                    class:hidden=debug_region_collapsed
                    class="flex flex-col items-start gap-4"
                >
                    <h1 class="font-bold font-xl">"Begin Debug"</h1>
                    <p>"Override player id: "</p>
                    <TextInput
                        default=player_id().unwrap_or("".to_string())
                        disabled=MaybeSignal::derive(cx, move|| player_id().is_some())
                        on_input=move |text| player_id.set(Some(text))
                    />
                    <ResetButton/>
                    <p>{move || format!("player_id = {:#?}", player_id())}</p>
                    <p>{move || format!("game_state = {:#?}", sse::game_state(cx))}</p>
                    <h1 class="font-bold font-xl">"End Debug"</h1>
                </div>
            </Debug>
            <h1 class="text-xl font-bold">"Acronymia"</h1>
            { move || match game_step() {
                None => view! {cx, <><GameNotFound /></>},
                Some(GameStep::Setup) => view! { cx, <><GameSetup /></> },
                Some(GameStep::Submission) => view! { cx, <><GameSubmission /></> },
                Some(GameStep::Judging) => view! { cx, <><GameJudging /></> },
                Some(GameStep::Results) => view! { cx, <><GameResults /></> },
            }}
        </div>
    }
}

#[component]
fn GameNotFound(_cx: Scope) -> impl IntoView {
    view! {
        cx,
        "Game not found!"
    }
}
