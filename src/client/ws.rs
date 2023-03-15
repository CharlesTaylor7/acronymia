use crate::extensions::ResultExt;
use crate::typed_context::*;
use crate::types::{ServerMessage::*, *};
use futures::{stream::SplitSink, SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use leptos::*;

define_context!(WS_GameState, RwSignal<ClientGameState>);
define_context!(
    WS_Writer,
    StoredValue<Option<SplitSink<WebSocket, Message>>>
);

pub fn connect_to_server(cx: Scope) {
    // TODO: put websocket url & port into ENV
    let (writer, mut reader) = WebSocket::open("ws://localhost:3000/ws").unwrap().split();
    log!("connected");

    let signal = create_rw_signal(cx, Default::default());
    provide_typed_context::<WS_GameState>(cx, signal);
    provide_typed_context::<WS_Writer>(cx, store_value(cx, Some(writer)));

    spawn_local(async move {
        while let Some(msg) = reader.next().await {
            if let Some(Message::Text(m)) = msg.ok_or_log() {
                if let Some(m) = serde_json::from_str(&m).ok_or_log() {
                    signal.update(|g| apply_server_message(g, m));
                }
            }
        }
        log!("disconnected");
    });
}

pub async fn send(cx: Scope, message: ClientMessage) {
    // do a dance to take ownership of the websocket connection's writer
    let mut ws_writer = None;
    let stored_writer = use_typed_context::<WS_Writer>(cx);
    stored_writer.update_value(|v| {
        ws_writer = v.take();
    });

    if let Some(mut ws_writer) = ws_writer {
        ws_writer.send(serialize(&message)).await.ok_or_log();

        // Put that thing back where it came from (or so help me)
        // Ensures the web socket writer can be reused later.
        stored_writer.set_value(Some(ws_writer));
    } else {
        // TODO: should we buffer writes somewhere on the client side?
        log!("busy, message dropped: {:#?}", &message);
    }
}

fn serialize(message: &ClientMessage) -> Message {
    Message::Text(serde_json::to_string(message).expect("ClientMessage serialization failed"))
}

pub fn game_state(cx: Scope) -> RwSignal<ClientGameState> {
    use_typed_context::<WS_GameState>(cx)
}

fn apply_server_message(state: &mut ClientGameState, message: ServerMessage) {
    match message {
        GameState(g) => {
            // replace the current game state completely
            *state = g;
        }

        PlayerJoined(new) => {
            if let Some(mut p) = state.players.iter_mut().find(|p| p.id == new.id) {
                p.name = new.name;
            } else {
                state.players.push(new);
            }
        }
    }
}
