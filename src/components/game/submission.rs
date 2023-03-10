use ::leptos::*;
use crate::components::timer::*;

#[component]
pub fn GameSubmission(cx: Scope) -> impl IntoView {
    let seconds = timer(cx, 30);
    view! {
        cx,
        "Seconds remaining: "{seconds}
    }
}
