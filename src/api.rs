use leptos::*;

#[cfg(feature = "ssr")]
use crate::types::*;

#[cfg(feature = "ssr")]
use std::sync::*;

#[cfg(feature = "ssr")]
lazy_static::lazy_static! {
    pub static ref STATE: Arc<Mutex<GameState>> = Arc::new(Mutex::new(Default::default()));
}

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    _ = JoinGame::register();
    _ = KickPlayer::register();
    _ = StartGame::register();
    _ = ResetState::register();
}

// Apis

/// register your name for the current game
/// allows you to update your name if you already joined
#[server(JoinGame, "/api")]
pub async fn join_game(id: String, name: String) -> Result<(), ServerFnError> {
    debug_warn!("id={} name={}", &id, &name);
    let mut state = STATE.lock().expect("locking thread crashed");

    let player = Player {
        id: id.clone(),
        name: name,
    };
    if let None = state.players.insert(id.clone(), player) {
        debug_warn!("new player joined: {}", id.clone());
        state.rotation.push(id);
    }

    Ok(())
}

/// kick a player from the current game
/// TODO: restrict this to the room "owner"
#[server(KickPlayer, "/api")]
pub async fn kick_player(id: String) -> Result<(), ServerFnError> {
    debug_warn!("kicking {}", &id);
    let mut state = STATE.lock().expect("locking thread crashed");

    state.rotation.retain(|p| *p != id);
    Ok(())
}

/// start the game
/// TODO: restrict this to the room "owner"
#[server(StartGame, "/api")]
pub async fn start_game() -> Result<(), ServerFnError> {
    debug_warn!("starting game");
    let mut state = STATE.lock().expect("locking thread crashed");

    state.start_round();
    Ok(())
}

/// reset the server state completely
#[server(ResetState, "/api")]
pub async fn reset_state() -> Result<(), ServerFnError> {
    let mut state = STATE.lock().expect("locking thread crashed");

    *state = Default::default();
    Result::Ok(())
}

// sse payloads
#[cfg(feature = "ssr")]
pub fn fetch_players() -> Vec<Player> {
    let state = STATE.lock().expect("locking thread crashed");

    state
        .rotation
        .iter()
        .map(|id| state.players.get(id))
        .flatten()
        .cloned()
        .collect()
}

#[cfg(feature = "ssr")]
pub fn fetch_game_step() -> GameStep {
    let state = STATE.lock().expect("locking thread crashed");

    state.step.clone()
}
