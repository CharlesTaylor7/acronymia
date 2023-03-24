use serde::{Deserialize, Serialize};

/// User submitted pick
pub type Submission = Vec<String>;

/// Uuid generated automatically client side
pub type PlayerId = String;

pub type PlayerName = String;

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

/// game state for a single client
/// some of the server game state should be hidden, and some should be transformed for easier consumption
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ClientGameState {
    pub judge: Option<PlayerId>,
    pub step: GameStep,
    pub players: Vec<Player>,
    pub acronym: String,
    pub timer: Option<u64>,
    /// everyone can see the current submission count
    pub submission_count: usize,
    /// Empty vector when not at the judging step.
    /// This technically enables cheating,
    /// if a savvy player were to inspect the network tab &
    /// cross reference with the players vector.
    pub submissions: Vec<(PlayerId, Submission)>,
    /// Empty until the results step.
    pub scores: Vec<(PlayerName, i64)>,
    pub round_winner: Option<PlayerId>,
    pub round_counter: Option<String>,
}

/// message from a client to the server
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    Connected,
    JoinGame(Player),
    KickPlayer(PlayerId),
    StartGame,
    SubmitAcronym(PlayerId, Submission),
    JudgeRound(PlayerId),
    GetRemainingTime,
    /// for debugging only
    ResetState,
    /// for debugging only
    StopTimer,
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
    ShowRoundWinner(PlayerId),
    IncrementSubmissionCount,
    /// Seconds remaining on the clock
    UpdateRemainingTime(Option<u64>),
}
