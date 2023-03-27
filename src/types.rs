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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct ClientConfig {}


/// Step specific fields are nested under the `step` field. 
/// Everything else is shared.
pub struct ClientGameState_ {
    pub players: Vec<Player>,
    pub step: ClientGameStep,
}


/// Legacy game state definition
pub struct ClientGameState {
}

/// Each step of the game has distinct state.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ClientGameStep {
    Setup(SetupState),
    Submission(SubmissionState),
    Judging(JudgingState),
    Results(ResultsState),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetupState {
    pub players: Vec<Player>,
    pub config: ClientConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SubmissionState {
    pub timer: u64,
    pub acronym: String,
    pub judge: Player,
    pub round_counter: String,
    pub submission_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JudgingState {
    pub timer: Option<u64>,
    pub acronym: String,
    pub judge: Player,
    pub round_counter: String,
    /// This technically enables cheating,
    /// if a savvy player were to inspect the network tab.
    pub submissions: Vec<(Player, Submission)>,
    pub round_winner: Option<PlayerId>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResultsState {
    pub scores: Vec<(PlayerName, i64)>,
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
