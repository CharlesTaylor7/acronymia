use crate::extensions::ResultExt;
use crate::typed_context::*;
use crate::types::{ClientGameState, ClientMessage, ServerMessage, TimerTag};
use futures::{stream::SplitSink, SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use leptos::*;

define_context!(
    WS_Writer,
    StoredValue<Option<SplitSink<WebSocket, Message>>>
);

pub fn connect_to_server(game_state: RwSignal<ClientGameState>, player_id: String) {
    let loc = window().location();
    let host = loc.host().unwrap();
    let protocol = loc.protocol().unwrap();
    let protocol = if protocol == "https:" { "wss:" } else { "ws:" };
    let uri = format!("{protocol}//{host}/ws?player_id={player_id}");

    let stored_writer = store_value(None);
    provide_typed_context::<WS_Writer>(stored_writer);

    spawn_local(async move {
        loop {
            let (writer, mut reader) = WebSocket::open(&uri).unwrap().split();
            stored_writer.set_value(Some(writer));
            log!("connected");

            while let Some(msg) = reader.next().await {
                if let Some(Message::Text(m)) = msg.ok_or_log() {
                    if let Some(m) = serde_json::from_str(&m).ok_or_log() {
                        game_state.update(|g| apply_server_message(g, m));
                    }
                }
            }
            log!("disconnected");
        }
    });
}

pub async fn send_from(owner: Owner, message: ClientMessage) {
    // do a dance to take ownership of the websocket connection's writer
    let mut ws_writer = None;
    let stored_writer = use_typed_context_from::<WS_Writer>(owner);
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

fn apply_server_message(state: &mut ClientGameState, message: ServerMessage) {
    match message {
        ServerMessage::GameState(g) => {
            // replace the current game state completely
            *state = g;
        }

        ServerMessage::PlayerJoined(new) => {
            if let Some(p) = state.players.iter_mut().find(|p| p.id == new.id) {
                p.name = new.name;
            } else {
                state.players.push(new);
            }
        }

        ServerMessage::ShowRoundWinner(player_id) => {
            state.round_winner = Some(player_id);
            state.timer = Some(TimerTag::ShowRoundWinner.secs());
        }

        ServerMessage::IncrementSubmissionCount => {
            state.submission_count += 1;
        }

        ServerMessage::UpdateRemainingTime(time) => {
            state.timer = time;
        }

        ServerMessage::Disconnect(_) => {}
    }
}
