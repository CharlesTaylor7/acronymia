use crate::components::styles::*;
use ::leptos::*;
use ::leptos_router::*;

#[component]
pub fn Lobby() -> impl IntoView {
    #[cfg(feature = "ssr")]
    let on_click = move |_| ();

    #[cfg(feature = "hydrate")]
    let on_click = move |_| {
        use ::uuid::*;
        let code = Uuid::new_v4().to_string();
        let nav = use_navigate();
        nav(&format!("room/{}", code), NavigateOptions::default())
    };

    view! {
        Welcome to the lobby!
        <button
            class=button_class(ButtonStyle::Primary, "")
            on:click=on_click
        >
            Start New Game
        </button>

        <A href="/room/stale" class=button_class(ButtonStyle::Danger, "")>
            Demo Stale Game Link
        </A>
    }
}
