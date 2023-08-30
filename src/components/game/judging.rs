use super::{context::*, prompt::*, timer::*};
use crate::components::game::utils::state::*;
use crate::components::styles::*;
use crate::typed_context::*;
use crate::types::ClientMessage::*;
use crate::types::*;
use futures::future::OptionFuture;
use leptos::*;
use std::collections::HashMap;

#[component]
pub fn GameJudging() -> impl IntoView {
    provide_player_lookup();
    let judge = use_typed_context::<Memo_Judge>();
    let round_counter = use_typed_context::<Memo_RoundCounter>();
    view! {
        <h2 class="text-l font-bold">
            {round_counter}
        </h2>
        {
            move || match judge.get() {
                None => view! {<><span>"Error: No judge"</span></>},
                Some(Judge::Me) => view! { <><JudgePerspective /></>},
                Some(Judge::Name(name)) => view! { <><PlayerPerspective judge_name=name /></>},
            }
        }
        <Timer />
    }
}

#[component]
fn JudgePerspective() -> impl IntoView {
    let selected = create_rw_signal(None);
    let submit_action = create_ws_action();
    let submit_winner = move || {
        if let Some(winner) = selected() {
            submit_action.dispatch(JudgeRound(winner));
        }
    };

    let option_class = move |id: &PlayerId| {
        let id = id.clone();
        MaybeSignal::derive(move || {
            if selected.with(|s| s.as_ref() == Some(&id)) {
                "bg-cyan-600".to_owned()
            } else {
                "bg-slate-600 hover:bg-cyan-500".to_owned()
            }
        })
    };

    view! {
        <header><Prompt /></header>
        <Submissions disabled=false on_select=move|t| selected.set(Some(t)) option_class=option_class />

        <button
            class=button_class(ButtonStyle::Secondary, "mt-12")
            disabled=move|| {selected.get().is_none() || submit_action.version().get() > 0}
            on:click=move|_| submit_winner()
        >
        "Submit"
        </button>
    }
}

#[component]
fn PlayerPerspective(judge_name: String) -> impl IntoView {
    let game_signal = game_state();
    view! {
        <Show when=move|| game_signal.with(|g| g.round_winner.is_none()) fallback=move||()>
            <p><span class=judge_class()>{&judge_name}</span>" is deliberating."</p>
        </Show>
        <p><Prompt /></p>
        <Submissions
            option_class=move|_| "".into()
            disabled=true
            on_select=move|_| { }
        />
    }
}

#[component]
fn Submissions<F1, F2>(disabled: bool, option_class: F1, on_select: F2) -> impl IntoView
where
    F1: 'static + Fn(&PlayerId) -> MaybeSignal<String>,
    F2: 'static + Copy + Fn(String),
{
    game_state()
        .with(|g| g.submissions.clone())
        .into_iter()
        .map(|(id, words)| {
            let class = option_class(&id);
            let id2 = id.clone();
            view! {

                <div class="flex flex-col justify-content">
                    <button
                        class=move|| class.with(|s| button_class(ButtonStyle::Nothing, s))
                        disabled=disabled
                        on:click=move|_| on_select(id.clone())
                    >
                        {words.join(" ")}
                    </button>

                    {move|| lookup(&id2).map(|p|
                        view! {
                            <div class="font-bold">
                                {p.is_winner.then_some("👑 ")}{p.name}
                            </div>
                        }
                    )}
                </div>
            }
        })
        .collect::<Vec<_>>()
}

define_context!(LookupPlayer, Memo<HashMap<PlayerId, PlayerInfo>>);
fn provide_player_lookup() {
    let hashmap = create_memo(move |_| {
        game_state().with(|g| {
            g.round_winner
                .as_ref()
                .map_or(HashMap::with_capacity(0), |w| {
                    let mut hashmap = HashMap::new();
                    for p in &g.players {
                        hashmap.insert(
                            p.id.clone(),
                            PlayerInfo {
                                name: p.name.clone(),
                                is_winner: p.id == *w,
                            },
                        );
                    }
                    hashmap
                })
        })
    });

    provide_typed_context::<LookupPlayer>(hashmap);
}

fn lookup(id: &PlayerId) -> Option<PlayerInfo> {
    use_typed_context::<LookupPlayer>().with(|h| h.get(id).cloned())
}

#[derive(Clone, PartialEq)]
pub struct PlayerInfo {
    is_winner: bool,
    name: PlayerName,
}
