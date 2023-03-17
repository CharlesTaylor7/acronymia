use leptos::*;

/// counts down from initial value to 0
#[cfg(not(feature = "ssr"))]
pub fn apply_timer(cx: Scope) {
    use crate::components::game::context::*;
    use crate::components::game::utils::state::game_state;
    use std::time::Duration;

    let stored = use_typed_context::<TimerHandle>(cx);
    if stored.with_value(|s| s.is_some()) {
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

    let handle = set_interval(
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
pub fn apply_timer(_cx: Scope) {}
