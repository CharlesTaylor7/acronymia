use super::{acronym::*, context::*, timer::*};
use crate::components::game::utils::state::*;
use crate::components::styles::*;
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
    view! { cx,
        <Timer />
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
    let submissions = move || game_state(cx).with(|g| g.submissions.clone());
    let submit_winner = create_action(cx, move |_: &()| {
        OptionFuture::from(selected().map(|winner| send(cx, JudgeRound(winner))))
    });
    let option_class = move |id: &PlayerId| {
        let id = id.clone();
        move || {
            button_class(if selected.with(|s| s.as_ref() == Some(&id)) {
                "bg-blue-300"
            } else {
                "bg-slate-200 hover:bg-blue-200"
            })
        }
    };

    view! {
        cx,
        <div class="flex flex-col items-start gap-4">
            <header>"What is "<Acronym />"?"</header>
            <For
                each=submissions
                key=|(id, _)| id.clone()
                view=move |cx, (id, words)| {
                    view! {
                        cx,
                        <button
                            class=option_class(&id)
                            on:click=move|_| selected.set(Some(id.clone()))
                        >
                            {words.join(" ")}
                        </button>
                    }
                }
            />

            <button
                class=button_class("bg-green-300 mt-12")
                disabled=move|| selected().is_none()
                on:click=move|_| submit_winner.dispatch(())
            >
            "Submit"
            </button>
        </div>
    }
}

#[component]
fn PlayerPerspective(cx: Scope, judge_name: String) -> impl IntoView {
    view! { cx,
        <p><span class="inline font-bold">{judge_name}</span>" is deliberating."</p>
        <p>"Submissions for "<Acronym />": "</p>
        <ul class="list-inside list-disc flex flex-col items-start" >
            {
                game_state(cx)
                    .with(|g| g
                          .submissions
                          .iter()
                          .map(|(id, s)| {
                              let id = id.clone();
                              view! {cx,
                              <li class="inline">
                                <span class="pr-3">{s.join(" ")}</span>
                                {move|| lookup(cx, &id).map(|p| view! {cx,

                                    <span class="font-bold pr-3">{p.name}{p.is_winner.then_some(" (winner)")}</span>
                                })}
                              </li>
                          }})
                          .collect::<Vec<_>>()
                    )
            }
        </ul>
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
