use super::types::*;
use ::leptos::*;
use ::std::sync::OnceLock;
use ::tokio::sync::{broadcast, mpsc, Mutex};

pub struct Global<'a> {
    mailbox_sender: mpsc::Sender<ClientMessage>,
    broadcast_sender: broadcast::Sender<ServerMessage<'a>>,
    state: Mutex<GameState>,
}

pub static GLOBAL: OnceLock<Global> = OnceLock::new();

/// This is the read-side of a channel which receives messages from the state thread.
/// i.e. it "subscribes" to server updates messages.
///
/// # Panics
/// Panics if `spawn_state_thread` has not been run yet.  
pub fn subscribe() -> broadcast::Receiver<ServerMessage<'static>> {
    GLOBAL.get().unwrap().broadcast_sender.subscribe()
}

/// This is the write-side of a channel which messages the state thread.
/// i.e. it "mails" the server with messages.
///
/// # Panics
/// Panics if `spawn_state_thread` has not been run yet.  
pub fn mailer() -> mpsc::Sender<ClientMessage> {
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
            state: (Mutex::new(init_game_state())),
        });

        while let Some(message) = receiver.recv().await {
            let mut state = state().lock().await;
            handle_message(message, &mut state, &sender).await;
        }

        log!("state thread closed");
    });
}
