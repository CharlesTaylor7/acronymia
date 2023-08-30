use super::context::*;
use crate::components::game::*;
use crate::types::ClientMessage::*;
use crate::types::*;
use ::leptos::*;

#[component]
pub fn PlayerRoster() -> impl IntoView {
    let players = use_typed_context::<Memo_Players>();
    let impersonate = SignalSetter::from(use_typed_context::<Signal_PlayerId>());

    view! {
        <ul class="gap-3 list-inside list-disc flex flex-col items-start" >
            <For
                each=players
                key=|p| p.id.clone()
                view=move |p| view! {
                    <PlayerView
                        player=p
                        impersonate=impersonate
                    />
                }
            />
        </ul>
    }
}

#[component]
fn PlayerView(player: Player, impersonate: SignalSetter<Option<PlayerId>>) -> impl IntoView {
    let player_id = use_typed_context::<Signal_PlayerId>();
    let action = create_ws_action();

    // TODO: why do I have to clone this variable so many times?
    // If I try to only once in each callback, I get weird ownership errors.
    let id1 = player.id.clone();
    let id2 = player.id.clone();
    let id3 = player.id.clone();
    view! {

        <li>
            {player.name}
            <button
                class="bg-cyan-500 text-blue-50 rounded mx-2 px-2 disabled:bg-slate-600"
                on:click=move|_| impersonate.set(Some(id1.clone()))
            >
                "Impersonate"
            </button>
            <button
                class="bg-rose-400 text-blue-50 rounded mx-2 px-2 disabled:bg-slate-600"
                disabled=move|| Some(id3.clone()) == player_id.get() // can't kick self
                on:click=move|_| action.dispatch(KickPlayer(id2.clone()))
            >
                "Kick"
            </button>
        </li>
    }
}
