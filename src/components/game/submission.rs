use crate::components::timer::*;
use crate::sse::*;
use ::leptos::*;
use ::core::time::*;

#[component]
pub fn GameSubmission(cx: Scope) -> impl IntoView {
    let seconds = timer(cx, 30);

    view! {
        cx,
        "Seconds remaining: "{seconds}
    }
}
