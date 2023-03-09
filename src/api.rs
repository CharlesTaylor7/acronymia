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
    log!("id={} name={}", &id, &name);
    let mut state = STATE.lock().expect("locking thread crashed");

    let player = Player {
        id: id.clone(),
        name: name,
    };
    if let None = state.players.insert(id.clone(), player) {
        log!("new player joined: {}", id.clone());
        state.rotation.push(id);
    }

    Ok(())
}

/// kick a player from the current game
/// TODO: restrict this to the room "owner"
#[server(KickPlayer, "/api")]
pub async fn kick_player(id: String) -> Result<(), ServerFnError> {
    log!("kicking {}", &id);
    let mut state = STATE.lock().expect("locking thread crashed");

    state.rotation.retain(|p| *p != id);
    Ok(())
}

/// start the game
/// TODO: restrict this to the room "owner"
#[server(StartGame, "/api")]
pub async fn start_game() -> Result<(), ServerFnError> {
    log!("starting game");
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
pub fn client_game_state(id: String) -> ClientGameState {
    let state = STATE.lock().expect("locking thread crashed");

    let judge_id = state.rotation.get(state.current_judge());
    let judge = match judge_id {
        None => Judge::Someone("".to_owned()),
        Some(judge_id) if id == *judge_id => Judge::Me(JudgeInfo {}),
        Some(judge_id) => Judge::Someone(judge_id.clone()),
    };

    ClientGameState {
        judge: judge,
        step: state.step.clone(),
        players: state
            .rotation
            .iter()
            .map(|id| state.players.get(id))
            .flatten()
            .cloned()
            .collect(),
    }
}
