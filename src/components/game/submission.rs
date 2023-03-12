use super::context::*;
use crate::api;
use crate::components::text_input::*;
use crate::components::timer::*;
use crate::components::utils::*;
use crate::sse::*;
use crate::typed_context::*;
use crate::types::*;
use ::leptos::*;

#[component]
pub fn GameSubmission(cx: Scope) -> impl IntoView {
    apply_timer_to_game(cx);
    let player_id = use_typed_context::<Signal_PlayerId>(cx);
    let acronym = store_value(cx, game_state(cx).with(|g| g.acronym.clone()));
    let submission = store_value(
        cx,
        vec![String::new(); acronym.with_value(std::string::String::len)],
    );
    let judge = create_memo(cx, move |_| game_state(cx).with(|g| g.judge.clone()));
    let submissions = create_memo(cx, move |_| game_state(cx).with(|g| g.submission_count));
    let player_count = game_state(cx).with(|g| g.players.len());
    let submit = create_action(cx, move |_: &()| {
        use futures::future::OptionFuture;
        OptionFuture::from(player_id().map(|id| api::submit_acronym(id, submission.get_value())))
    });

    view! {
        cx,
        <p>
            "Submissions received: "{submissions}"/"{player_count}
        </p>
        {move|| match game_state(cx).with(|g| g.round_timer) {
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

        <When predicate=move|| judge() == Judge::Me >
            <p>
                "You are the judge. "
            </p>
            <p>
                "Submissions incoming for "
                {acronym.with_value(|a| view_acronym(cx, a))}
            </p>
        </When>
        <When predicate=move|| judge() != Judge::Me >
            <p>
                "What is "{acronym.with_value(|a| view_acronym(cx, a))}" ?"
            </p>
            {
                let n = acronym.with_value(std::string::String::len);
                (0..n)
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

/// - apply bold
/// - capitalize letters
/// - insert periods
fn view_acronym(cx: Scope, s: &str) -> impl IntoView + Clone {
    use core::iter::once;
    view! { cx,
        <span class="inline font-bold">
        {
            s.chars()
                .flat_map(|c| c.to_uppercase().chain(once('.')))
                .collect::<String>()
        }
        </span>
    }
}
