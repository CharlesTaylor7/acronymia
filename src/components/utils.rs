use leptos::*;

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
