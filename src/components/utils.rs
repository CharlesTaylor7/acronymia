use leptos::*;

/// Conditionally render a view. Just to reduce boilerplate
pub fn when(cx: Scope, condition: bool, view: impl IntoView) -> impl IntoView {
    if condition {
        view! {
            cx,
            <>{view}</>
        }
    }
    else {
        view! {
            cx,
            <></>
        }
    }
}
