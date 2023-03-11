use crate::components::timer::*;
use crate::sse::*;
use ::leptos::*;

#[component]
pub fn GameSubmission(cx: Scope) -> impl IntoView {
    apply_timer_to_game(cx);
    view! {
        cx,
        "Seconds remaining: "{move|| game_state(cx).with(|g| g.round_timer)}
    }
}
