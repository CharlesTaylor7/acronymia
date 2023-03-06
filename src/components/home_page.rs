use crate::components::text_input::*;
use leptos::*;
use leptos_router::*;
use crate::api::*;

/// The home page allows you to:
/// - Set your nickname
/// - Join a game
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let name = create_rw_signal::<String>(cx, "boaty_mcboatface".to_owned());
    let room_code = create_rw_signal::<String>(cx, "abc".to_owned());
    let join = create_action(cx, move |name: &String| join_game(name.clone()));
    let navigate = use_navigate(cx);
    view! {
        cx,
        <h1>"Welcome to Acronymia!"</h1>
        "Enter your nickname:"
        <TextInput signal=name />

        "Enter your room code: "
        <TextInput signal=room_code />
        <button
            on:click=move |_| {
                join.dispatch(name.get());
                navigate(&format!("/game/{}?name={}", room_code.get(), name.get()), Default::default());
            }
        >
            "Join!"
        </button>
        <p>{ name }</p>
    }
}
