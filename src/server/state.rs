use crate::constants::*;
use crate::server::sync::GLOBAL;
use crate::types::*;
use ::std::collections::*;
use super::letter_bag::*;
use ::tokio::{sync::broadcast::Sender, task::spawn, time::*};

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
            state.players.remove(&id);
            _ = sender.send(ServerMessage::GameState(state.to_client_state()))
        }
        ClientMessage::ResetState => {
            *state = default_game_state();
            _ = sender.send(ServerMessage::GameState(state.to_client_state()));
        }

        ClientMessage::StartRound => {
            state.rounds.push(Round {
                judge: state.next_judge(),
                acronym: random_initialism(3),
                winner: None,
                submissions: HashMap::new(),
            });

            state.step = GameStep::Submission;
            let now = Instant::now();
            state.timer_started_at = Some(now);

            _ = sender.send(ServerMessage::GameState(state.to_client_state()));
            // spawn a thread to timeout the submission step
            let sender = sender.clone();
            spawn(async move {
                sleep_until(now + ROUND_TIMER_DURATION).await;
                let mut state = GLOBAL.state.lock().await;
                // if its still the submission step, then end the step.
                if state.step == GameStep::Submission {
                    state.step = GameStep::Judging;
                    _ = sender.send(ServerMessage::GameState(state.to_client_state()));
                }
            });
        }

        ClientMessage::SubmitAcronym(player_id, submission) => {
            if state.players.get(&player_id).is_none() {
                return;
            }

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
