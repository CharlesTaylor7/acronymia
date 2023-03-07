use crate::api::*;
use crate::components::text_input::*;
use leptos::*;
use leptos_router::*;

/// The home page allows you to:
/// - Set your nickname
/// - Join a game
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let name = create_rw_signal::<String>(cx, "boaty_mcboatface".to_owned());
    let join = create_action(cx, move |name: &String| join_game(name.clone()));
    let navigate = use_navigate(cx);
    view! {
        cx,
        <h1>"Welcome to Acronymia!"</h1>
        "Enter your nickname:"
        <TextInput signal=name />

        <button
            on:click=move |_| {
                join.dispatch(name.get().clone());
                navigate("/game", Default::default());
                /*
                let result = block_on(join_game(name.get().clone()));
                
                dbg!(&result);
                match result {
                    Ok(_) => 
                        navigate("/game", Default::default()),
                    Err(_) => Ok(()),
                };
                */
            }
        >
            "Join!"
        </button>
        <p>{ name }</p>
    }
}
