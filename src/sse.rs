use crate::types::*;
use ::leptos::*;
use ::serde::*;

// Trait to ensure server and client use the same event types
pub trait ServerSentEvent:
    Default + core::fmt::Debug + PartialEq + Clone + Serialize + for<'de> Deserialize<'de>
{
    fn event_type() -> &'static str;
}

impl ServerSentEvent for ClientGameState {
    fn event_type() -> &'static str {
        "game-state"
    }
}

//pub struct SseSignal<T: 'a>(StoredValue<Box<dyn 'static + Fn() -> Option<T>>>);
/*
impl Fn<()> for SseSignal<T> {
}
*/

pub type SseSignal<T> = RwSignal<T>;

pub fn game_state(cx: Scope) -> SseSignal<ClientGameState> {
    // depend on the dummy signal which just tells us
    // when the inner event stream is swapped out
    if let Some(s) = use_context::<Signal<()>>(cx) {
        s.get();
    }

    // read the signal from context
    use_context::<SseSignal<ClientGameState>>(cx)
        .expect("did you forget to call provide_game_state?")
}

pub fn provide_game_state(cx: Scope, id: Signal<Option<PlayerId>>) {
    provide_sse_signal::<ClientGameState>(cx, id);
}

#[cfg(not(feature = "ssr"))]
use gloo_net::eventsource::futures::*;

// whenever the player id changes, resubscribe to a new stream
#[cfg(not(feature = "ssr"))]
pub fn provide_sse_signal<T: ServerSentEvent + 'static>(cx: Scope, id: Signal<Option<PlayerId>>) {
    let handle = store_value::<Option<ScopeDisposer>>(cx, None);
    let dummy_signal = create_rw_signal(cx, ());
    provide_context::<Signal<()>>(cx, dummy_signal.into());

    // provide a placeholder signal
    provide_context::<SseSignal<T>>(cx, create_rw_signal(cx, Default::default()));

    create_effect(cx, move |_| {
        handle.update_value(|h| {
            if let Some(h) = std::mem::take(h) {
                h.dispose();
            }
        });

        id.with(|id| {
            if let Some(id) = id {
                let (stream, disposer) = cx.run_child_scope(move |cx| subscribe::<T>(cx, id));
                handle.set_value(Some(disposer));
                provide_context::<SseSignal<T>>(cx, to_signal::<T>(cx, stream));
                dummy_signal.set(());
            }
        });
    });
}

#[cfg(not(feature = "ssr"))]
type Event =
    Result<(std::string::String, web_sys::MessageEvent), gloo_net::eventsource::EventSourceError>;

#[cfg(not(feature = "ssr"))]
fn parse_event<T: ServerSentEvent>(event: Event) -> Option<T> {
    event
        .ok()?
        .1
        .data()
        .as_string()
        .and_then(|x| serde_json::from_str::<T>(&x).ok())
}

#[cfg(not(feature = "ssr"))]
fn to_signal<T: ServerSentEvent>(cx: Scope, stream: EventSourceSubscription) -> SseSignal<T> {
    create_rw_signal_from_stream::<T>(
        cx,
        stream.filter_map(|e| std::future::ready(parse_event(e))),
    )
}

#[cfg(not(feature = "ssr"))]
use futures::{Stream, StreamExt};

// based on create_signal_from_stream:
// https://docs.rs/leptos_reactive/0.2.1/src/leptos_reactive/signal.rs.html#383-389
#[cfg(not(feature = "ssr"))]
pub fn create_rw_signal_from_stream<T: Default>(
    cx: Scope,
    mut stream: impl Stream<Item = T> + Unpin + 'static,
) -> RwSignal<T> {
    let signal = create_rw_signal(cx, Default::default());
    spawn_local(async move {
        while let Some(value) = stream.next().await {
            log!("sse");
            signal.set(value);
        }
    });
    signal
}

#[cfg(not(feature = "ssr"))]
fn subscribe<T: ServerSentEvent>(cx: Scope, id: &PlayerId) -> EventSourceSubscription {
    let mut source = EventSource::new(&format!("/api/events/{}", id))
        .expect("couldn't connect to SSE server endpoint");
    let stream = source
        .subscribe(<T as ServerSentEvent>::event_type())
        .expect("couldn't subscribe to events");
    on_cleanup(cx, move || {
        source.close();
    });
    stream
}
// stub definition for SSR context
#[cfg(feature = "ssr")]
pub fn provide_sse_signal<T: ServerSentEvent + 'static>(cx: Scope, _id: Signal<Option<PlayerId>>) {
    provide_context::<SseSignal<T>>(cx, create_rw_signal(cx, Default::default()));
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
