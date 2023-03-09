use leptos::*;

/// Conditionally render a view. Just to reduce boilerplate
#[component]
pub fn When(
    cx: Scope,
    predicate: MaybeSignal<bool>,
    children: Box<dyn Fn(Scope) -> Fragment>,
) -> impl IntoView {
    move || {
        if predicate() {
            view! {cx, <>{children(cx)}</>}
        } else {
            view! {cx, <></>}
        }
    }
}

#[component]
pub fn If(
    cx: Scope,
    r#if: MaybeSignal<bool>,
    then: Box<dyn Fn(Scope) -> Fragment>,
    r#else: Box<dyn Fn(Scope) -> Fragment>,
) -> impl IntoView {
    move || {
        if r#if() {
            view! {cx, <>{then(cx)}</>}
        } else {
            view! {cx, <>{r#else(cx)}</>}
        }
    }
}

/// A component that only exists in debug mode
#[component]
pub fn Debug(cx: Scope, children: Box<dyn Fn(Scope) -> Fragment>) -> impl IntoView {
    if cfg!(debug_assertions) {
        view! {cx, <>{children(cx)}</>}
    } else {
        view! {cx, <></>}
    }
}
