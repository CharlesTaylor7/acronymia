use crate::components::text_input::*;
use crate::components::timer::*;
use crate::components::utils::*;
use crate::sse::*;
use crate::types::*;
use ::leptos::*;

#[component]
pub fn GameSubmission(cx: Scope) -> impl IntoView {
    apply_timer_to_game(cx);
    let acronym = store_value(cx, game_state(cx).with(|g| g.acronym.clone()));
    let submission = store_value(cx, vec!["".to_owned(); acronym.with_value(|a| a.len())]);
    let judge = create_memo(cx, move |_| {
        game_state(cx).with(|g| match g.judge {
            Judge::Me(_) => true,
            _ => false,
        })
    });
    view! {
        cx,
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

        <When predicate=judge >
            <p>
                "You are the judge. "
            </p>
            <p>
                "Submissions incoming for "
                {acronym.with_value(|a| view_acronym(cx, a))}
            </p>
        </When>
        <When predicate=move|| !judge() >
            <p>
                "What is "{acronym.with_value(|a| view_acronym(cx, a))}" ?"
            </p>
            {
                let n = acronym.with_value(|a| a.len());
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
                                })
                            }
                        /> })
                    .collect::<Vec<_>>()
            }
            <button
                class="border rounded p-2 bg-green-300 border-slate-200"
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
