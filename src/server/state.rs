use super::types::*;
use crate::constants::*;
use crate::server::sync;
use ::leptos::logging::log;
use ::std::collections::*;
use ::tokio::{
    select,
    sync::{broadcast::Sender, oneshot},
    task::spawn,
    time::{sleep_until, Instant},
};

// TODO: client actions need to be both restricted by game step & player role
pub async fn handle_message(
    session_id: SessionId,
    message: ClientMessage,
    state: &mut GameState,
    sessions: &mut Sessions,
    messenger: &Sender<ServerMessage>,
) {
    log!("session {:#?}", session_id);
    match message {
        ClientMessage::Connect(player_id) => match sessions.connect(session_id, player_id) {
            Ok(()) => {
                _ = messenger.send(ServerMessage::GameState(state.to_client_state()));
            }
            Err(session_id) => {
                _ = messenger.send(ServerMessage::DuplicateSession(session_id));
            }
        },

        ClientMessage::Disconnect => {
            sessions.remove(&session_id);
        }

        // register your name for the current game
        // allows you to update your name if you already joined
        ClientMessage::JoinGame { name } => {
            if state.step != GameStep::Setup {
                return;
            }

            let id = sessions.player_id(&session_id);
            if id.is_none() {
                return;
            }
            let id = id.unwrap().clone();
            let server_player = ServerPlayer {
                id: id.clone(),
                name: name.clone(),
                quit: false,
            };
            if state.players.insert(id.clone(), server_player).is_none() {
                state.rotation.push(id.clone());
            }

            _ = messenger.send(ServerMessage::PlayerJoined(Player { id, name }));
        }

        ClientMessage::KickPlayer(id) => {
            if let Some(player) = state.players.get_mut(&id) {
                player.quit = true;
                _ = messenger.send(ServerMessage::GameState(state.to_client_state()));
            }
        }

        ClientMessage::StartGame(config) => {
            if state.step != GameStep::Setup {
                return;
            }
            state.config = config;
            start_submission_step(state, messenger);
        }

        ClientMessage::SubmitAcronym(submission) => {
            // can't submit after step ends
            if state.step != GameStep::Submission {
                return;
            }

            let id = sessions.player_id(&session_id);
            if id.is_none() {
                return;
            }
            let id = id.unwrap().clone();

            // have to be in the game to submit
            if state.players.get(&id).is_none() {
                return;
            }

            if let Some(round) = state.rounds.last_mut() {
                let prev = round.submissions.insert(id, submission);

                // if all submissions are in, go to judging step
                if round.submissions.len() + 1 == state.rotation.len() {
                    start_judging_step(state, messenger);
                } else if prev.is_none() {
                    _ = messenger.send(ServerMessage::IncrementSubmissionCount);
                }
            }
        }

        ClientMessage::JudgeRound(winner_id) => {
            if state.step != GameStep::Judging {
                return;
            }

            if let Some(round) = state.rounds.last_mut() {
                round.winner = Some(winner_id.clone());
            } else {
                // prevent double submission
                return;
            }

            _ = messenger.send(ServerMessage::ShowRoundWinner(winner_id));
            set_timer(
                TimerTag::ShowRoundWinner,
                state,
                messenger,
                end_judging_step,
            );
        }

        ClientMessage::GetRemainingTime => {
            _ = messenger.send(ServerMessage::UpdateRemainingTime(
                state.timer.remaining_secs(),
            ));
        }

        // BEGIN DEBUG MESSAGES
        ClientMessage::ResetState => {
            *state = game_state_init();
            _ = messenger.send(ServerMessage::GameState(state.to_client_state()));
        }

        ClientMessage::StopTimer => {
            state.timer.cancel();
        } // END DEBUG MESSAGES
    }
}

fn start_submission_step(state: &mut GameState, messenger: &Sender<ServerMessage>) {
    state.cancel_timer();
    state.rounds.push(Round {
        judge: state.next_judge(),
        winner: None,
        submissions: HashMap::new(),
        prompt: state.next_prompt(),
    });

    state.step = GameStep::Submission;
    set_timer(TimerTag::Submission, state, messenger, start_judging_step);
    _ = messenger.send(ServerMessage::GameState(state.to_client_state()));
}

fn start_judging_step(state: &mut GameState, messenger: &Sender<ServerMessage>) {
    state.cancel_timer();
    state.step = GameStep::Judging;
    state.shuffle_current_round_submissions();

    set_timer(TimerTag::Judging, state, messenger, end_judging_step);
    _ = messenger.send(ServerMessage::GameState(state.to_client_state()));
}

fn end_judging_step(state: &mut GameState, messenger: &Sender<ServerMessage>) {
    let game_length = if DEV_MODE {
        3
    } else {
        // everyone goes twice as judge
        2 * state.rotation.len()
    };

    // game end
    if state.rounds.len() == game_length {
        state.step = GameStep::Results;
        _ = messenger.send(ServerMessage::GameState(state.to_client_state()));
    // next round
    } else {
        start_submission_step(state, messenger);
    }
}

fn set_timer(
    tag: TimerTag,
    state: &mut GameState,
    messenger: &Sender<ServerMessage>,
    on_timeout: impl FnOnce(&mut GameState, &Sender<ServerMessage>) + 'static + Send,
) {
    state.timer.cancel();
    let (cancel, cancelled) = oneshot::channel();
    let now = Instant::now();
    state.timer = Timer::new(now, cancel, tag.clone());

    let messenger = messenger.clone();
    spawn(async move {
        let sleep_then_lock_state = async move {
            sleep_until(now + Timer::duration(&tag)).await;
            sync::state().lock().await
        };

        select! {
            biased;
            mut state = sleep_then_lock_state => on_timeout(&mut state, &messenger),
            // do nothing if cancelled
            _ = cancelled => { },
        }
    });
}
