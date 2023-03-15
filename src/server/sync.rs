use crate::types::*;
use ::leptos::*;
use ::std::sync::Arc;
use ::tokio::sync::{broadcast, mpsc, Mutex};

pub struct Global {
    mailbox_sender: mpsc::Sender<ClientMessage>,
    mailbox_receiver: Mutex<mpsc::Receiver<ClientMessage>>,
    broadcast_sender: broadcast::Sender<ServerMessage>,

    /// This is exposed publicly so that the state thread
    /// can implement timers.
    /// Websocket connection threads should NOT use this directly.
    /// The channels are `subscribe` and `mailer` methods are what you want.
    pub state: Arc<Mutex<GameState>>,
}

lazy_static::lazy_static! {
    pub static ref GLOBAL: Global = {
        let (mailbox_sender, mailbox_receiver) = mpsc::channel(100);
        let (broadcast_sender, _) = broadcast::channel(100);
        Global {
            mailbox_sender,
            mailbox_receiver: Mutex::new(mailbox_receiver),
            broadcast_sender,
            state: Arc::new(Mutex::new(default_game_state())),
        }
    };
}

/// This is the read-side of a channel which receives messages from the state thread.
/// i.e. it "subscribes" to server updates messages.
pub fn subscribe() -> broadcast::Receiver<ServerMessage> {
    GLOBAL.broadcast_sender.subscribe()
}

/// This is the write-side of a channel which messages the state thread.
/// i.e. it "mails" the server with messages.
pub fn mailer() -> mpsc::Sender<ClientMessage> {
    GLOBAL.mailbox_sender.clone()
}

/// You should only call this once at the top level of the app.
/// Manages the application state, with message passing infrastructure.
/// To send to messages to this thread call `mailer`.
/// To receive (broadcast) messges from this thread, call `subscribe`.
pub fn spawn_state_thread() {
    use super::state::*;

    tokio::spawn(async move {
        let mut receiver = GLOBAL.mailbox_receiver.lock().await;
        let sender = GLOBAL.broadcast_sender.clone();

        while let Some(message) = receiver.recv().await {
            let mut state = GLOBAL.state.lock().await;
            handle_message(message, &mut state, &sender).await;
        }

        log!("state thread closed");
    });
}
