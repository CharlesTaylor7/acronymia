use crate::types::*;
use ::tokio::sync::broadcast::Sender;
use std::collections::*;

// TODO: client actions need to be both restricted by game step & player role
pub async fn handle_message(
    message: ClientMessage,
    state: &mut GameState,
    sender: &Sender<ServerMessage>,
) {
    match message {
        ClientMessage::Connected => {
            _ = sender.send(ServerMessage::GameState(state.to_client_state()));
        }
        // register your name for the current game
        // allows you to update your name if you already joined
        ClientMessage::JoinGame(player) => {
            let id = player.id.clone();
            if let None = state.players.insert(player.id.clone(), player.clone()) {
                state.rotation.push(id);
            }

            _ = sender.send(ServerMessage::PlayerJoined(player))
        }
        ClientMessage::KickPlayer(id) => {
            state.rotation.retain(|p| *p != id);
            _ = sender.send(ServerMessage::GameState(state.to_client_state()))
        }
        ClientMessage::ResetState => {
            *state = default_game_state();
            _ = sender.send(ServerMessage::GameState(state.to_client_state()))
        }

        ClientMessage::StartGame => {
            state.start_round();

            _ = sender.send(ServerMessage::GameState(state.to_client_state()))
        }

        ClientMessage::SubmitAcronym(player_id, submission) => {
            if let Some(round) = state.rounds.last_mut() {
                round.submissions.insert(player_id, submission);
                if round.submissions.len() + 1 == state.rotation.len() {
                    state.step = GameStep::Judging;
                }
            }

            _ = sender.send(ServerMessage::GameState(state.to_client_state()))
        }

        ClientMessage::JudgeRound(winner_id) => {
            state.rounds.last_mut().map(|r| {
                r.winner = Some(winner_id);
            });

            _ = sender.send(ServerMessage::GameState(state.to_client_state()))
        }
    }
}

pub fn default_game_state() -> GameState {
    if cfg!(debug_assertions) {
        demo_init(vec!["alice", "bob", "carl"])
    } else {
        Default::default()
    }
}

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
        round_started_at: None,
    }
}
