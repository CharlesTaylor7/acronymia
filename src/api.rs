use crate::types::*;
use leptos::*;

#[cfg(feature = "ssr")]
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
    _ = ResetState::register();
}

// Apis
/// get the players in the game
#[server(FetchPlayers, "/api")]
pub async fn fetch_players() -> Result<Vec<Player>, ServerFnError> {
    let state = STATE.lock().expect("locking thread crashed");

    Result::Ok(state.players.clone())
}

/// get the current game state
#[server(FetchGameStep, "/api")]
pub async fn fetch_game_step() -> Result<GameStep, ServerFnError> {
    let state = STATE.lock().expect("locking thread crashed");

    Result::Ok(state.step.clone())
}

/// register for the game the current game state
#[server(JoinGame, "/api")]
pub async fn join_game(name: String) -> Result<ApiResult<()>, ServerFnError> {
    let mut state = STATE.lock().expect("locking thread crashed");

    if state.players.iter().find(|p| p.name == name).is_some() {
        return api_error("a player with this name has already registered!");
    }
    state.players.push(Player::new(name));

    api_ok(())
}

/// reset the server state completely
#[server(ResetState, "/api")]
pub async fn reset_state() -> Result<(), ServerFnError> {
    let mut state = STATE.lock().expect("locking thread crashed");

    *state = Default::default();
    Result::Ok(())
}

/// nested error types because the outer ServerFnError results in thrown exception
pub type ApiResult<T> = Result<T, String>;

fn api_ok<T>(item: T) -> Result<ApiResult<T>, ServerFnError> {
    Result::Ok(Result::Ok(item))
}

fn api_error<T, M>(message: M) -> Result<ApiResult<T>, ServerFnError>
where
    M: Into<String>,
{
    Result::Ok(Result::Err(message.into()))
}
