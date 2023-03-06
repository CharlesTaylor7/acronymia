use leptos::{Resource, ServerFnError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub type Server<T> = Result<T, ServerFnError>;


#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: u32,
    pub name: String,
}

// user submitted pick
pub type Submission = Vec<String>;
// maybe change to UUID?
pub type RoundId = u32;
pub type PlayerId = u32;

#[derive(Serialize, Deserialize, Clone)]
pub struct Round {
    id: RoundId,
    judge: PlayerId,
    acronym: String,
}

/// The idea is for a very normalized game state.
/// e.g. Determining who the current judge is can be a function, that looks at the last item of the rounds vector. (instead of another field for meta data like that)
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GameState {
    step: GameStep,
    players: Vec<Player>, // list of registered players
    rounds: Vec<Round>,   // list of rounds, records past or present chosen judge and acronym
    submissions: HashMap<(RoundId, PlayerId), Submission>,
    winners: Vec<PlayerId>, // list of winning player indexed by round
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub enum GameStep {
    #[default]
    Setup,      // Player's joining and game config
    Submission, // Player's submit acronyms
    Judging,    // Judge judges
    Results,    // Scoreboard at game end
}

/// Messages passed to the main thread to update the game state
pub enum Message {
    Dummy,
}

pub type Res<T> = Resource<u32, T>;
