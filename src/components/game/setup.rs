use super::context::*;
use super::player_roster::*;
use super::utils::state::{game_state, send};
use crate::components::{styles::*, text_input::*, utils::*};
use crate::types::ClientMessage::*;
use crate::types::*;
use ::leptos::*;
use futures::future::OptionFuture;

#[component]
pub fn GameSetup(cx: Scope) -> impl IntoView {
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
    let is_creator: Memo<bool> = create_memo(cx, move |_| {
        player_id()
            .and_then(|me| {
                game_state(cx)
                    .get()
                    .players
                    .first()
                    .as_ref()
                    .map(|p| p.id == me)
            })
            .unwrap_or(false)
    });

    view! {
        cx,
       <div class="flex flex-col items-start gap-4">
            "Pick a Nickname to join: "
            <TextInput
                default=player_name()
                on_input=move |text| player_name.set(text)
            />
            <div class="flex flex-row gap-4">
                <button
                    class=button_class("bg-blue-300")
                    prop:disabled=Signal::derive(cx, move|| player_id().is_none())
                    on:click=move |_| join_game.dispatch(())
                >
                    "Join!"
                </button>

                <When predicate=MaybeSignal::derive(cx, move|| is_creator() || DEBUG_MODE)>
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
            <PlayerRoster />
        </div>
    }
}
