use leptos::*;

pub const DEBUG_MODE: bool = cfg!(debug_assertions);

/// A component that only exists in debug mode
#[component]
pub fn Debug(cx: Scope, children: Box<dyn Fn(Scope) -> Fragment>) -> impl IntoView {
    if DEBUG_MODE {
        view! {cx, <>{children(cx)}</>}
    } else {
        view! {cx, <></>}
    }
}

/// Conditionally render a view. Just to reduce boilerplate
#[component]
pub fn When<P>(cx: Scope, predicate: P, children: Box<dyn Fn(Scope) -> Fragment>) -> impl IntoView
where
    P: 'static + Fn() -> bool,
{
    move || {
        if predicate() {
            view! {cx, <>{children(cx)}</>}
        } else {
            view! {cx, <></>}
        }
    }
}
