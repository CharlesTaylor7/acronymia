use ::leptos::*;

use crate::components::text_input::*;
use crate::sse::*;
use crate::typed_context::*;
use crate::types::*;
use super::context::*;


#[component]
pub fn GameSetup(cx: Scope) -> impl IntoView {
    let player_id = use_typed_context::<Signal_PlayerId>(cx);
    let player_name = use_typed_context::<Signal_PlayerName>(cx);
    let players = create_sse_signal::<Vec<Player>>(cx);
    let join_game = use_typed_context::<Action_JoinGame>(cx);

    view! {
        cx,
       <div>
            "Pick a Nickname to join: "
            <TextInput
                default=player_name()
                on_input=move |text| player_name.set(text)
            />
            <button
                class="border rounded p-2 m-2 bg-blue-300 border-slate-200"
                prop:disabled=MaybeSignal::derive(cx, move|| player_id().is_none())
                on:click=move |_| join_game.dispatch(())
            >
                "Join!"
            </button>
            <p> "Players: "</p>
            <ul class="list-inside list-disc" >
                {move|| players()
                    .into_iter()
                    .flatten()
                    .map(|p| view! {cx, <li>{p.name}</li>})
                    .collect::<Vec<_>>()
                }
            </ul>
        </div>
    }
}
