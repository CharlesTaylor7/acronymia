use super::sync::*;
use crate::extensions::ResultExt;
use crate::types::*;
use ::actix_web::{rt, web, Error, HttpRequest, HttpResponse};
use ::actix_ws::{CloseCode, CloseReason, Message};
use ::futures::StreamExt as _;
use ::leptos::log;
use ::std::time::{Duration, Instant};
use ::tokio::{
    pin, select,
    sync::{broadcast::error::RecvError, mpsc},
    time::interval,
};

/// How often heartbeat pings are sent.
/// Should be half (or less) of the acceptable client timeout.
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout.
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Handshake and start websocket handler with heartbeats.
/// Adapted from [Actix example code](https://github.com/actix/examples/blob/25368e6b65120224f845137c9333850968456153/websockets/echo-actorless/src/handler.rs).
pub async fn handle_ws_request(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    rt::spawn(handle_connection(session, msg_stream));

    Ok(res)
}

async fn handle_connection(
    mut session: actix_ws::Session,
    mut msg_stream: actix_ws::MessageStream,
) {
    log!("connected");

    let mut server_broadcast = subscribe();
    let mailer = mailer();
    let mut last_heartbeat = Instant::now();
    let mut interval = interval(HEARTBEAT_INTERVAL);

    // just connected, server will send back the complete state
    mailer.send(ClientMessage::Connected).await.ok_or_log();

    let reason = loop {
        let tick = interval.tick();
        pin!(tick);
        // poll each future, until one returns with a result and cancel the others
        let reason = select! {
            biased;
            // ^ "biased" tells the macro to poll futures in the listed order,
            // instead of randomizing the poll order each loop.
            //
            // The futures are ordered in priority order.
            // The whole system would get bogged down if the server is ready to
            // broadcast but clients keep firing events without listening.
            //
            // The heartbeat is the least important, so it can wait until
            // no other activity is going on.

            // (1) Server broadcast
            msg = server_broadcast.recv() =>
                handle_server_message(msg, &mut session).await,

            // (2) Client websocket
            msg = msg_stream.next() =>
                handle_client_message(msg, &mut session, &mut last_heartbeat, &mailer).await,
            // (3) Heartbeat. Sends a ping, or closes the socket.
            _ = tick =>
                handle_heartbeat(&mut session, last_heartbeat).await,
        };

        if let Some(reason) = reason {
            break reason;
        }
    };

    session.close(Some(reason)).await.ok_or_log();

    log!("disconnected");
}

async fn handle_heartbeat(
    session: &mut actix_ws::Session,
    last_heartbeat: Instant,
) -> Option<CloseReason> {
    // if no heartbeat ping/pong received recently, close the connection
    if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
        let reason = CloseReason {
            code: CloseCode::Normal,
            description: Some("client timed out".to_owned()),
        };
        return Some(reason);
    }

    // send heartbeat ping
    session.ping(b"").await.ok_or_log();
    None
}

async fn handle_client_message(
    msg: Option<Result<Message, actix_ws::ProtocolError>>,
    session: &mut actix_ws::Session,
    last_heartbeat: &mut Instant,
    mailer: &mpsc::Sender<ClientMessage>,
) -> Option<CloseReason> {
    // websocket closed
    if msg.is_none() {
        return Some(CloseCode::Normal.into());
    }

    if let Some(msg) = msg && let Some(msg) = msg.ok_or_log() {
        match msg {
            Message::Text(text) => {
                if let Some(msg) = serde_json::from_str(&text).ok_or_log() {
                    mailer.send(msg).await.ok_or_log();
                }
            },

            Message::Close(reason) => {
                return Some(reason.unwrap_or(CloseCode::Normal.into()));
            }

            Message::Ping(bytes) => {
                *last_heartbeat = Instant::now();
                session.pong(&bytes).await.ok_or_log();
            }

            Message::Pong(_) => {
                *last_heartbeat = Instant::now();
            }

            Message::Binary(_) => {
                log::warn!("no support for binary");
            }

            Message::Continuation(_) => {
                log::warn!("no support for continuation frames");
            }

            // no-op; ignore
            Message::Nop => {}
        };
    }

    None
}

async fn handle_server_message(
    msg: Result<ServerMessage, RecvError>,
    session: &mut actix_ws::Session,
) -> Option<CloseReason> {
    if let Some(msg) = msg.ok_or_log() {
        if let Some(msg) = serde_json::to_string(&msg).ok_or_log() {
            session.text(msg).await.ok_or_log();
        }
    }
    None
}
