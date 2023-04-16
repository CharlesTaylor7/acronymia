use crate::components::game::utils::state::game_state;
use crate::components::styles::*;
use leptos::*;

#[component]
pub fn Timer(cx: Scope) -> impl IntoView {
    apply_timer(cx);
    {
        move || match game_state(cx).with(|g| (g.timer, g.round_winner.is_some())) {
            (Some(secs), true) => view! { cx,
                <p>
                    <span class=counter_class()>{secs}</span>" seconds until next round"
                </p>
            },
            (Some(secs), false) => view! { cx,
                <p>
                    <span class=counter_class()>{secs}</span>" seconds remaining"
                </p>
            },
            (None, _) => view! { cx,
                <p>
                    "Times up!"
                </p>
            },
        }
    }
}
/// counts down from initial value to 0
#[cfg(not(feature = "ssr"))]
fn apply_timer(cx: Scope) {
    use crate::components::game::context::*;
    use std::time::Duration;

    let stored = use_typed_context::<TimerHandle>(cx);
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

    let handle = set_interval_with_handle(
        move || {
            game_state(cx).update(|g| match g.timer {
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
    on_cleanup(cx, cleanup);
}

/// stub for ssr
#[cfg(feature = "ssr")]
fn apply_timer(_cx: Scope) {}
