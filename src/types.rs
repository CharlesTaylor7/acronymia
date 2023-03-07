use leptos::{Resource, ServerFnError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// aliases because I'm lazy
pub type Server<T> = Result<T, ServerFnError>;
pub type Res<T> = Resource<u32, T>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
}
impl Player {
    pub fn new(name: String) -> Self {
        Player {
            id: Uuid::new_v4().to_string(),
            name: name,
        }
    }
}

// user submitted pick
pub type Submission = Vec<String>;
pub type RoundId = u32;
// uuid
pub type PlayerId = String;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Round {
    judge: PlayerId,
    acronym: String,
}

/// The idea is for a very normalized game state.
/// e.g. Determining who the current judge is can be a function, that looks at the last item of the rounds vector. (instead of another field for meta data like that)
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct GameState {
    pub step: GameStep,
    pub players: Vec<Player>, // list of registered players
    pub rounds: Vec<Round>,   // list of rounds, records past or present chosen judge and acronym
    pub submissions: HashMap<(RoundId, PlayerId), Submission>,
    pub winners: Vec<PlayerId>, // list of winning player indexed by round
}

impl GameState {
    fn start_round(&mut self) {
        self.rounds.push(Round {
            judge: self.next_judge().id.clone(),
            acronym: "fart".to_string(),
        });

        self.step = GameStep::Submission;
    }

    fn current_judge(&self) -> Option<&Player> {
        let length = self.rounds.len();
        if length == 0 {
            return None;
        }
        let id = &self.rounds[length - 1].judge;
        self.players.iter().find(|p| &p.id == id)
    }

    fn next_judge(&self) -> &Player {
        let judge = self.current_judge();
        let length = self.players.len();
        let default = length - 1;
        let index = match judge {
            None => default,
            Some(judge) => self
                .players
                .iter()
                .position(|x| x.id == judge.id)
                .unwrap_or(default),
        };
        &self.players[(index + 1) % length]
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub enum GameStep {
    #[default]
    Setup, // Player's joining and game config
    Submission, // Player's submit acronyms
    Judging,    // Judge judges
    Results,    // Scoreboard at game end
}

/// Messages passed to the main thread to update the game state
pub enum Message {
    Dummy,
}