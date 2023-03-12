use super::context::*;
use crate::api;
use crate::components::text_input::*;
use crate::components::utils::*;
use crate::sse::*;
use crate::typed_context::*;
use crate::types::*;
use ::leptos::*;
use futures::future::OptionFuture;

#[component]
pub fn GameSetup(cx: Scope) -> impl IntoView {
    let player_id = use_typed_context::<Signal_PlayerId>(cx);
    let player_name = use_typed_context::<Signal_PlayerName>(cx);
    let join_game = create_action(cx, move |_: &()| {
        OptionFuture::from(player_id().map(|id| api::join_game(id, player_name())))
    });
    let kick_player = create_action(cx, move |id: &PlayerId| api::kick_player(id.clone()));
    let start_game = create_action(cx, move |_: &()| {
        game_state(cx).update(|g| {
            g.step = GameStep::Submission;
            g.round_timer = Some(30);
        });
        api::start_game()
    });

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
    let players: Memo<Vec<Player>> = create_memo(cx, move |_| game_state(cx).get().players);

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
                    class="border rounded p-2 bg-blue-300 border-slate-200"
                    prop:disabled=Signal::derive(cx, move|| player_id().is_none())
                    on:click=move |_| join_game.dispatch(())
                >
                    "Join!"
                </button>

                <When predicate=MaybeSignal::derive(cx, move|| is_creator() || DEBUG_MODE)>
                    <button
                        class="border rounded p-2 bg-green-300 border-slate-200"
                        on:click=move |_| start_game.dispatch(())
                    >
                        "Start game!"
                    </button>
                </When>
            </div>

            <p>{move || players.get().len()}" players joined"</p>
            <ul class="list-inside list-disc flex flex-col items-start" >
                {move|| players()
                    .into_iter()
                    .map(|p| view! { cx,
                        <li>
                            {p.name.clone()}
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
