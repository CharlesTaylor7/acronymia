use super::context::*;
use super::utils::state::*;
use crate::components::{styles::*, utils::*};
use crate::types::ClientMessage::*;
use crate::types::*;
use ::leptos::*;
use futures::future::OptionFuture;

#[component]
pub fn GameSetup(cx: Scope) -> impl IntoView {
    let is_host = use_typed_context::<Memo_IsHost>(cx);
    let player_id = use_typed_context::<Signal_PlayerId>(cx);
    let player_name = use_typed_context::<Signal_PlayerName>(cx);
    let player = create_memo(cx, move |_| {
        player_id().map(|id| Player {
            id,
            name: player_name(),
        })
    });

    let players = use_typed_context::<Memo_Players>(cx);
    let join_game = create_action(cx, move |_: &()| {
        OptionFuture::from(player().map(|p| send(cx, JoinGame(p))))
    });

    let start_game = create_action(cx, move |_: &()| send(cx, StartGame));
    view! {
        cx,
       <div class="flex flex-col items-start gap-4">
            "Pick a Nickname to join: "
            <input
                type="text"
                class=text_input_class("")
                default=player_name()
                on:input=move |e| player_name.set(event_target_value(&e))
            />
            <div class="flex flex-row gap-4">
                <button
                    class=button_class("bg-blue-300")
                    prop:disabled=Signal::derive(cx, move|| player_id().is_none())
                    on:click=move |_| join_game.dispatch(())
                >
                {move|| if join_game.version()() > 0 { "Update name" } else { "Join" }}
                </button>

                <When predicate=is_host >
                    <button
                        class=button_class("bg-green-300")
                        disabled=move|| players.with(|ps| ps.len() < 3)
                        on:click=move |_| start_game.dispatch(())
                    >
                        "Start game!"
                    </button>
                </When>
            </div>
            <p>{move || players.with(|ps| ps.len())}" players joined"</p>
            <ul class="list-inside list-disc flex flex-col items-start">
                {move|| players.with(|ps| ps
                    .iter()
                    .map(|p| view! { cx, <li>{p.name.clone()}</li>})
                    .collect::<Vec<_>>()
                )}
            </ul>
        </div>
    }
}
