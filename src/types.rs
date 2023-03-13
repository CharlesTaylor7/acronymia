use leptos::{Resource, ServerFnError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

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
    pub submission_count: usize,
    // ^ everyone can see the current submission count
    pub submissions: Vec<(PlayerId, Submission)>,
    // ^ empty vector when not at the judging step
    // this technically enables cheating,
    // if a savvy player were to inspect the network tab & cross reference with the players vector
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Judge {
    Me,
    Name(String),
}
impl Default for Judge {
    fn default() -> Judge {
        Judge::Name(String::new())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct JudgeInfo {
    // info privy to me as the judge
}

/// message from a client to the server
#[derive(Clone, Debug)]
pub enum ClientMessage {
    JoinGame(Player),
}

/// message from the server broadcast to each client
#[derive(Clone, Debug)]
pub enum ServerMessage {
    Demo(usize), // for testing
}

/// 30 second timer for everyone
pub const ROUND_TIMER_DURATION: Duration = Duration::new(30, 0);

impl GameState {
    pub fn start_round(&mut self) {
        self.rounds.push(Round {
            judge: self.next_judge(),
            //acronym: "fart".to_string(),
            acronym: "f".to_string(),
            winner: None,
            submissions: HashMap::new(),
        });

        self.step = GameStep::Submission;
        self.round_started_at = Some(Instant::now());
    }

    pub fn current_judge(&self) -> Option<JudgeId> {
        self.rounds.last().as_ref().map(|r| r.judge)
    }

    pub fn next_judge(&self) -> JudgeId {
        self.current_judge()
            .map_or(0, |j| (j + 1) % self.rotation.len())
    }
}
