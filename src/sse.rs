use crate::types::*;
use ::leptos::*;
use ::serde::*;

// Trait to ensure server and client use the same event types
pub trait ServerSentEvent: PartialEq + Clone + Serialize + for<'de> Deserialize<'de> {
    fn event_type() -> &'static str;
}

impl ServerSentEvent for ClientGameState {
    fn event_type() -> &'static str {
        "game-state"
    }
}

#[cfg(not(feature = "ssr"))]
use gloo_net::eventsource::futures::*;

// use gloo_net::eventsource::EventSourceError;
// use web_sys::MessageEvent;
// type Event = Result<(std::string::String, MessageEvent), EventSourceError>;

// whenever the player id changes, resubscribe to a new stream
#[cfg(not(feature = "ssr"))]
pub fn create_sse_signal<T: ServerSentEvent>(
    cx: Scope,
    id: Signal<Option<PlayerId>>,
) -> Signal<Option<T>> {
    let handle = store_value::<Option<ScopeDisposer>>(cx, None);

    let signal: Signal<Signal<Option<T>>> = Signal::derive(cx, move || {
        log!("run sse effect");
        handle.update_value(|h| {
            std::mem::take(h).map(|h| {
                log!("dispose of child scope");
                h.dispose();
            });
        });

        if let Some(id) = id() {
            log!("id = {}", &id);
            let (stream, disposer) = cx.run_child_scope(move |cx| subscribe::<T>(cx, id));
            handle.set_value(Some(disposer));
            return to_signal(cx, stream);
        }
        Signal::derive(cx, || None)
    });

    Signal::derive(cx, move || signal.with(|s| s.get_untracked()))
}

#[cfg(not(feature = "ssr"))]
fn to_signal<T: ServerSentEvent>(cx: Scope, stream: EventSourceSubscription) -> Signal<Option<T>> {
    let signal = create_signal_from_stream(cx, stream);
    Signal::derive(cx, move || {
        signal()?
            .ok()?
            .1
            .data()
            .as_string()
            .and_then(|x| serde_json::from_str::<T>(&x).ok())
    })
}

#[cfg(not(feature = "ssr"))]
fn subscribe<T: ServerSentEvent>(cx: Scope, id: PlayerId) -> EventSourceSubscription {
    log!("subscribing with {}", &id);
    let mut source = EventSource::new(&format!("/api/events/{}", &id))
        .expect("couldn't connect to SSE server endpoint");
    let stream = source
        .subscribe(<T as ServerSentEvent>::event_type())
        .expect("couldn't subscribe to events");
    on_cleanup(cx, move || {
        log!("closing event source");
        source.close();
    });
    stream
}
// stub definition for SSR context
#[cfg(feature = "ssr")]
pub fn create_sse_signal<T: ServerSentEvent>(
    cx: Scope,
    _id: RwSignal<Option<PlayerId>>,
) -> Signal<Option<T>> {
    create_signal(cx, Default::default()).0.into()
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
