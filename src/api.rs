use crate::types::*;
use leptos::*;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    _ = FetchPlayers::register();
    _ = FetchGameStep::register();
}

// Apis
/// get the players in the game
#[server(FetchPlayers, "/api")]
pub async fn fetch_players(cx: Scope, room_code: String) -> Result<Vec<Player>, ServerFnError> {
    dbg!(use_context::<u32>(cx));
    dbg!(use_context::<actix_web::HttpRequest>(cx));

    // pretend we're fetching people
    Result::Ok(vec![
        Player {
            id: 0,
            name: "karl".to_string(),
        },
        Player {
            id: 1,
            name: "marx".to_string(),
        },
    ])
}

/// get the current game state
#[server(FetchGameStep, "/api")]
pub async fn fetch_game_step(room_code: String) -> Result<GameStep, ServerFnError> {
    // pretend we're fetching game state
    Result::Ok(GameStep::Setup)
}
