use super::{acronym::*, context::*, timer::*};
use crate::components::game::utils::state::*;
use crate::components::styles::*;
use crate::components::utils::*;
use crate::typed_context::*;
use crate::types::ClientMessage::*;
use crate::types::*;
use futures::future::OptionFuture;
use leptos::*;
use std::collections::HashMap;

#[component]
pub fn GameJudging(cx: Scope) -> impl IntoView {
    provide_player_lookup(cx);
    let judge = use_typed_context::<Memo_Judge>(cx);
    let show_timer = create_memo(cx, move |_| {
        game_state(cx).with(|g| g.round_winner.is_none())
    });
    let show_timer = MaybeSignal::derive(cx, move || show_timer());
    view! { cx,
        <When predicate=show_timer>
            <Timer />
        </When>
        {
            move || match judge() {
                None => view! {cx, <><span>"Error: No judge"</span></>},
                Some(Judge::Me) => view! { cx, <><JudgePerspective /></>},
                Some(Judge::Name(name)) => view! { cx, <><PlayerPerspective judge_name=name /></>},
            }
        }
    }
}

#[component]
fn JudgePerspective(cx: Scope) -> impl IntoView {
    let selected = create_rw_signal(cx, None);
    let submit_winner = create_action(cx, move |_: &()| {
        OptionFuture::from(selected().map(|winner| send(cx, JudgeRound(winner))))
    });

    let option_class = move |id: &PlayerId| {
        let id = id.clone();
        MaybeSignal::derive(cx, move || {
            if selected.with(|s| s.as_ref() == Some(&id)) {
                "bg-blue-300".to_owned()
            } else {
                "bg-slate-200 hover:bg-blue-200".to_owned()
            }
        })
    };

    view! {
        cx,
        <header>"What is "<Acronym />"?"</header>
        <Submissions disabled=false on_select=move|t| selected.set(Some(t)) option_class=option_class />

        <button
            class=button_class("bg-green-300 mt-12")
            disabled=move|| {selected().is_none() || submit_winner.version()() > 0}
            on:click=move|_| submit_winner.dispatch(())
        >
        "Submit"
        </button>
    }
}

#[component]
fn PlayerPerspective(cx: Scope, judge_name: String) -> impl IntoView {
    view! { cx,
        <p><span class="inline font-bold">{judge_name}</span>" is deliberating."</p>
        <p>"Submissions for "<Acronym />": "</p>
        <Submissions
            option_class=move|_| "".into()
            disabled=true
            on_select=move|_| { }
        />
    }
}

#[component]
fn Submissions<F1, F2>(
    cx: Scope,
    disabled: bool,
    option_class: F1,
    on_select: F2,
    //impl 'static + Copy + Fn(String),
) -> impl IntoView
where
    F1: 'static + Fn(&PlayerId) -> MaybeSignal<String>,
    F2: 'static + Copy + Fn(String),
{
    //let submissions = move || game_state(cx).with(|g| g.submissions.clone());

    view! { cx,
        <For
            each=move|| game_state(cx).with(|g| g.submissions.clone())
            key=|(id, _)| id.clone()
            view=move |cx, (id, words)| {
                let class = option_class(&id);
                let id2 = id.clone();
                view! {
                    cx,
                    <button
                        class=move|| class.with(|s| button_class(s))
                        disabled=disabled
                        on:click=move|_| on_select(id.clone())
                    >
                        {words.join(" ")}
                    </button>
                    {move|| lookup(cx, &id2).map(|p|
                        view! {cx,
                            <span class="font-bold pr-3">
                                {p.name}{p.is_winner.then_some(" (winner)")}
                            </span>
                        }
                    )}
                }
            }
        />
    }
}

define_context!(LookupPlayer, Memo<HashMap<PlayerId, PlayerInfo>>);
fn provide_player_lookup(cx: Scope) {
    let hashmap = create_memo(cx, move |_| {
        game_state(cx).with(|g| {
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

    provide_typed_context::<LookupPlayer>(cx, hashmap);
}

fn lookup(cx: Scope, id: &PlayerId) -> Option<PlayerInfo> {
    use_typed_context::<LookupPlayer>(cx).with(|h| h.get(id).cloned())
}

#[derive(Clone, PartialEq)]
pub struct PlayerInfo {
    is_winner: bool,
    name: PlayerName,
}
