use leptos::*;

/// Conditionally render a view. Just to reduce boilerplate
#[component]
pub fn When(cx: Scope, predicate: MaybeSignal<bool>, children: Box<dyn Fn(Scope) -> Fragment>) -> impl IntoView 
{
    move || {
        if predicate() {
            view! {cx, {children(cx)}}
        }
        else {
            view! {cx, <></>}
        }
    }
}


/// A component that only exists in debug mode
#[component]
pub fn Debug(cx: Scope, children: Box<dyn Fn(Scope) -> Fragment>) -> impl IntoView {
    if cfg!(debug_assertions) {
        view! {cx, {children(cx)}}
    } else {
        view! {cx, <></>}
    }
}
