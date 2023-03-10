use leptos::*;
use std::time::Duration;

#[component]
pub fn Timer(cx: Scope) -> impl IntoView {
    let seconds = timer(cx, 60);
    view! { cx, "Seconds: "{seconds}}
}

/// counts down from initial value to 0
fn timer(cx: Scope, initial: u32) -> RwSignal<u32> {
    let seconds = create_rw_signal(cx, initial);

    #[cfg(not(feature = "ssr"))]
    {
        let handle = set_interval(
            move || {
                let s = seconds.get();
                if s > 0 {
                    seconds.set(s - 1);
                }
            },
            Duration::new(1, 0),
        );
        log::debug!("{:?}", &handle);
        on_cleanup(cx, move || {
            _ = handle.map(|h| h.clear());
        });

    }

    seconds
}

/// counts up from an initial value
pub fn clock(cx: Scope, initial: u32) -> RwSignal<u32> {
    let seconds = create_rw_signal(cx, initial);

    // This effect doesn't depend on any signal reactively. Therefore it only runs once.
    // However, we still have to wrap it in create_effect to ensure it runs on the client,
    // not during server side rendering.
    create_effect(cx, move |_| {
        let handle = set_interval(
            move || {
                let s = seconds.get();
                seconds.set(s + 1);
            },
            Duration::new(1, 0),
        );
        log::debug!("{:?}", &handle);
        on_cleanup(cx, move || {
            _ = handle.map(|h| h.clear());
        });
    });

    seconds
}
