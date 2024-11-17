use crate::extensions::ResultExt;
use crate::typed_context::*;
use crate::types::{ClientGameState, ClientMessage, PlayerId, ServerMessage, TimerTag};
use ::futures::{stream::SplitSink, SinkExt, StreamExt};
use ::gloo_net::websocket::{futures::WebSocket, Message};
use ::gloo_timers::future::sleep;
use ::leptos::logging::log;
use ::leptos::prelude::*;
use ::leptos::StoredValue;
use ::std::time::Duration;

define_context!(WS_Writer, RwSignal<Option<SplitSink<WebSocket, Message>>>);

///
/// # Panics
/// - May panic if the page url cannot be obtained from the browser
pub fn connect_to_server(game_state: RwSignal<ClientGameState>, player_id: RwSignal<PlayerId>) {
    let loc = leptos::window().location();
    let host = loc.host().unwrap();
    let protocol = loc.protocol().unwrap();
    let protocol = if protocol == "https:" { "wss:" } else { "ws:" };
    let uri = format!("{protocol}//{host}/ws");

    let signal_ws_writer = todo!("fixme: 0.7.0");
    //RwSignal::new(None);
    provide_typed_context::<WS_Writer>(signal_ws_writer);

    leptos::spawn_local(async move {
        let mut backoff = 1;
        loop {
            let (writer, mut reader) = WebSocket::open(&uri).unwrap().split();
            // FIXME: signal_ws_writer.set(Some(writer));

            while let Some(msg) = reader.next().await {
                if let Some(Message::Text(m)) = msg.ok_or_log() {
                    if let Some(m) = serde_json::from_str(&m).ok_or_log() {
                        game_state.update(|g| apply_server_message(g, m));
                    }
                }
            }
            signal_ws_writer.set(None);
            log!("disconnected");
            sleep(Duration::new(backoff, 0)).await;
            backoff *= 2;
        }
    });

    Effect::new(move |_| {
        signal_ws_writer.with(|ws_writer| {
            if ws_writer.is_some() {
                let id = player_id();
                log!("connect as {}", id);
                leptos::spawn_local(async move {
                    send(signal_ws_writer, ClientMessage::Connect(id)).await;
                });
            }
        });
    });
}

pub fn take<T>(stored: StoredValue<Option<T>>) -> Option<T> {
    let mut val = None;
    stored.update_value(|v| val = v.take());
    val
}

pub fn take_untracked<T>(signal: RwSignal<Option<T>>) -> Option<T> {
    let mut val = None;
    // FIXME: signal.update_untracked(|v| val = v.take());
    val
}

pub async fn send_from(owner: Owner, message: ClientMessage) {
    let signal_ws_writer = use_typed_context_from::<WS_Writer>(owner);
    send(signal_ws_writer, message).await;
}

pub async fn send(signal_ws_writer: context_type!(WS_Writer), message: ClientMessage) {
    let ws_writer = take_untracked(signal_ws_writer);

    if let Some(mut ws_writer) = ws_writer {
        ws_writer.send(serialize(&message)).await.ok_or_log();

        // Put that thing back where it came from (or so help me)
        // Ensures the web socket writer can be reused later.
        // FIXME: signal_ws_writer.set_untracked(Some(ws_writer));
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

        ServerMessage::DuplicateSession(_) => {}
    }
}
