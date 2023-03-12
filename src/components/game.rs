use ::leptos::*;

use crate::components::reset_button::*;
use crate::components::utils::*;
use crate::sse;
use crate::sse::*;
use crate::types::*;

mod context;
mod judging;
mod player_roster;
mod results;
mod setup;
mod submission;
mod timer;
use self::context::*;
use self::judging::*;
use self::player_roster::*;
use self::results::*;
use self::setup::*;
use self::submission::*;

#[component]
pub fn Game(cx: Scope) -> impl IntoView {
    provide_game_context(cx);
    let game_step = create_memo(cx, move |_| game_state(cx).with(|g| g.step.clone()));
    let debug_region_expanded = create_rw_signal(cx, false);

    create_effect(cx, move |_| {
        use wasm_bindgen::*;

        // whenever the game step changes, blur focused elements so that the
        // autofocus attribute on the next page will work
        _ = game_step();
        let q = document().query_selector(":focus");
        log!("{:#?}", q);
        if let Ok(Some(el)) = q {
            log!("{:#?}", el.unchecked_into::<web_sys::HtmlElement>().blur());
        }
    });

    view! {
        cx,
        <div class="flex flex-col items-start mx-20 my-4 gap-4">
           <h1 class="text-xl font-bold">"Acronymia"</h1>
            { move || match game_step() {
                GameStep::Setup => view! { cx, <><GameSetup /></> },
                GameStep::Submission => view! { cx, <><GameSubmission /></> },
                GameStep::Judging => view! { cx, <><GameJudging /></> },
                GameStep::Results => view! { cx, <><GameResults /></> },
            }}
            {debug_view(cx, view! {cx,
                <button
                    class="border rounded p-2 bg-slate-200"
                    on:click=move |_| debug_region_expanded.update(|b| *b = !*b)
                >
                    "Toggle Debug View"
                </button>
                <When predicate=debug_region_expanded >
                    <div class="flex flex-col items-start gap-4">
                        <h1 class="font-bold font-xl">"Begin Debug"</h1>
                        <PlayerRoster />
                        <div>{move || format!("game_state = {:#?}", sse::game_state(cx).get())}</div>
                        <ResetButton />
                        <h1 class="font-bold font-xl">"End Debug"</h1>
                    </div>
                </When>
            })}
        </div>
    }
}
