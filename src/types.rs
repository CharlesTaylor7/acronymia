use leptos::{Resource, ServerFnError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

// aliases
pub type Server<T> = Result<T, ServerFnError>;
pub type Res<T> = Resource<u32, T>;
pub type Submission = Vec<String>; // user submitted pick
pub type RoundId = usize; // index into the rounds vector
pub type PlayerId = String; // uuid
pub type PlayerName = String;
pub type JudgeId = usize; // index into the rotation vector

/// Server game state
/// The idea is to make the state very normalized.
/// e.g. Determining who the current judge is can be a function, that looks at the last item of the rounds vector. (instead of another field for meta data like that)
#[derive(Default, Debug)]
pub struct GameState {
    pub step: GameStep,
    pub players: HashMap<PlayerId, Player>, // registered players
    pub rotation: Vec<PlayerId>,            // players in order they will be judge
    pub rounds: Vec<Round>, // list of rounds, records past or present chosen judge and acronym
    pub round_started_at: Option<Instant>,
}
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub enum GameStep {
    #[default]
    Setup, // Player's joining and game config
    Submission, // Player's submit acronyms
    Judging,    // Judge judges
    Results,    // Scoreboard at game end
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Round {
    pub judge: JudgeId,
    pub acronym: String,
    pub winner: Option<PlayerId>,
    pub submissions: HashMap<PlayerId, Submission>,
}

/// game state for a single client
/// some of the server game state should be hidden, and some should be transformed for easier consumption
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ClientGameState {
    pub judge: Judge,
    pub step: GameStep,
    pub players: Vec<Player>,
    pub acronym: String,
    pub round_timer: Option<u64>,
    // everyone can see the current submission count
    pub submission_count: usize,
    // empty vector when not at the judging step
    pub submissions: Vec<(PlayerName, Submission)>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Judge {
    Me,
    Name(String),
}
impl Default for Judge {
    fn default() -> Judge {
        Judge::Name(Default::default())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct JudgeInfo {
    // info privy to me as the judge
}

impl GameState {
    pub fn start_round(&mut self) {
        self.rounds.push(Round {
            judge: self.next_judge(),
            acronym: "fart".to_string(),
            winner: None,
            submissions: HashMap::new(),
        });

        self.step = GameStep::Submission;
        self.round_started_at = Some(Instant::now());
    }

    pub fn current_judge(&self) -> JudgeId {
        last(&self.rounds).as_ref().map(|r| r.judge).unwrap_or(0)
    }

    pub fn next_judge(&self) -> JudgeId {
        (self.current_judge() + 1) % self.rotation.len()
    }
}

pub fn last<'a, T>(v: &'a Vec<T>) -> Option<&'a T> {
    if v.len() == 0 {
        return None;
    }
    v.get(v.len() - 1)
}

pub fn last_mut<'a, T>(v: &'a mut Vec<T>) -> Option<&'a mut T> {
    let n = v.len();
    if n == 0 {
        return None;
    }
    v.get_mut(n - 1)
}
