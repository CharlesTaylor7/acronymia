use super::context::*;
use crate::components::game::*;
use crate::constants::DEV_MODE;
use crate::types::ClientMessage::*;
use crate::types::*;
use ::leptos::*;

#[component]
pub fn PlayerRoster() -> impl IntoView {
    let players = use_typed_context::<Memo_Players>();

    view! {
        <ul class="gap-3 list-inside list-disc flex flex-col items-start" >
            <For
                each=players
                key=|p| format!("{}-{}", p.id, p.name)
                view=move |p| view! {
                    <PlayerView
                        player=p
                    />
                }
            />
        </ul>
    }
}

#[component]
fn PlayerView(player: Player) -> impl IntoView {
    let player_id = use_typed_context::<Signal_PlayerId>();
    let action = create_ws_action();

    // TODO: why do I have to clone this variable so many times?
    // If I try to only once in each callback, I get weird ownership errors.
    let stored_id = store_value(player.id);
    let impersonate = move || player_id.set(Some(stored_id.get_value()));
    let kick = move || action.dispatch(KickPlayer(stored_id.get_value()));
    let disabled_kick =
        move || stored_id.with_value(|id1| player_id.with(|id2| Some(id1) == id2.as_ref()));
    view! {
        <li>
            {player.name}
            <Show
                fallback=|| ()
                when=|| DEV_MODE
            >
                <button
                    class="bg-cyan-500 text-blue-50 rounded mx-2 px-2 disabled:bg-slate-600"
                    on:click=move|_| impersonate()
                >
                    "Impersonate"
                </button>
            </Show>
            <button
                class="bg-rose-400 text-blue-50 rounded mx-2 px-2 disabled:bg-slate-600"
                disabled=disabled_kick
                on:click=move|_| kick()
            >
                "Kick"
            </button>
        </li>
    }
}
