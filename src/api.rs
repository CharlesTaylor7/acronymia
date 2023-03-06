use crate::types::*;
use leptos::*;
use std::sync::*;

#[cfg(feature = "ssr")]
lazy_static::lazy_static! {
    pub static ref STATE: Arc<Mutex<GameState>> = Arc::new(Mutex::new(Default::default()));
}

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    _ = FetchPlayers::register();
    _ = FetchGameStep::register();
    _ = JoinGame::register();
}

// Apis
/// get the players in the game
#[server(FetchPlayers, "/api")]
pub async fn fetch_players(cx: Scope, _room_code: String) -> Result<Vec<Player>, ServerFnError> {
    let http_request = use_context::<actix_web::HttpRequest>(cx);
    dbg!(http_request);

    let state = STATE.lock().expect("locking thread crashed");

    Result::Ok(state.players.clone())
}

/// get the current game state
#[server(FetchGameStep, "/api")]
pub async fn fetch_game_step(_room_code: String) -> Result<GameStep, ServerFnError> {
    let state = STATE.lock().expect("locking thread crashed");

    Result::Ok(state.step.clone())
}


/// register for the game the current game state
#[server(JoinGame, "/api")]
pub async fn join_game(name: String) -> Result<(), ServerFnError> {
    let mut state = STATE.lock().expect("locking thread crashed");

    let length = state.players.len();
    state.players.push(Player{id: length.try_into().unwrap(), name: name});

    Result::Ok(())
}
