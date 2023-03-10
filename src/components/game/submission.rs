use crate::components::timer::*;
use crate::sse::*;
use ::leptos::*;

#[component]
pub fn GameSubmission(cx: Scope) -> impl IntoView {
    let seconds = timer(cx, 30);
    create_effect(cx, move |_| {
        if let Some(t) = game_state(cx).and_then(|s| s.round_timer) {
            seconds.set(t);
        }
    });

    view! {
        cx,
        "Seconds remaining: "{seconds}
    }
}
