use leptos::*;
use std::time::Duration;

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
        use leptos_dom::helpers::IntervalHandle;
        let stored = store_value::<Option<IntervalHandle>>(cx, None);
        let handle = set_interval(
            move || {
                let s = seconds.get();
                if s > 0 {
                    seconds.set(s - 1);
                } else {
                    // stored.with_value(|h| h.map(|h| h.clear()));
                }
            },
            Duration::new(1, 0),
        );
        stored.set_value(handle.ok());

        // cleanup the handle if the scope is dropped
        on_cleanup(cx, move || {
            stored.with_value(|h| h.map(|h| h.clear()));
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
