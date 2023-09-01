use super::types::*;
use ::leptos::*;
use ::std::collections::{hash_map, HashMap};
use ::std::sync::OnceLock;
use ::tokio::sync::{broadcast, mpsc, Mutex};

pub struct Global {
    mailbox_sender: mpsc::Sender<(SessionId, ClientMessage)>,
    broadcast_sender: broadcast::Sender<ServerMessage>,
    state: Mutex<GameState>,
}

#[derive(Debug)]
pub struct Sessions {
    session_ids: HashMap<PlayerId, SessionId>,
    player_ids: HashMap<SessionId, PlayerId>,
}

impl Sessions {
    pub fn new() -> Sessions {
        Sessions {
            session_ids: HashMap::new(),
            player_ids: HashMap::new(),
        }
    }

    pub fn connect(&mut self, session_id: SessionId, player_id: PlayerId) -> Result<(), ()> {
        match self.session_ids.entry(player_id.clone()) {
            hash_map::Entry::Vacant(entry) => {
                leptos::log!("player_id: {:#?}", player_id);
                entry.insert(session_id.clone());
                self.player_ids.insert(session_id, player_id);
                Ok(())
            }
            hash_map::Entry::Occupied(_) => {
                leptos::log!("stopped the hackers!");
                Err(())
            }
        }
    }

    pub fn remove(&mut self, session_id: &SessionId) {
        let player_id = self.player_ids.remove(session_id);
        if let Some(player_id) = player_id {
            self.session_ids.remove(&player_id);
        }
    }
}

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

        let mut sessions = Sessions::new();

        while let Some((session_id, message)) = receiver.recv().await {
            let mut state = state().lock().await;
            handle_message(session_id, message, &mut state, &mut sessions, &sender).await;
        }

        log!("state thread closed");
    });
}
