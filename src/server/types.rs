mod sessions;
pub use self::sessions::Sessions;
use super::random::shuffle;
use crate::constants::*;
pub use crate::types::*;
use ::leptos::log;
use ::std::collections::{hash_map, HashMap};
use ::tokio::{
    sync::oneshot,
    time::{Duration, Instant},
};
pub use ::uuid::Uuid;

/// Index into the rotation vector
type JudgeId = usize;

/// Server game state
/// The idea is to make the state very normalized.
/// e.g. Determining who the current judge is can be a function, that looks at the last item of the rounds vector. (instead of another field for meta data like that)
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
    pub config: Config,
    pub prompts: Vec<(String, String)>,
}

#[derive(Default, Debug)]
pub struct Round {
    pub judge: JudgeId,
    pub prompt: Prompt,
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

impl Timer {
    pub fn duration(tag: &TimerTag) -> Duration {
        Duration::new(tag.secs(), 0)
    }

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
            let duration = Self::duration(&t.tag);
            if elapsed < duration {
                let diff = duration - elapsed;
                // clippy recommended the From instance for bool to u64
                let rounded_sec = u64::from(diff.subsec_nanos() >= 500_000_000);
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
    pub fn next_prompt(&self) -> Prompt {
        use crate::server::letter_bag::random_initialism;
        let acronym = random_initialism(&self.config.letters_per_acronym);
        let n = self.rounds.len();
        let (before, after) = if let Some(p) = self.prompts.get(n) {
            (p.0.clone(), p.1.clone())
        } else {
            ("What is ".to_owned(), " ?".to_owned())
        };
        Prompt {
            acronym,
            before,
            after,
        }
    }

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
        for round in &self.rounds {
            let points = round.prompt.acronym.len() as i64;
            if let Some(winner) = &round.winner {
                let penalty = submission_penalty(&round.submissions[winner]);
                let points = std::cmp::max(points + penalty, 0);
                insert_or_add(&mut score_map, winner, points);
            } else {
                // The judge is penalized for a timeout.
                // This is because any round where you don't select a winner
                // you've denied all of your peers any points.
                // Penalizing the judge fixes this issue from a "game theory" perspective
                insert_or_add(&mut score_map, &self.rotation[round.judge], -points);
            }
        }
        let mut scores = Vec::with_capacity(self.rotation.len());
        for id in &self.rotation {
            let score = score_map.get(id).map_or(0, |s| *s);
            scores.push((self.players[id].name.clone(), score));
        }
        // sort descending
        scores.sort_by(|(_, a_score), (_, b_score)| b_score.cmp(a_score));

        scores
    }

    pub fn to_client_state(&self) -> ClientGameState {
        let judge = self
            .current_judge()
            .and_then(|j| self.rotation.get(j))
            .cloned();

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

        let round_counter = format!("Round {}/{}", self.rounds.len(), 2 * self.rotation.len());

        ClientGameState {
            judge,
            submissions,
            scores,
            round_counter,
            config: self.config.clone(),
            timer: self.timer.remaining_secs(),
            round_winner: self.rounds.last().and_then(|r| r.winner.clone()),
            step: self.step.clone(),
            submission_count: self.rounds.last().map_or(0, |r| r.submissions.len()),
            prompt: self
                .rounds
                .last()
                .map_or(Default::default(), |r| r.prompt.clone()),
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

pub fn game_state_init() -> GameState {
    let mut state = if DEV_MODE {
        demo_init(vec!["alice", "bob", "carl"])
    } else {
        Default::default()
    };

    state.prompts = {
        let s: String = std::fs::read_to_string("assets/prompts.txt").unwrap_or_default();
        let mut lines = s
            .lines()
            .filter_map(|line| {
                let fragments = line.split("___").collect::<Vec<_>>();
                if fragments.len() == 2 {
                    Some((fragments[0].to_owned(), fragments[1].to_owned()))
                } else {
                    log!("Discarding invalid prompt: {}", line);
                    None
                }
            })
            .collect::<Vec<_>>();
        shuffle(&mut lines);
        lines
    };

    state
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
        config: Config::default(),
        prompts: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use crate::server::types::*;
    #[test]
    fn scores() {
        let state = GameState {
            rounds: vec![
                Round {
                    winner: Some("a".to_owned()),
                    prompt: Prompt {
                        acronym: "abc".to_owned(),
                        ..Prompt::default()
                    },
                    submissions: vec![(
                        "a".to_owned(),
                        vec!["A".to_owned(), "B".to_owned(), "C".to_owned()],
                    )]
                    .into_iter()
                    .collect::<HashMap<_, _>>(),
                    ..Round::default()
                },
                Round {
                    winner: Some("a".to_owned()),
                    prompt: Prompt {
                        acronym: "ef".to_owned(),
                        ..Prompt::default()
                    },
                    submissions: vec![("a".to_owned(), vec!["E".to_owned(), "F".to_owned()])]
                        .into_iter()
                        .collect::<HashMap<_, _>>(),
                    ..Round::default()
                },
                Round {
                    winner: Some("b".to_owned()),
                    prompt: Prompt {
                        acronym: "four".to_owned(),
                        ..Prompt::default()
                    },
                    submissions: vec![(
                        "b".to_owned(),
                        vec![
                            "F".to_owned(),
                            "O".to_owned(),
                            "U".to_owned(),
                            "R".to_owned(),
                        ],
                    )]
                    .into_iter()
                    .collect::<HashMap<_, _>>(),
                    ..Round::default()
                },
                Round {
                    winner: None,
                    judge: 2,
                    prompt: Prompt {
                        acronym: "a".to_owned(),
                        ..Prompt::default()
                    },
                    ..Round::default()
                },
            ],
            ..demo_init(vec!["a", "b", "c"])
        };
        assert_eq!(
            state.scores(),
            vec![
                ("a".to_owned(), 5),
                ("b".to_owned(), 4),
                ("c".to_owned(), -1)
            ]
        );
    }
}
