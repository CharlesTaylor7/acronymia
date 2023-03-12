use super::context::*;
use crate::api;
use crate::components::utils::*;
use crate::typed_context::*;
use crate::types::*;
use ::leptos::*;

#[component]
pub fn PlayerRoster(cx: Scope) -> impl IntoView {
    let players = use_typed_context::<Memo_Players>(cx);
    let kick_player = create_action(cx, move |id: &PlayerId| api::kick_player(id.clone()));
    let impersonate = SignalSetter::from(use_typed_context::<Signal_PlayerId>(cx));

    view! {
        cx,
        <ul class="list-inside list-disc flex flex-col items-start" >
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
    kick_player: Action<PlayerId, Server<()>>,
    impersonate: SignalSetter<Option<PlayerId>>,
) -> impl IntoView {
    // TODO: why do I have to clone this variable so many times?
    // If I try to only once in each callback, I get weird ownership errors.
    let id1 = player.id.clone();
    let id2 = player.id.clone();
    view! {
        cx,
        <li>
            {player.name}
            {debug_view(cx, view! {cx,
                <button
                    class="bg-blue-300 border rounded mx-2 px-2 border-slate-200"
                    on:click=move |_| impersonate(Some(id1.clone()))
                >
                    "Impersonate"
                </button>
                <button
                    class="bg-red-200 border rounded mx-2 px-2 border-slate-200"
                    on:click=move |_| kick_player.dispatch(id2.clone())
                >
                    "Kick"
                </button>
            })}
        </li>
    }
}