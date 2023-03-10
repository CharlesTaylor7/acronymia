use leptos::*;

#[component]
pub fn Timer(cx: Scope, initial: u64) -> impl IntoView {
    let seconds = timer(cx, initial);
    view! { cx, "Seconds: "{seconds}}
}

/// counts down from initial value to 0
pub fn timer(cx: Scope, initial: u64) -> RwSignal<u64> {
    let seconds = create_rw_signal(cx, initial);
    #[cfg(not(feature = "ssr"))]
    {
        use crate::sse::*;
        use leptos_dom::helpers::IntervalHandle;
        use std::time::Duration;

        let stored = store_value::<Option<IntervalHandle>>(cx, None);
        let handle = set_interval(
            move || {
                seconds.update(|s| {
                    log!("browser");
                    if *s > 0 {
                        *s = *s - 1
                    } else {
                        // clear interval when time reaches 0
                        stored.with_value(|h| h.map(|h| h.clear()));
                    }
                })
            },
            Duration::new(1, 0),
        );
        stored.set_value(handle.ok());

        // clear interval if the scope is dropped
        on_cleanup(cx, move || {
            stored.with_value(|h| h.map(|h| h.clear()));
        });
    }

    seconds
}
/// counts down from initial value to 0
pub fn apply_timer_to_game(cx: Scope) {
    let _ = cx;
    #[cfg(not(feature = "ssr"))]
    {
        use crate::sse::*;
        use leptos_dom::helpers::IntervalHandle;
        use std::time::Duration;

        let stored = store_value::<Option<IntervalHandle>>(cx, None);
        let handle = set_interval(
            move || {
                game_state(cx).update(|g| {
                    g.round_timer.as_mut().map(|s| {
                        log!("browser");
                        if *s > 0 {
                            *s = *s - 1
                        } else {
                            // clear interval when time reaches 0
                            stored.with_value(|h| h.map(|h| h.clear()));
                        }
                    });
                })
            },
            Duration::new(1, 0),
        );
        stored.set_value(handle.ok());

        // clear interval if the scope is dropped
        on_cleanup(cx, move || {
            stored.with_value(|h| h.map(|h| h.clear()));
        });
    }
}
