use super::sync::*;
use crate::extensions::ResultExt;
use crate::types::*;
use ::actix_web::{rt, web, Error, HttpRequest, HttpResponse};
use ::actix_ws::{CloseCode, CloseReason, Message};
use ::derive_more::{Display, Error};
use ::futures::StreamExt as _;
use ::leptos::log;
use ::serde::Deserialize;
use ::std::time::{Duration, Instant};
use ::tokio::{
    pin, select,
    sync::{broadcast::error::RecvError, mpsc},
    time::interval,
};
use ::uuid::Uuid;

#[derive(Debug, Display, Error)]
struct ApplicationError {
    message: &'static str,
}

// Use default implementation for `error_response()` method
impl ::actix_web::ResponseError for ApplicationError {}

/// How often heartbeat pings are sent.
/// Should be half (or less) of the acceptable client timeout.
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout.
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Deserialize)]
pub struct Params {
    player_id: PlayerId,
}

/// Handshake and start websocket handler with heartbeats.
/// Adapted from [Actix example code](https://github.com/actix/examples/blob/25368e6b65120224f845137c9333850968456153/websockets/echo-actorless/src/handler.rs).
pub async fn handle_ws_request(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let query = web::Query::<Params>::from_query(req.query_string());
    match query {
        Ok(query) => {
            let Params { player_id } = query.into_inner();
            let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;
            rt::spawn(handle_connection(player_id, session, msg_stream));
            return Ok(res);
        }
        Err(_) => Err(ApplicationError {
            message: "missing player id",
        }
        .into()),
    }
}

async fn handle_connection(
    player_id: PlayerId,
    mut session: actix_ws::Session,
    mut msg_stream: actix_ws::MessageStream,
) {
    log!("connected");

    let mut server_broadcast = subscribe();
    let mailer = mailer();
    let mut last_heartbeat = Instant::now();
    let mut interval = interval(HEARTBEAT_INTERVAL);

    let session_id = SessionId(Uuid::new_v4().to_string());
    mailer
        .send((session_id.clone(), ClientMessage::Connect(player_id)))
        .await
        .ok_or_log();

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
                handle_server_message(msg, &mut session, &session_id).await,

            // (2) Client websocket
            msg = msg_stream.next() =>
                handle_client_message(session_id.clone(), msg, &mut session, &mut last_heartbeat, &mailer).await,

            // (3) Heartbeat. Sends a ping, or closes the socket.
            _ = tick =>
                handle_heartbeat(&mut session, last_heartbeat).await,
        };

        if let Some(reason) = reason {
            break reason;
        }
    };

    session.close(Some(reason)).await.ok_or_log();
    mailer
        .send((session_id, ClientMessage::Disconnect))
        .await
        .ok_or_log();

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
    session_id: SessionId,
    msg: Option<Result<Message, actix_ws::ProtocolError>>,
    session: &mut actix_ws::Session,
    last_heartbeat: &mut Instant,
    mailer: &mpsc::Sender<(SessionId, ClientMessage)>,
) -> Option<CloseReason> {
    // websocket closed
    if msg.is_none() {
        return Some(CloseCode::Normal.into());
    }

    if let Some(msg) = msg && let Some(msg) = msg.ok_or_log() {
        match msg {
            Message::Text(text) => {
                if let Some(msg) = serde_json::from_str(&text).ok_or_log() {
                    mailer.send((session_id, msg)).await.ok_or_log();
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
    session_id: &SessionId,
) -> Option<CloseReason> {
    if let Some(msg) = msg.ok_or_log() {
        let serialized = serde_json::to_string(&msg).ok_or_log();

        if let ServerMessage::DuplicateSession(id) = msg {
            if id == *session_id {
                return Some(CloseReason {
                    code: CloseCode::Other(0),
                    description: Some(
                        "player cannot open duplicate web socket connections".to_owned(),
                    ),
                });
            }
        }

        if let Some(text) = serialized {
            session.text(text).await.ok_or_log();
        }
    }
    None
}
