use super::context::*;
use super::utils::state::*;
use crate::components::{styles::*, utils::*};
use crate::types::ClientMessage::*;
use crate::types::*;
use ::futures::future::OptionFuture;
use ::leptos::*;

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
        <label>"Pick a Nickname to join: "</label>
        <input
            type="text"
            class=text_input_class("")
            default=player_name()
            on:input=move |e| player_name.set(event_target_value(&e))
            on:keydown=move |e| {
                if e.key() == "Enter" {
                    join_game.dispatch(());
                }
            }
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
        <div>
            <p>{move || players.with(|ps| ps.len())}" players joined"</p>
            <ul class="list-inside list-disc flex flex-col items-start">
                {move|| players.with(|ps| ps
                    .iter()
                    .map(|p| view! { cx, <li>{p.name.clone()}</li>})
                    .collect::<Vec<_>>()
                )}
            </ul>
        </div>
        <h1 class="text-xl font-bold">"Configuration"</h1>
        <Range />
    }
}

#[component]
pub fn Range(cx: Scope) -> impl IntoView {
    const MIN: usize = 2;
    let lower_bound = create_rw_signal::<usize>(cx, MIN);
    let upper_bound = create_rw_signal::<usize>(cx, 6);
    view! { cx,
        <div>{move|| lower_bound}</div>
        <div>{move|| upper_bound}</div>
        <div class="flex flex-row gap-2">
            "From"
            <input
                type="number"
                class=number_input_class("w-[4rem]")
                min=MIN
                prop:max=upper_bound
                prop:value=lower_bound
                on:change=move|e| {
                    if let Ok(n) = event_target_value(&e).parse() {
                        lower_bound.set(n);
                    }
                }
            />
            "to"
            <input
                type="number"
                class=number_input_class("w-[4rem]")
                prop:min=lower_bound
                prop:value=upper_bound
                on:input=move|e| {
                    if let Ok(n) = event_target_value(&e).parse() {
                        upper_bound.set(n);
                    }
                }
            />
            "letters per acronym"
        </div>
    }
}
