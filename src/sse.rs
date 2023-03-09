use crate::types::*;
use gloo_net::eventsource::*;
use leptos::*;
use serde::*;
use web_sys::MessageEvent;

// Trait to ensure server and client use the same event types
pub trait ServerSentEvent: Serialize + for<'de> Deserialize<'de> {
    fn event_type() -> &'static str;
}

impl ServerSentEvent for GameStep {
    fn event_type() -> &'static str {
        "game-step"
    }
}

impl ServerSentEvent for Vec<Player> {
    fn event_type() -> &'static str {
        "players"
    }
}

use crate::typed_context::*;
use gloo_net::eventsource::futures::EventSource;
define_context!(SseStream, Option<EventSource>);

// Client side signal
pub fn provide_sse_stream(cx: Scope) {
    #[cfg(not(feature = "ssr"))]
    let source = EventSource::new("/api/events").ok();

    #[cfg(feature = "ssr")]
    let source = None;

    provide_typed_context::<SseStream>(cx, source);
    //("couldn't connect to SSE stream");
}

/// readonly signal that subscribes to Server Sent Events
pub fn create_sse_signal<T: ServerSentEvent>(cx: Scope) -> impl Copy + Fn() -> Option<T> {
    #[cfg(feature = "ssr")]
    let signal = _fake_sse_signal(cx);

    #[cfg(not(feature = "ssr"))]
    let signal = _sse_signal(cx, T::event_type());

    move || {
        signal()?
            .ok()?
            .1
            .data()
            .as_string()
            .and_then(|x| serde_json::from_str::<T>(&x).ok())
    }
}

type SsePayload = Option<Result<(String, MessageEvent), EventSourceError>>;

/// raw signal that subscribes to Server Sent Events
fn _sse_signal(cx: Scope, event_type: &str) -> ReadSignal<SsePayload> {
    let mut source = use_typed_context::<SseStream>(cx).expect("couldn't connect to SSE stream");
    let stream = source.subscribe(event_type).expect("couldn't subscribe");
    let signal = create_signal_from_stream(cx, stream);

    on_cleanup(cx, move || source.close());
    signal
}

/// signal that is never invoked, just to satisfy compiler during SSR
fn _fake_sse_signal(cx: Scope) -> ReadSignal<SsePayload> {
    let (signal, _) = create_signal(cx, None);
    signal
}

// server side api handler
use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature = "ssr")] {
        use ::futures::{stream, Stream};
        use ::futures::StreamExt;
        use actix_web::{
            Error,
            web::Bytes,
        };

        pub fn to_stream<T: ServerSentEvent>(event: T) ->
            impl Stream<Item = Result<Bytes, Error>> {
            stream::once(async {event})
                .map(|value| Ok::<_, Error>(to_bytes(value)))
        }

        fn to_bytes<T: ServerSentEvent>(event: T) -> Bytes {
            Bytes::from(format!(
                "event: {}\ndata: {}\n\n",
                <T as ServerSentEvent>::event_type(),
                 serde_json::to_string(&event).unwrap()
            ))
        }
    }
}
