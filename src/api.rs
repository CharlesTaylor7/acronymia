use leptos::*;

#[cfg(feature = "ssr")]
use crate::types::*;

#[cfg(feature = "ssr")]
use std::sync::*;

#[cfg(feature = "ssr")]
use std::collections::*;

#[cfg(feature = "ssr")]
lazy_static::lazy_static! {
    pub static ref STATE: Arc<Mutex<GameState>> = Arc::new(Mutex::new(game_state_default()));
}

#[cfg(feature = "ssr")]
fn game_state_default() -> GameState {
    if cfg!(debug_assertions) {
        demo_init(vec!["alice", "bob", "carl"])
    } else {
        Default::default()
    }
}
#[cfg(feature = "ssr")]
fn demo_init(players: Vec<&str>) -> GameState {
    use std::time::Instant;

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
        submissions: HashMap::new(),
        step: GameStep::Setup,
        round_started_at: None,
    }
}

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    _ = JoinGame::register();
    _ = KickPlayer::register();
    _ = StartGame::register();
    _ = ResetState::register();
}

// Apis

/// register your name for the current game
/// allows you to update your name if you already joined
#[server(JoinGame, "/api")]
pub async fn join_game(id: String, name: String) -> Result<(), ServerFnError> {
    log!("id={} name={}", &id, &name);
    let mut state = STATE.lock().expect("locking thread crashed");

    let player = Player {
        id: id.clone(),
        name: name,
    };
    if let None = state.players.insert(id.clone(), player) {
        log!("new player joined: {}", id.clone());
        state.rotation.push(id);
    }

    Ok(())
}

/// kick a player from the current game
/// TODO: restrict this to the room "owner"
#[server(KickPlayer, "/api")]
pub async fn kick_player(id: String) -> Result<(), ServerFnError> {
    log!("kicking {}", &id);
    let mut state = STATE.lock().expect("locking thread crashed");

    state.rotation.retain(|p| *p != id);
    Ok(())
}

/// start the game
/// TODO: restrict this to the room "owner"
#[server(StartGame, "/api")]
pub async fn start_game() -> Result<(), ServerFnError> {
    log!("starting game");
    let mut state = STATE.lock().expect("locking thread crashed");

    state.start_round();
    Ok(())
}

/// reset the game state to default
#[server(ResetState, "/api")]
pub async fn reset_state() -> Result<(), ServerFnError> {
    let mut state = STATE.lock().expect("locking thread crashed");

    *state = game_state_default();
    Result::Ok(())
}

// sse payloads
#[cfg(feature = "ssr")]
pub fn client_game_state(id: String) -> ClientGameState {
    use std::time::Duration;

    let state = STATE.lock().expect("locking thread crashed");

    let judge_id = state.rotation.get(state.current_judge());
    let judge = match judge_id {
        None => Judge::Someone("".to_owned()),
        Some(judge_id) if id == *judge_id => Judge::Me(JudgeInfo {}),
        Some(judge_id) => Judge::Someone(judge_id.clone()),
    };

    // 30 second timer for everyone
    const ALLOTTED: Duration = Duration::new(30, 0);
    let round_timer = state.round_started_at.map(|instant| {
        let elapsed = instant.elapsed();

        if elapsed < ALLOTTED {
            let diff = ALLOTTED - elapsed;
            let rounded_sec = if diff.subsec_nanos() >= 500_000_000 {
                1
            } else {
                0
            };
            diff.as_secs() + rounded_sec
        } else {
            0
        }
    });

    ClientGameState {
        round_timer: round_timer,
        judge: judge,
        step: state.step.clone(),
        players: state
            .rotation
            .iter()
            .map(|id| state.players.get(id))
            .flatten()
            .cloned()
            .collect(),
    }
}
