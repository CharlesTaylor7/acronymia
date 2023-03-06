use crate::types::*;
use leptos::*;
use std::sync::{Mutex, Arc};
//use std::sync::mpsc::*;


#[cfg(feature = "ssr")]
lazy_static::lazy_static! {
    pub static ref STATE: Arc<Mutex<GameState>> = Arc::new(Mutex::new(Default::default()));
}


#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    _ = FetchPlayers::register();
    _ = FetchGameStep::register();
}

// Apis
/// get the players in the game
#[server(FetchPlayers, "/api")]
pub async fn fetch_players(cx: Scope, room_code: String) -> Result<Vec<Player>, ServerFnError> {
    let http_request = use_context::<actix_web::HttpRequest>(cx);
    dbg!(http_request);
    let state = Arc::clone(&STATE);
    let state = state.lock().expect("locking thread crashed");

    Result::Ok(state.players.clone())
}

/// get the current game state
#[server(FetchGameStep, "/api")]
pub async fn fetch_game_step(room_code: String) -> Result<GameStep, ServerFnError> {
    // pretend we're fetching game state
    Result::Ok(GameStep::Setup)
}
