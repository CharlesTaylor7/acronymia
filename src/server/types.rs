use crate::constants::*;
use crate::random::shuffle;
pub use crate::types::*;
use ::leptos::log;
use ::std::collections::HashMap;
use ::tokio::{
    sync::oneshot,
    time::{Duration, Instant},
};

/// Index into the rotation vector
type JudgeId = usize;

/// Server game state
/// The idea is to make the state very normalized.
/// e.g. Determining who the current judge is can be a function, that looks at the last item of the rounds vector. (instead of another field for meta data like that)
#[cfg(feature = "ssr")]
#[derive(Default, Debug)]
pub struct GameState {
    pub step: GameStep,
    /// Player information
    pub players: HashMap<PlayerId, ServerPlayer>,
    /// Player ids in order they will be judge
    pub rotation: Vec<PlayerId>,
    pub rounds: Vec<Round>,
    pub shuffled_submissions: Vec<(PlayerId, Submission)>,
    pub timer: Timer,
}

#[derive(Default, Debug)]
pub struct Round {
    pub judge: JudgeId,
    pub acronym: String,
    pub winner: Option<PlayerId>,
    pub submissions: HashMap<PlayerId, Submission>,
}

#[derive(Debug)]
pub struct ServerPlayer {
    /// If a player needs to leave midgame, we leave them in place to not break the judge rotation
    /// and allow showing their end of game score.
    pub quit: bool,
    pub id: PlayerId,
    pub name: String,
}

#[derive(Debug, Default)]
pub struct Timer(Option<TimerFields>);

#[derive(Debug)]
struct TimerFields {
    started_at: Instant,
    cancellation: oneshot::Sender<()>,
    tag: TimerTag,
}

#[derive(Debug, Clone)]
pub enum TimerTag {
    Submission,
    Judging,
    ShowRoundWinner,
}

impl TimerTag {
    pub fn duration(&self) -> Duration {
        match self {
            TimerTag::Submission => Duration::new(60, 0),
            TimerTag::Judging => Duration::new(30, 0),
            TimerTag::ShowRoundWinner => Duration::new(5, 0),
        }
    }
}

impl Timer {
    pub fn new(started_at: Instant, cancellation: oneshot::Sender<()>, tag: TimerTag) -> Self {
        Self(Some(TimerFields {
            started_at,
            cancellation,
            tag,
        }))
    }

    pub fn elapsed(&self) -> Option<Duration> {
        self.0.as_ref().map(|f| f.started_at.elapsed())
    }

    pub fn remaining_secs(&self) -> Option<u64> {
        self.0.as_ref().and_then(|t| {
            let elapsed = t.started_at.elapsed();
            let duration = t.tag.duration();
            if elapsed < duration {
                let diff = duration - elapsed;
                let rounded_sec = if diff.subsec_nanos() >= 500_000_000 {
                    1
                } else {
                    0
                };
                Some(diff.as_secs() + rounded_sec)
            } else {
                None
            }
        })
    }

    pub fn cancel(&mut self) {
        if let Some(fields) = self.0.take() {
            _ = fields.cancellation.send(());
        }
    }
}

impl GameState {
    pub fn current_judge(&self) -> Option<JudgeId> {
        self.rounds.last().as_ref().map(|r| r.judge)
    }

    pub fn next_judge(&self) -> JudgeId {
        let n = self.rotation.len();
        if let Some(j) = self.current_judge() && n > 0 {
            // scan for next un-quited player
            for offset in 1..n {
                let index = (j + offset) % n;
                if !self.players[&self.rotation[index]].quit {
                    return index;
                }
            }
        }
        0
    }

    pub fn cancel_timer(&mut self) {
        self.timer.cancel();
    }

    pub fn shuffle_current_round_submissions(&mut self) {
        if let Some(round) = self.rounds.last() {
            let mut subs = round
                .submissions
                .iter()
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect::<Vec<_>>();

            shuffle(&mut subs);

            self.shuffled_submissions = subs;
        } else {
            log!("warning: shuffle_current_round_submissions");
        }
    }

    pub fn scores(&self) -> Vec<(PlayerName, i64)> {
        let mut score_map = HashMap::new();
        for round in self.rounds.iter() {
            if let Some(winner) = &round.winner {
                insert_or_add(&mut score_map, winner, 1);
            } else {
                // The judge is penalized for a timeout.
                // This is because any round where you don't select a winner
                // you've denied any of your peers a point.
                // Penalizing the judge fixes this issue from a "game theory" perspective
                insert_or_add(&mut score_map, &self.rotation[round.judge], -1);
            }
        }
        let mut scores = Vec::with_capacity(self.rotation.len());
        for id in self.rotation.iter() {
            let score = score_map.get(id).map_or(0, |s| *s);
            scores.push((self.players[id].name.clone(), score));
        }
        // sort descending
        scores.sort_by(|(_, a_score), (_, b_score)| b_score.cmp(&a_score));

        scores
    }

    pub fn to_client_state(&self) -> ClientGameState {
        let judge = self
            .current_judge()
            .and_then(|j| self.rotation.get(j))
            .cloned();

        let timer = self.timer.remaining_secs();

        let empty_vec = Vec::new();
        let submissions = {
            if self.step == GameStep::Judging {
                self.shuffled_submissions.clone()
            } else {
                empty_vec
            }
        };

        let scores = if self.step == GameStep::Results {
            self.scores()
        } else {
            Vec::new()
        };

        ClientGameState {
            timer,
            judge,
            submissions,
            scores,
            round_winner: self.rounds.last().and_then(|r| r.winner.clone()),
            step: self.step.clone(),
            round_counter: format!("Round {}/{}", self.rounds.len(), 2 * self.rotation.len()),
            submission_count: self.rounds.last().map(|r| r.submissions.len()).unwrap_or(0),
            acronym: self
                .rounds
                .last()
                .map(|r| r.acronym.clone())
                .unwrap_or(String::new()),
            players: self
                .rotation
                .iter()
                .filter_map(|id| {
                    self.players.get(id).and_then(|p| {
                        if p.quit {
                            None
                        } else {
                            Some(Player {
                                id: p.id.clone(),
                                name: p.name.clone(),
                            })
                        }
                    })
                })
                .collect(),
        }
    }
}

fn insert_or_add<K>(map: &mut HashMap<K, i64>, key: K, amount: i64)
where
    K: std::hash::Hash + Eq,
{
    if let Err(mut occupied) = map.try_insert(key, amount) {
        *occupied.entry.get_mut() += amount;
    }
}

pub fn default_game_state() -> GameState {
    if DEBUG_MODE {
        demo_init(vec!["alice", "bob", "carl"])
    } else {
        Default::default()
    }
}

pub fn demo_init(players: Vec<&str>) -> GameState {
    let players = players
        .into_iter()
        .map(|name| ServerPlayer {
            id: name.to_owned(),
            name: name.to_owned(),
            quit: false,
        })
        .collect::<Vec<_>>();

    let rotation = players.iter().map(|p| p.id.clone()).collect::<Vec<_>>();
    let players = players
        .into_iter()
        .map(|p| (p.id.clone(), p))
        .collect::<HashMap<_, _>>();
    GameState {
        rounds: Vec::with_capacity(2 * &players.len()),
        players,
        rotation,
        step: GameStep::Setup,
        timer: Timer::default(),
        shuffled_submissions: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use crate::server::types::*;
    #[test]
    fn scores() {
        let state = GameState::default();
        let state = GameState {
            rounds: vec![
                Round {
                    winner: Some("a".to_owned()),
                    ..Round::default()
                },
                Round {
                    winner: Some("a".to_owned()),
                    ..Round::default()
                },
                Round {
                    winner: Some("b".to_owned()),
                    ..Round::default()
                },
                Round {
                    winner: None,
                    judge: 2,
                    ..Round::default()
                },
            ],
            ..demo_init(vec!["a", "b", "c"])
        };
        let scores = state.scores();
        assert_eq!(
            state.scores(),
            vec![
                ("a".to_owned(), 2),
                ("b".to_owned(), 1),
                ("c".to_owned(), -1)
            ]
        );
    }
}
