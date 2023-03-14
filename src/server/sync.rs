use crate::types::*;
use ::leptos::*;
use ::std::cell::RefCell;
use ::tokio::sync::{broadcast, mpsc, Mutex};

pub struct Global {
    mailbox_sender: mpsc::Sender<ClientMessage>,
    mailbox_receiver: Mutex<mpsc::Receiver<ClientMessage>>,
    broadcast_sender: broadcast::Sender<ServerMessage>,
}

lazy_static::lazy_static! {
    pub static ref GLOBAL: Global = {
        let (mailbox_sender, mailbox_receiver) = mpsc::channel(100);
        let (broadcast_sender, _) = broadcast::channel(100);
        Global {
            mailbox_sender,
            mailbox_receiver: Mutex::new(mailbox_receiver),
            broadcast_sender,
        }
    };
}

/// Have to wrap thread local statics in this type
type C<T> = RefCell<Option<T>>;

thread_local! {
    // each client's websocket connection is handled by a dedicated tokio thread
    // each server thread has its own inbox & outbox to communicate
    // with the single state management thread
    pub static BROADCASTED: C<broadcast::Receiver<ServerMessage>> = RefCell::new(Some(GLOBAL.broadcast_sender.subscribe()));
    pub static MAILER: C<mpsc::Sender<ClientMessage>> = RefCell::new(Some(GLOBAL.mailbox_sender.clone()));
}

pub async fn send(message: ClientMessage) {
    let handle = MAILER.take().expect("MAILER");

    let m = handle.send(message).await;
    if let Err(e) = m {
        dbg!(e);
    }

    MAILER.set(Some(handle));
}

pub async fn receive<T, F1, F2>(on_ok: F1, on_err: F2)
where
    F1: FnOnce(ServerMessage),
    F2: FnOnce(broadcast::error::RecvError),
{
    let mut handle = BROADCASTED.take().expect("BROADCASTED");

    let result = match handle.recv().await {
        Ok(m) => on_ok(m),
        Err(e) => on_err(e),
    };

    BROADCASTED.set(Some(handle));

    result
}

pub fn spawn_state_thread() {
    tokio::spawn(async move {
        let mut receiver = GLOBAL.mailbox_receiver.lock().await;
        let sender = GLOBAL.broadcast_sender.clone();
        let mut state = GameState::default();

        while let Some(message) = receiver.recv().await {
            match message {
                ClientMessage::JoinGame(player) => {
                    let id = player.id.clone();
                    if let None = state.players.insert(player.id.clone(), player) {
                        state.rotation.push(id);
                    }

                    _ = sender.send(ServerMessage::Demo(state.players.len()));
                }
            }
        }

        log!("mailbox channel closed");
    });
}
