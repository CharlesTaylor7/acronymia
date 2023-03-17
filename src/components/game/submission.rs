use super::acronym::*;
use super::{context::*, timer::*};
use crate::components::game::utils::state::*;
use crate::components::text_input::*;
use crate::components::utils::*;
use crate::types::ClientMessage::*;
use ::leptos::*;
use futures::future::OptionFuture;

#[component]
pub fn GameSubmission(cx: Scope) -> impl IntoView {
    apply_timer(cx);
    let player_id = use_typed_context::<Signal_PlayerId>(cx);
    let acronym = move || game_state(cx).with(|g| g.acronym.clone());
    let num_of_words = game_state(cx).with(|g| g.acronym.len());

    let submission = store_value(cx, vec![String::new(); num_of_words]);
    let judge = use_typed_context::<Memo_Judge>(cx);
    let submissions = create_memo(cx, move |_| game_state(cx).with(|g| g.submission_count));
    let player_count = game_state(cx).with(|g| g.players.len());
    let submit = create_action(cx, move |_: &()| {
        OptionFuture::from(
            player_id().map(|id| send(cx, SubmitAcronym(id, submission.get_value()))),
        )
    });

    view! {
        cx,
        <p>
            "Submissions received: "{submissions}"/"{player_count - 1} // judge doesn't submit
        </p>
        {move|| match game_state(cx).with(|g| g.timer) {
            Some(secs) => view! { cx,
                <>
                <p>
                    "Seconds remaining: "{secs}
                </p>
                </>
            },
            None => view! { cx,
                <>
                <p>
                    "Times up!"
                </p>
                </>
            },
        }}

        <When predicate=move|| judge() == Some(Judge::Me) >
            <p>
                "You are the judge. "
            </p>
            <p>
                "Submissions incoming for "
                {view! {cx, <Acronym letters=acronym() />}}
            </p>
        </When>
        <When predicate=move|| judge() != Some(Judge::Me) >
            <p>
                "What is "{view! {cx, <Acronym letters=acronym() />}}" ?"
            </p>
            {
                (0..num_of_words)
                    .map(|i| view! { cx,
                        <TextInput
                            // focus on the first input
                            focus=MaybeSignal::Static(i == 0)
                            on_input=move|text| {
                                submission.update_value(move |s| {
                                    if let Some(elem) = s.get_mut(i) {
                                        *elem = text;
                                    }
                                });
                            }
                        /> })
                    .collect::<Vec<_>>()
            }
            <button
                class="border rounded p-2 bg-green-300 border-slate-200"
                on:click=move|_| submit.dispatch(())
            >
                "Submit!"
            </button>
        </When>
    }
}
