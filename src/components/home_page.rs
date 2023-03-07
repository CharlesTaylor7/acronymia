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
        "Enter your nickname:"
        <TextInput signal=name />

        <button
            on:click=move |_| join.dispatch(name.get().clone())
        >
            "Register!"
        </button>
        {when(cx, !join.pending().get(), view! { cx, <A href="/game">"Join!"</A>})}
        /*
        
        {
            if !join.pending().get() {
                view! {
                    cx, 
                    <A href="/game">
                        "Join!"
                    </A>
                }
            }
        }
        */
    }
}

fn when(cx: Scope, condition: bool, view: impl IntoView) -> impl IntoView {
    if condition {
        view! {
            cx,
            <>{view}</>
        }
    }
    else {
        view! {
            cx,
            <></>
        }
    }
}
