use leptos::*;

/// Conditionally render a view. Just to reduce boilerplate
pub fn when(cx: Scope, condition: bool, view: impl IntoView) -> impl IntoView {
    if condition {
        view! {
            cx,
            <>{view}</>
        }
    } else {
        view! {
            cx,
            <></>
        }
    }
}

/// Render an optional value or a default into a view
pub fn view_option<T, F, V>(
    cx: Scope,
    o: Option<T>,
    default: impl IntoView,
    fun: F,
) -> impl IntoView
where
    F: FnOnce(T) -> V,
    V: IntoView,
{
    match o {
        Some(item) => view! {
            cx,
            <>{fun(item)}</>
        },
        None => view! {
            cx,
            <>{default}</>
        },
    }
}

/// A component that only exists in debug mode
#[component]
pub fn Debug(cx: Scope, children: Box<dyn Fn(Scope) -> Fragment>) -> impl IntoView {
    if cfg!(debug_assertions) {
        view! {cx, <>{children(cx)}</> }
    } else {
        view! {cx, <></>}
    }
}
