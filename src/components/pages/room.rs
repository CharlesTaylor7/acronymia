use crate::components::game::Game;
use ::leptos::prelude::*;
use ::leptos_router::hooks::*;
use ::std::time::Duration;

#[component]
pub fn Room() -> impl IntoView {
    let params = use_params_map();
    let stale_code = String::from("stale");

    // room code is part of the url.
    // if the room code doesn't match an active game redirect back to the landing page
    view! {
        <Show
            fallback=Game
            when=move|| params.with(|p| &p.get("code") == stale_code)
        >
            <RedirectAfter
                fallback= || "The room you're looking for doesn't exist. Maybe the game ended?  Redirecting to lobby..."
                timeout=Duration::new(3,0)
                path="/lobby"
            />
        </Show>
    }
}

#[component]
#[allow(unused_variables)]
fn RedirectAfter<F, V>(fallback: F, path: &'static str, timeout: Duration) -> impl IntoView
where
    F: 'static + Fn() -> V,
    V: IntoView,
{
    let timeout_signal = RwSignal::new(false);

    #[cfg(feature = "hydrate")]
    set_interval(move || timeout_signal.set(true), timeout);

    view! {
        <Show
            fallback=fallback
            when=timeout_signal
        >
            <Redirect
                path=path
            />
        </Show>
    }
}
