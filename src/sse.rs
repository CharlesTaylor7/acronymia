use crate::typed_context::*;
use crate::types::*;
use ::leptos::*;

define_context!(Dummy, Signal<()>);
define_context!(SSE, RwSignal<ClientGameState>);

pub fn game_state(cx: Scope) -> RwSignal<ClientGameState> {
    // depend on the dummy signal which just tells us
    // when the inner event stream is swapped out
    use_typed_context::<Dummy>(cx).get();

    // read the signal from context
    use_typed_context::<SSE>(cx)
}

#[cfg(not(feature = "ssr"))]
use gloo_net::eventsource::futures::*;

// whenever the player id changes, resubscribe to a new stream
#[cfg(not(feature = "ssr"))]
pub fn provide_game_state(cx: Scope, id: Signal<Option<PlayerId>>) {
    let handle = store_value::<Option<ScopeDisposer>>(cx, None);
    let dummy_signal = create_rw_signal(cx, ());
    provide_typed_context::<Dummy>(cx, dummy_signal.into());

    // provide a placeholder signal
    provide_typed_context::<SSE>(cx, create_rw_signal(cx, Default::default()));

    create_effect(cx, move |_| {
        handle.update_value(|h| {
            if let Some(h) = std::mem::take(h) {
                h.dispose();
            }
        });

        id.with(|id| {
            if let Some(id) = id {
                let (stream, disposer) = cx.run_child_scope(move |cx| subscribe(cx, id));
                handle.set_value(Some(disposer));
                provide_typed_context::<SSE>(cx, to_signal(cx, stream));
                dummy_signal.set(());
            }
        });
    });
}

#[cfg(not(feature = "ssr"))]
type Event =
    Result<(std::string::String, web_sys::MessageEvent), gloo_net::eventsource::EventSourceError>;

#[cfg(not(feature = "ssr"))]
fn parse_event(event: Event) -> Option<ClientGameState> {
    event
        .ok()?
        .1
        .data()
        .as_string()
        .and_then(|x| serde_json::from_str(&x).ok())
}

#[cfg(not(feature = "ssr"))]
fn to_signal(cx: Scope, stream: EventSourceSubscription) -> RwSignal<ClientGameState> {
    create_rw_signal_from_stream(
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
fn subscribe(cx: Scope, id: &PlayerId) -> EventSourceSubscription {
    let mut source = EventSource::new(&format!("/api/events/{}", id))
        .expect("couldn't connect to SSE server endpoint");
    let stream = source
        .subscribe("message")
        .expect("couldn't subscribe to events");
    on_cleanup(cx, move || {
        source.close();
    });
    stream
}

// stub definition for SSR context
#[cfg(feature = "ssr")]
pub fn provide_game_state(cx: Scope, _id: Signal<Option<PlayerId>>) {
    provide_typed_context::<Dummy>(cx, create_rw_signal(cx, ()).into());
    provide_typed_context::<SSE>(cx, create_rw_signal(cx, Default::default()));
}
