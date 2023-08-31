use super::types::*;
use ::leptos::*;
use ::std::collections::HashMap;
use ::std::sync::OnceLock;
use ::tokio::sync::{broadcast, mpsc, Mutex};

pub struct Global {
    mailbox_sender: mpsc::Sender<(SessionId, ClientMessage)>,
    broadcast_sender: broadcast::Sender<ServerMessage>,
    state: Mutex<GameState>,
}

pub type Sessions = HashMap<SessionId, PlayerId>;

pub static GLOBAL: OnceLock<Global> = OnceLock::new();

/// This is the read-side of a channel which receives messages from the state thread.
/// i.e. it "subscribes" to server updates messages.
///
/// # Panics
/// Panics if `spawn_state_thread` has not been run yet.  
pub fn subscribe() -> broadcast::Receiver<ServerMessage> {
    GLOBAL.get().unwrap().broadcast_sender.subscribe()
}

/// This is the write-side of a channel which messages the state thread.
/// i.e. it "mails" the server with messages.
///
/// # Panics
/// Panics if `spawn_state_thread` has not been run yet.  
pub fn mailer() -> mpsc::Sender<(SessionId, ClientMessage)> {
    GLOBAL.get().unwrap().mailbox_sender.clone()
}

/// # Panics
/// Panics if `spawn_state_thread` has not been run yet.  
pub fn state() -> &'static Mutex<GameState> {
    &GLOBAL.get().unwrap().state
}

/// You should only call this once at the top level of the app.
/// Manages the application state, with message passing infrastructure.
/// To send to messages to this thread call `mailer`.
/// To receive (broadcast) messges from this thread, call `subscribe`.
pub fn spawn_state_thread() {
    use super::state::*;

    tokio::spawn(async move {
        let (mailbox_sender, mut receiver) = mpsc::channel(100);
        let (broadcast_sender, _) = broadcast::channel(100);
        let sender = broadcast_sender.clone();
        _ = GLOBAL.set(Global {
            mailbox_sender,
            broadcast_sender,
            state: Mutex::new(game_state_init()),
        });

        let mut sessions = HashMap::new();

        while let Some((sessionId, message)) = receiver.recv().await {
            let mut state = state().lock().await;
            handle_message(sessionId, message, &mut state, &mut sessions, &sender).await;
        }

        log!("state thread closed");
    });
}
