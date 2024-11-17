use crate::components::styles::*;
use ::leptos::prelude::*;
use ::leptos_router::*;

#[component]
pub fn Lobby() -> impl IntoView {
    #[cfg(not(feature = "hydrate"))]
    let on_click = move |_| ();

    #[cfg(feature = "hydrate")]
    let on_click = move |_| {
        use ::uuid::*;
        let code = Uuid::new_v4().to_string();
        let nav = use_navigate();
        nav(&format!("room/{}", code), NavigateOptions::default());
    };

    view! {
        Welcome to the lobby!
        <button
            class=ButtonStyle::Primary.class()
            on:click=on_click
        >
            Start New Game
        </button>

        <A href="/room/stale" class=ButtonStyle::Danger.class()>
            Demo Stale Game Link
        </A>
    }
}
