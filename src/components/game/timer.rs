use crate::components::game::context::*;
use crate::components::state::*;
use crate::components::styles::*;
use crate::typed_context::*;
use leptos::*;

#[component]
pub fn Timer() -> impl IntoView {
    apply_timer();
    let game_state = use_typed_context::<Signal_GameState>();

    move || match game_state.with(|g| (g.timer, g.round_winner.is_some())) {
        (Some(secs), true) => view! {
            <p>
                <span class=counter_class()>{secs}</span>" seconds until next round"
            </p>
        },
        (Some(secs), false) => view! {
            <p>
                <span class=counter_class()>{secs}</span>" seconds remaining"
            </p>
        },
        (None, _) => view! {
            <p>
                "Times up!"
            </p>
        },
    }
}
/// counts down from initial value to 0
#[cfg(not(feature = "ssr"))]
fn apply_timer() {
    use crate::components::game::context::*;
    use std::time::Duration;

    let stored = use_typed_context::<TimerHandle>();
    if stored.with_value(|s| s.is_some()) {
        // skip if the timer interval is already in place
        return;
    }

    let cleanup = move || {
        stored.update_value(|h| {
            if let Some(handle) = h {
                handle.clear();
                *h = None;
            }
        });
    };

    let game_state = use_typed_context::<Signal_GameState>();
    let handle = set_interval_with_handle(
        move || {
            game_state.update(|g| match g.timer {
                Some(s) if s > 0 => {
                    g.timer = Some(s - 1);
                }
                _ => {
                    g.timer = Some(0);
                    cleanup();
                }
            });
        },
        Duration::new(1, 0),
    );
    stored.set_value(handle.ok());

    // clear interval if the scope is dropped
    on_cleanup(cleanup);
}

/// stub for ssr
#[cfg(feature = "ssr")]
fn apply_timer() {}
