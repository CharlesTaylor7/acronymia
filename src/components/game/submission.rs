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
    view! { cx,
        <p>
            "You are the judge. "
        </p>
        <p>
            "Submissions incoming for "
            {view! {cx, <Acronym />}}
        </p>
    }
}

#[component]
fn PlayerPerspective(cx: Scope, judge_name: String) -> impl IntoView {
    let player_id = use_typed_context::<Signal_PlayerId>(cx);
    let num_of_words = game_state(cx).with(|g| g.acronym.len());

    let submission = create_rw_signal(cx, vec![Err("empty".to_owned()); num_of_words]);

    let submit_args = move || {
        player_id().and_then(|id| submission.with(|s| all_ok(s).map(|s| (id, s))))
    };
    let submit = create_action(cx, move |_: &()| {
        OptionFuture::from(submit_args().map(|(id, s)| send(cx, SubmitAcronym(id, s))))
    });

    view! { cx,
        <p><span class="inline font-bold">{judge_name}</span>" will be judging."</p>
        <p>
            "What is "{view! {cx, <Acronym />}}" ?"
        </p>
        <For
            each=move|| {
                game_state(cx)
                .with(|g| g.acronym.chars().enumerate().collect::<Vec<_>>())
            }
            key=|(i, _)| i.clone()
            view=move |cx, (i, c)| {
                view! {cx,
                    <div>
                        <input
                            type="text"
                            class=text_input_class("inline")
                            autofocus={i == 0}
                            on:input=move |e| {
                                submission.update(move |s| {
                                    let text = event_target_value(&e);
                                    s[i] = validate_word(&c, text);
                                });
                            }
                        />
                        <span class="inline px-3">
                            {move|| submission.with(|s| s[i].clone().err())}
                        </span>
                    </div>
                }
            }
        />
        <button
            class=button_class("bg-green-300")
            disabled=move|| submit_args().is_none()
            on:click=move|_| submit.dispatch(())
        >
            "Submit!"
        </button>
    }
}

fn init_vec<T>(count: usize, f: impl Fn() -> T) -> Vec<T> {
    let mut vec = Vec::with_capacity(count);
    for _ in 0..count {
        vec.push(f())
    }
    vec
}

fn all_ok<T: Clone, E>(v: &[Result<T, E>]) -> Option<Vec<T>> {
    let mut ok_vec = Vec::with_capacity(v.len());
    for r in v {
        match r {
            Ok(item) => {
                ok_vec.push(item.clone());
            },
            Err(_) => {
                return None;
            }
        }
    }

    Some(ok_vec)
}


#[cfg(feature = "ssr")]
fn validate_word<'a>(lead: &'a char, word: String) -> Result<String, String> {
    Ok(word)
}

#[cfg(feature = "hydrate")]
fn validate_word<'a>(lead: &'a char, word: String) -> Result<String, String> {
    use js_sys::RegExp;
    let pattern = RegExp::new(&format!("^{}[a-z]*$", lead), "i");
    if let Some(array) = pattern.exec(&word) {
        Ok(word)
    } else {
        Err(format!(
            "should start with {}",
            lead.to_uppercase().collect::<String>(),
        ))
    }
}
