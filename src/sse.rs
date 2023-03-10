use crate::types::*;
use ::leptos::*;
use ::serde::*;

// Trait to ensure server and client use the same event types
pub trait ServerSentEvent: core::fmt::Debug + PartialEq + Clone + Serialize + for<'de> Deserialize<'de> {
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

pub type SseSignal<T> = Memo<Option<T>>;

pub fn game_state(cx: Scope) -> Option<ClientGameState> {
    // depend on the dummy signal which just tells us 
    // when the inner event stream is swapped out
    use_context::<Signal<()>>(cx).map(|s| s.get());
    // access the context value which holds the current event stream signal
    use_context::<SseSignal<ClientGameState>>(cx)?.get()
}

pub fn provide_game_state(cx: Scope, id: Signal<Option<PlayerId>>) {
    provide_sse_signal::<ClientGameState>(cx, id)
}

#[cfg(not(feature = "ssr"))]
use gloo_net::eventsource::futures::*;

// whenever the player id changes, resubscribe to a new stream
#[cfg(not(feature = "ssr"))]
pub fn provide_sse_signal<T: ServerSentEvent + 'static>(
    cx: Scope,
    id: Signal<Option<PlayerId>>,
)
{
    let handle = store_value::<Option<ScopeDisposer>>(cx, None);
    let dummy_signal = create_rw_signal(cx, ());
    provide_context::<Signal<()>>(cx, dummy_signal.into());
    //let stored_signal = store_value::<Option<SseSignal<T>>>(cx, None);
    //let stored_signal = store_value::<Option<SseSignal<T>>>(cx, None);
    //provide_conte

    create_effect(cx, move |_| {
        handle.update_value(|h| {
            std::mem::take(h).map(|h| {
                h.dispose();
            });
        });

        if let Some(id) = id() {
            let (stream, disposer) = cx.run_child_scope(move |cx| subscribe::<T>(cx, id));
            handle.set_value(Some(disposer));
            provide_context::<SseSignal<T>>(cx, to_signal::<T>(cx, stream));
            dummy_signal.set(());
        }
    });

}

// use gloo_net::eventsource::EventSourceError;
// use web_sys::MessageEvent;
// type Event = Result<(std::string::String, MessageEvent), EventSourceError>;
//
#[cfg(not(feature = "ssr"))]
fn to_signal<T: ServerSentEvent>(
    cx: Scope, 
    stream: EventSourceSubscription

) -> Memo<Option<T>> {
    let signal = create_signal_from_stream(cx, stream);
    create_memo(cx, move |_| {
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
    let mut source = EventSource::new(&format!("/api/events/{}", &id))
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
pub fn provide_sse_signal<T: ServerSentEvent + 'static>(
    _cx: Scope,
    _id: Signal<Option<PlayerId>>,
) 
{
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
