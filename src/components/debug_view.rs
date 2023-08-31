use crate::components::game::context::*;
use crate::components::game::player_roster::*;
use crate::components::reset_button::*;
use crate::components::state::*;
use crate::components::styles::*;
use crate::constants::*;
use crate::typed_context::*;
use ::leptos::*;

#[component]
pub fn DebugView() -> impl IntoView {
    let is_host = use_typed_context::<Memo_IsHost>();
    let debug_region_expanded = create_rw_signal(false);
    let action = create_ws_action();

    view! {
        <Show when=MaybeSignal::derive(move|| is_host() || DEV_MODE) fallback=|| ()>
            <button
                class=button_class(ButtonStyle::Neutral, "mt-4")
                on:click=move|_| debug_region_expanded.update(|b| *b = !*b)
            >
                "Toggle Debug View"
            </button>
            <Show when=debug_region_expanded fallback=|| () >
                <div class="flex flex-col items-start gap-4">
                    <h1 class="font-bold font-xl">"Begin Debug"</h1>
                    <p>
                        "You are "<PlayerName />
                    </p>
                    <PlayerRoster />
                    <button
                        class=button_class(ButtonStyle::Secondary, "")
                        on:click=move|_| action.dispatch(StopTimer)
                    >
                        "Stop timer"
                    </button>
                    Warning: This ends the current game in progress, and kicks all players.
                    <ResetButton />
                    <h1 class="font-bold font-xl">"End Debug"</h1>
                </div>
            </Show>
        </Show>
    }
}

#[component]
fn PlayerName() -> impl IntoView {
    move || {
        get_name().map_or(
            view! {
                <span class="inline font-bold text-red-300">
                    "nobody"
                </span>
            },
            |name| {
                view! {
                    <span class="inline font-bold text-green-300">
                        {name}
                    </span>
                }
            },
        )
    }
}

fn get_name() -> Option<String> {
    let player_id = use_typed_context::<Signal_PlayerId>();
    let game_state = use_typed_context::<Signal_GameState>();
    player_id.with(|id| {
        id.as_ref().and_then(|id| {
            game_state.with(|g| {
                g.players
                    .iter()
                    .find(|p| p.id == *id)
                    .map(|p| p.name.clone())
            })
        })
    })
}
