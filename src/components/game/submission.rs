use super::{acronym::*, context::*, timer::*};
use crate::components::game::utils::state::*;
use crate::components::styles::*;
use crate::types::ClientMessage::*;
use ::futures::future::OptionFuture;
use ::leptos::*;

#[component]
pub fn GameSubmission(cx: Scope) -> impl IntoView {
    let judge = use_typed_context::<Memo_Judge>(cx);
    let submissions = create_memo(cx, move |_| game_state(cx).with(|g| g.submission_count));
    let player_count = game_state(cx).with(|g| g.players.len());

    view! {
        cx,
        <p>
            "Submissions received: "{submissions}"/"{player_count - 1} // judge doesn't submit
        </p>
        <Timer />
        {move|| match judge() {
            None => view! {cx, <><span>"Error: No judge"</span></>},
            Some(Judge::Me) => view! {cx, <><JudgePerspective/></>},
            Some(Judge::Name(name)) => view! {cx, <><PlayerPerspective judge_name=name /></>},
        }}
    }
}

#[component]
fn JudgePerspective(cx: Scope) -> impl IntoView {
    let acronym = move || game_state(cx).with(|g| g.acronym.clone());
    view! { cx,
        <p>
            "You are the judge. "
        </p>
        <p>
            "Submissions incoming for "
            {view! {cx, <Acronym letters=acronym() />}}
        </p>
    }
}

#[component]
fn PlayerPerspective(cx: Scope, judge_name: String) -> impl IntoView {
    let acronym = move || game_state(cx).with(|g| g.acronym.clone());
    let player_id = use_typed_context::<Signal_PlayerId>(cx);
    let num_of_words = game_state(cx).with(|g| g.acronym.len());
    let submission = store_value(cx, vec![String::new(); num_of_words]);
    let submit = create_action(cx, move |_: &()| {
        OptionFuture::from(
            player_id().map(|id| send(cx, SubmitAcronym(id, submission.get_value()))),
        )
    });

    view! { cx,
        <p><span class="inline font-bold">{judge_name}</span>" will be judging."</p>
        <p>
            "What is "{view! {cx, <Acronym letters=acronym() />}}" ?"
        </p>
        {
            (0..num_of_words)
                .map(|i| view! { cx,
                    <input
                        type="text"
                        class=text_input_class()
                        // focus on the first input
                        autofocus={i == 0}
                        on:input=move|e| {
                            submission.update_value(move |s| {
                                if let Some(elem) = s.get_mut(i) {
                                    *elem = event_target_value(&e);
                                }
                            });
                        }
                    /> })
                .collect::<Vec<_>>()
        }
        <button
            class=button_class("bg-green-300")
            on:click=move|_| submit.dispatch(())
        >
            "Submit!"
        </button>
    }
}
