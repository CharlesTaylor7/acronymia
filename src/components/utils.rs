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


pub fn render_option<T, F, V>(
    cx: Scope,
    o: Option<T>,
    fun: F,
    default: impl IntoView,
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
