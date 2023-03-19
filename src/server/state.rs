use super::letter_bag::*;
use super::types::*;
use crate::constants::*;
use crate::server::sync::GLOBAL;
use ::std::collections::*;
use ::tokio::{
    select,
    sync::{broadcast::Sender, oneshot},
    task::spawn,
    time::{sleep_until, Instant},
};

// TODO: client actions need to be both restricted by game step & player role
pub async fn handle_message(
    message: ClientMessage,
    state: &mut GameState,
    messenger: &Sender<ServerMessage>,
) {
    match message {
        ClientMessage::Connected => {
            _ = messenger.send(ServerMessage::GameState(state.to_client_state()));
        }
        // register your name for the current game
        // allows you to update your name if you already joined
        ClientMessage::JoinGame(player) => {
            let id = player.id.clone();
            if let None = state.players.insert(player.id.clone(), player.clone()) {
                state.rotation.push(id);
            }

            _ = messenger.send(ServerMessage::PlayerJoined(player))
        }
        ClientMessage::KickPlayer(id) => {
            state.rotation.retain(|p| *p != id);
            state.players.remove(&id);
            _ = messenger.send(ServerMessage::GameState(state.to_client_state()))
        }

        ClientMessage::StartGame => {
            start_submission_step(state, messenger);
        }

        ClientMessage::SubmitAcronym(player_id, submission) => {
            // have to be in the game to submit
            if state.players.get(&player_id).is_none() {
                return;
            }

            // can't submit after step ends
            if state.step != GameStep::Submission {
                return;
            }

            if let Some(round) = state.rounds.last_mut() {
                round.submissions.insert(player_id, submission);

                // if all submissions are in, go to judging step
                if round.submissions.len() + 1 == state.rotation.len() {
                    start_judging_step(state, &messenger);
                }
            }
        }

        ClientMessage::JudgeRound(winner_id) => {
            state.rounds.last_mut().map(|r| {
                r.winner = Some(winner_id);
            });
            end_judging_step(state, messenger);
        }

        ClientMessage::ResetState => {
            *state = default_game_state();
            _ = messenger.send(ServerMessage::GameState(state.to_client_state()));
        }
    }
}

fn start_submission_step(state: &mut GameState, messenger: &Sender<ServerMessage>) {
    state.cancel_timer();
    state.rounds.push(Round {
        judge: state.next_judge(),
        acronym: random_initialism(3),
        winner: None,
        submissions: HashMap::new(),
    });

    state.step = GameStep::Submission;
    set_timer(state, messenger, start_judging_step);
    _ = messenger.send(ServerMessage::GameState(state.to_client_state()));
}

fn start_judging_step(state: &mut GameState, messenger: &Sender<ServerMessage>) {
    state.cancel_timer();
    state.step = GameStep::Judging;
    state.shuffle_current_round_submissions();

    // TODO:
    // I want to show the round winner to everyone on the judging step,
    // and then redirect after a few seconds
    // back to a new submission step
    //
    set_timer(state, messenger, end_judging_step);
    _ = messenger.send(ServerMessage::GameState(state.to_client_state()));
}

fn end_judging_step(state: &mut GameState, messenger: &Sender<ServerMessage>) {
    let game_length = if cfg!(debug_assertions) {
        3
    } else {
        // everyone goes twice as judge
        2 * state.rotation.len()
    };

    if state.rounds.len() == game_length {
        state.cancel_timer();
        state.step = GameStep::Results;
        _ = messenger.send(ServerMessage::GameState(state.to_client_state()));
    } else {
        start_submission_step(state, messenger);
    }
}

fn set_timer(
    state: &mut GameState,
    messenger: &Sender<ServerMessage>,
    on_timeout: impl FnOnce(&mut GameState, &Sender<ServerMessage>) + 'static + Send,
) {
    let (cancel, cancelled) = oneshot::channel();
    let now = Instant::now();
    state.timer = Timer::new(now, cancel);

    let messenger = messenger.clone();
    spawn(async move {
        let sleep_then_lock_state = async move {
            sleep_until(now + timer_duration()).await;
            GLOBAL.state.lock().await
        };

        select! {
            biased;
            mut state = sleep_then_lock_state => on_timeout(&mut state, &messenger),
            // do nothing if cancelled
            _ = cancelled => { },
        }
    });
}
