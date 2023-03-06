use crate::types::{GameState, Player};
use leptos::*;

// Apis
/// get the players in the game
#[server(FetchPlayers)]
pub async fn fetch_players(room_code: String) -> Result<Vec<crate::types::Player>, ServerFnError> {
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
#[server(FetchGameState)]
pub async fn fetch_game_state(room_code: String) -> Result<crate::types::GameState, ServerFnError> {
    // pretend we're fetching game state
    Result::Ok(GameState::Setup)
}
