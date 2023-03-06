use leptos::{Resource, ServerFnError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum GameState {
    Setup,      // Player's joining and game config
    Submission, // Player's submit acronyms
    Judging,    // Judge judges
    Results,    // Scoreboard at game end
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: u32,
    pub name: String,
}

pub type Res<T> = Resource<u32, T>;
pub type Server<T> = Result<T, ServerFnError>;
