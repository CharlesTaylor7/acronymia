#[cfg(feature = "ssr")]
use ::tokio::time::Instant;
use leptos::{Resource, ServerFnError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
#[cfg(feature = "ssr")]
#[derive(Default, Debug)]
pub struct GameState {
    pub step: GameStep,
    pub players: HashMap<PlayerId, Player>, // registered players
    pub rotation: Vec<PlayerId>,            // players in order they will be judge
    pub rounds: Vec<Round>, // list of rounds, records past or present chosen judge and acronym
    pub timer_started_at: Option<Instant>,
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
    pub judge: Option<PlayerId>,
    pub step: GameStep,
    pub players: Vec<Player>,
    pub acronym: String,
    pub timer: Option<u64>,
    pub submission_count: usize,
    // ^ everyone can see the current submission count
    pub submissions: Vec<(PlayerId, Submission)>,
    // ^ empty vector when not at the judging step
    // this technically enables cheating,
    // if a savvy player were to inspect the network tab & cross reference with the players vector
}

/// message from a client to the server
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    /// id of the thread handling a web socket connection with a particular client
    Connected,
    ResetState,
    JoinGame(Player),
    KickPlayer(PlayerId),
    StartGame,
    SubmitAcronym(PlayerId, Submission),
    JudgeRound(PlayerId),
}

/// message from the server broadcast to each client
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    /// Sent when a client connects for the first time.
    /// Theoretically, a client connecting is the only time when we need to send this message.
    /// We could send smaller message payloads for other events and patch game state on the client
    /// side.
    /// That is the plan longer term, but for development speed, for now we're saving time by sending the
    /// whole client game state.
    /// I have implemented the more granular approach for the PlayerJoined payload to demonstrate
    /// its possible.
    GameState(ClientGameState),
    PlayerJoined(Player),
}

#[cfg(feature = "ssr")]
impl GameState {
    pub fn current_judge(&self) -> Option<JudgeId> {
        self.rounds.last().as_ref().map(|r| r.judge)
    }

    pub fn next_judge(&self) -> JudgeId {
        let n = self.rotation.len();
        if let Some(j) = self.current_judge() && n > 0 {
            (j + 1) % self.rotation.len()
        } else {
            0
        }
    }

    pub fn to_client_state(&self) -> ClientGameState {
        use crate::constants::*;
        use crate::random::shuffle;

        let judge = self
            .current_judge()
            .and_then(|j| self.rotation.get(j))
            .cloned();

        let timer = self.timer_started_at.and_then(|instant| {
            let elapsed = instant.elapsed();

            if elapsed < ROUND_TIMER_DURATION {
                let diff = ROUND_TIMER_DURATION - elapsed;
                let rounded_sec = if diff.subsec_nanos() >= 500_000_000 {
                    1
                } else {
                    0
                };
                Some(diff.as_secs() + rounded_sec)
            } else {
                //self.step = GameStep::Judging;
                None
            }
        });

        let empty_vec = Vec::new();
        let submissions = {
            if self.step == GameStep::Judging && let Some(round) = self.rounds.last() {
                let mut subs = round
                    .submissions
                    .iter()
                    .map(|(key, value)| (key.clone(), value.clone()))
                    .collect::<Vec<_>>();
                shuffle(&mut subs);
                subs
            } else {
                empty_vec
            }
        };

        ClientGameState {
            timer,
            judge,
            submissions,
            step: self.step.clone(),
            submission_count: self.rounds.last().map(|r| r.submissions.len()).unwrap_or(0),
            acronym: self
                .rounds
                .last()
                .map(|r| r.acronym.clone())
                .unwrap_or(String::new()),
            players: self
                .rotation
                .iter()
                .map(|id| self.players.get(id))
                .flatten()
                .cloned()
                .collect(),
        }
    }
}

#[cfg(feature = "ssr")]
pub fn default_game_state() -> GameState {
    if cfg!(debug_assertions) {
        demo_init(vec!["alice", "bob", "carl"])
    } else {
        Default::default()
    }
}

#[cfg(feature = "ssr")]
fn demo_init(players: Vec<&str>) -> GameState {
    let players = players
        .into_iter()
        .enumerate()
        .map(|(id, name)| Player {
            id: id.to_string(),
            name: name.to_owned(),
        })
        .collect::<Vec<_>>();
    let rotation = players.iter().map(|p| p.id.clone()).collect::<Vec<_>>();
    let players = players
        .into_iter()
        .map(|p| (p.id.clone(), p))
        .collect::<HashMap<_, _>>();
    GameState {
        players: players,
        rotation: rotation,
        rounds: Vec::new(),
        step: GameStep::Setup,
        timer_started_at: None,
    }
}
