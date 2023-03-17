use super::acronym::*;
use super::context::*;
use crate::components::game::utils::state::*;
use crate::types::ClientMessage::*;
use crate::types::*;
use futures::future::OptionFuture;
use leptos::*;

#[component]
pub fn GameJudging(cx: Scope) -> impl IntoView {
    let judge = use_typed_context::<Memo_Judge>(cx);
    {
        move || match judge() {
            None => view! {cx, <><span>"Error: No judge"</span></>},
            Some(Judge::Me) => view! { cx, <><JudgePerspective /></>},
            Some(Judge::Name(name)) => view! { cx, <><PlayerPerspective judge_name=name /></>},
        }
    }
}

#[component]
fn JudgePerspective(cx: Scope) -> impl IntoView {
    let selected = create_rw_signal(cx, None);
    let acronym = game_state(cx).with(|g| g.acronym.clone());
    let submissions = move || game_state(cx).with(|g| g.submissions.clone());
    let submit_winner = create_action(cx, move |_: &()| {
        OptionFuture::from(selected().map(|winner| send(cx, JudgeRound(winner))))
    });
    let option_class = move |id: &PlayerId| {
        let id = id.clone();
        move || {
            format!(
                "cursor-pointer p-2 border rounded border-slate-300 {}",
                if selected() == Some(id.clone()) {
                    "bg-blue-200"
                } else {
                    "bg-slate-200 hover:bg-blue-300"
                }
            )
        }
    };

    view! {
        cx,
        <div class="flex flex-col items-start gap-4">
            <header>"What is "<Acronym letters=acronym /></header>
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
                class="border rounded p-2 bg-green-300 border-slate-200 disabled:cursor-not-allowed"
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
    let acronym = game_state(cx).with(|g| g.acronym.clone());

    view! { cx,
        <p><span class="inline font-bold">{judge_name}</span>" is deliberating."</p>
        <p>"Submissions for "<Acronym letters=acronym />": "</p>
        <ul class="list-inside list-disc flex flex-col items-start" >
            {
                game_state(cx)
                    .with(|g| g
                          .submissions
                          .iter()
                          .map(|(_, s)| view! {cx, <li>{s.join(" ")}</li>})
                          .collect::<Vec<_>>()
                    )
            }
        </ul>
    }
}
