use crate::components::input::*;
use leptos::*;
use leptos_router::*;

/// The home page allows you to:
/// - Set your nickname
/// - Join a game
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let name = create_rw_signal::<String>(cx, "boaty_mcboatface".to_owned());
    let room_code = create_rw_signal::<String>(cx, "abc".to_owned());

    view! {
        cx,
        <h1>"Welcome to Acronymia!"</h1>
        "Enter your nickname:"
        <TextInput signal=name />

        "Enter your room code: "
        <TextInput signal=room_code />
        <A
            href=move|| format!("/game/{}?name={}", room_code.get(), name.get())
        >
            "Join!"
        </A>
        <p>{ name }</p>
    }
}
