use crate::components::game::context::*;
use crate::components::game::player_roster::*;
use crate::components::reset_button::*;
use crate::components::state::*;
use crate::components::{styles::*, utils::*};
use crate::constants::*;
use crate::typed_context::*;
use ::leptos::*;

#[component]
pub fn DebugView(cx: Scope) -> impl IntoView {
    let is_host = use_typed_context::<Memo_IsHost>(cx);
    let debug_region_expanded = create_rw_signal(cx, false);
    let stop_timer = create_action(cx, move |_| send(cx, StopTimer));

    view! { cx,
            <When predicate=MaybeSignal::derive(cx, move|| is_host() || DEBUG_MODE) >
                <button
                    class=button_class(ButtonStyle::Neutral, "mt-4")
                    on:click=move |_| debug_region_expanded.update(|b| *b = !*b)
                >
                    "Toggle Debug View"
                </button>
                <When predicate=debug_region_expanded >
                    <div class="flex flex-col items-start gap-4">
                        <h1 class="font-bold font-xl">"Begin Debug"</h1>
                        <p>
                            "You are "<PlayerName />
                        </p>
                        <PlayerRoster />
                        <button
                            class=button_class(ButtonStyle::Secondary, "")
                            on:click=move|_| stop_timer.dispatch(())
                        >
                            "Stop timer"
                        </button>
                        <ResetButton />

                        <h1 class="font-bold font-xl">"End Debug"</h1>
                    </div>
                </When>
            </When>
    }
}

#[component]
fn PlayerName(cx: Scope) -> impl IntoView {
    move || {
        get_name(cx).map_or(
            view! { cx,
                <span class="inline font-bold text-red-300">
                    "nobody"
                </span>
            },
            |name| {
                view! { cx,
                    <span class="inline font-bold text-green-300">
                        {name}
                    </span>
                }
            },
        )
    }
}

fn get_name(cx: Scope) -> Option<String> {
    use_typed_context::<Signal_PlayerId>(cx).with(|id| {
        id.as_ref().and_then(|id| {
            game_state(cx).with(|g| {
                g.players
                    .iter()
                    .find(|p| p.id == *id)
                    .map(|p| p.name.clone())
            })
        })
    })
}
