use super::context::*;
use crate::components::game::send;
use crate::types::ClientMessage::*;
use crate::types::*;
use ::leptos::*;

#[component]
pub fn PlayerRoster(cx: Scope) -> impl IntoView {
    let players = use_typed_context::<Memo_Players>(cx);
    let kick_player = create_action(cx, move |id: &PlayerId| send(cx, KickPlayer(id.clone())));
    let impersonate = SignalSetter::from(use_typed_context::<Signal_PlayerId>(cx));

    view! {
        cx,
        <ul class="gap-3 list-inside list-disc flex flex-col items-start" >
            {move||
                players.with(|ps| ps
                    .iter()
                    .map(|p| view! { cx,
                        <PlayerView
                            player=p.clone()
                            kick_player=kick_player
                            impersonate=impersonate
                        />
                    })
                    .collect::<Vec<_>>()
                )
            }
        </ul>
    }
}

#[component]
fn PlayerView(
    cx: Scope,
    player: Player,
    kick_player: Action<PlayerId, ()>,
    impersonate: SignalSetter<Option<PlayerId>>,
) -> impl IntoView {
    let player_id = use_typed_context::<Signal_PlayerId>(cx);
    // TODO: why do I have to clone this variable so many times?
    // If I try to only once in each callback, I get weird ownership errors.
    let id1 = player.id.clone();
    let id2 = player.id.clone();
    let id3 = player.id.clone();
    view! {
        cx,
        <li>
            {player.name}
            <button
                class="bg-cyan-500 rounded mx-2 px-2 disabled:bg-slate-600"
                on:click=move |_| impersonate(Some(id1.clone()))
            >
                "Impersonate"
            </button>
            <button
                class="bg-rose-400 rounded mx-2 px-2 disabled:bg-slate-600"
                disabled=move|| Some(id3.clone()) == player_id() // can't kick self
                on:click=move |_| kick_player.dispatch(id2.clone())
            >
                "Kick"
            </button>
        </li>
    }
}
