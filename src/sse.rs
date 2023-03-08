use gloo_net::eventsource::*;
use web_sys::MessageEvent;
use serde::*;
use leptos::*;


/// readonly signal that subscribes to Server Sent Events
pub fn create_sse_signal<T>(cx: Scope) -> impl Fn() -> Option<T>
where
    T: Clone + for<'de> Deserialize<'de>,
{
    #[cfg(feature = "ssr")]
    let signal = _fake_sse_signal(cx);

    #[cfg(not(feature = "ssr"))]
    let signal = _sse_signal(cx);

    move || {
        let payload = signal();
        dbg!(&payload);
        let js_val = payload?.ok().map(|x| x.1.data()).into();
        dbg!(&js_val);
    
        serde_wasm_bindgen::from_value::<T>(js_val).ok()
    }
}

type SsePayload = Option<Result<(String, MessageEvent), EventSourceError>>;

/// raw signal that subscribes to Server Sent Events
fn _sse_signal(cx: Scope) -> ReadSignal<SsePayload> {
    use gloo_net::eventsource::futures::EventSource;
    use serde_wasm_bindgen::*;

    let mut source = EventSource::new("/api/events").expect("couldn't connect to SSE stream");
    let stream = source.subscribe("message").expect("subscription");
    let signal = create_signal_from_stream(cx, stream);

    on_cleanup(cx, move || source.close());
    signal
}

/// signal that is never invoked, just to satisfy compiler during SSR
fn _fake_sse_signal(cx: Scope) -> ReadSignal<SsePayload> {
    let (signal, _) = create_signal(cx, None);
    signal
}
