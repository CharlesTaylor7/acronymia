use ::leptos::*;

use crate::components::text_input::*;
use crate::sse::*;
use crate::typed_context::*;
use crate::types::*;
use super::context::*;
use crate::api;


#[component]
pub fn GameSetup(cx: Scope) -> impl IntoView {
    let player_id = use_typed_context::<Signal_PlayerId>(cx);
    let player_name = use_typed_context::<Signal_PlayerName>(cx);
    let players = create_sse_signal::<Vec<Player>>(cx);
    let join_game = use_typed_context::<Action_JoinGame>(cx);
    let kick_player = create_action(cx, move |id: &PlayerId| api::kick_player(id.clone()));

    view! {
        cx,
       <div class="flex flex-col items-start gap-4">
            "Pick a Nickname to join: "
            <TextInput
                default=player_name()
                on_input=move |text| player_name.set(text)
            />
            <button
                class="border rounded p-2 bg-blue-300 border-slate-200"
                prop:disabled=MaybeSignal::derive(cx, move|| player_id().is_none())
                on:click=move |_| join_game.dispatch(())
            >
                "Join!"
            </button>
            <p>{move || players().map(|v| v.len()).unwrap_or(0)}" players joined"</p>
            <ul class="list-inside list-disc flex flex-col items-start" >
                {move|| players()
                    .into_iter()
                    .flatten()
                    .map(|p| view! { cx, 
                        <li>
                            {p.name} 
                            <button 
                                class="bg-red-200 border rounded mx-2 px-2 border-slate-200"
                                on:click=move |_| kick_player.dispatch(p.id.clone())
                            >
                                "Kick"
                            </button>
                        </li>
                    })
                    .collect::<Vec<_>>()
                }
            </ul>
        </div>
    }
}
