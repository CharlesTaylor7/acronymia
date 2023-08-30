use super::{context::*, prompt::*, timer::*};
use crate::components::state::*;
use crate::components::styles::*;
use crate::types::ClientMessage::*;
use ::leptos::*;

#[component]
pub fn GameSubmission() -> impl IntoView {
    let judge = use_typed_context::<Memo_Judge>();
    let round_counter = use_typed_context::<Memo_RoundCounter>();
    let game_state = use_typed_context::<Signal_GameState>();
    let submission_ratio = create_memo(move |_| {
        game_state.with(|g| format!("{}/{}", g.submission_count, g.players.len() - 1))
    });

    view! {
        <h2 class="text-l font-bold">
            {round_counter}
        </h2>
        <Prompt/>
        <Show when=move|| judge.get() != Some(Judge::Me) fallback=move|| () >
            <PlayerPerspective />
        </Show>
        <Timer/>
        <JudgeDescription/>
        <p>
            <span class="text-pink-100">
                {submission_ratio}
            </span>
            " submissions received"
        </p>
    }
}

#[component]
fn JudgeDescription() -> impl IntoView {
    let judge = use_typed_context::<Memo_Judge>();
    move || match judge.get() {
        None => view! {<p>"Error: No judge"</p>},
        Some(Judge::Me) => view! {
            <p>
                <span class=judge_class()>"You"</span>" are the judge."
            </p>
        },
        Some(Judge::Name(name)) => view! {
            <p>
                <span class=judge_class()>{name}</span>" is the judge."
            </p>
        },
    }
}

#[component]
fn PlayerPerspective() -> impl IntoView {
    let player_id = use_typed_context::<Signal_PlayerId>();
    let game_state = use_typed_context::<Signal_GameState>();
    let acronym = create_memo(move |_| game_state.with(|g| g.prompt.acronym.clone()));

    let num_of_words = acronym().len();
    let input_refs = store_value(init_vec(10, move || create_node_ref::<html::Input>()));
    let get_ref = move |i| input_refs.with_value(|r| r[i]);
    let submission = create_rw_signal::<Vec<Option<String>>>(vec![None; num_of_words]);

    let submit_args = move || {
        player_id
            .get()
            .and_then(|id| submission.with(|s| all_some(s).map(|s| (id, s))))
    };

    let last_submission = store_value(None as Option<String>);
    let submit_action = create_ws_action();
    let submit = move || {
        if let Some((id, s)) = submit_args() {
            last_submission.set_value(Some(s.join(" ")));
            submit_action.dispatch(SubmitAcronym(id, s));
        }
    };

    view! {
        {move|| acronym.get().chars().enumerate().map(|(i, c)|{
            // the macro gets confused and doesn't notice this variable is used
            #[allow(unused_variables)]
            let node_ref = get_ref(i);
            view! {
                <input
                    type="text"
                    class=text_input_class("invalid:border-red-300")
                    node_ref=node_ref
                    on:keydown=move |e| {
                        if e.key() == "Enter" {
                            if i == num_of_words - 1 {
                                submit()
                            } else {
                               _ = get_ref(i+1).get().unwrap().focus();
                            }
                        }
                    }
                    on:input=move |e| {
                        let input: web_sys::HtmlInputElement = event_target(&e);
                        let text = input.value();
                        let result = validate_word(c, &text);
                        match &result {
                            Ok(_) => {
                                input.set_custom_validity("");
                                submission.update(move |s| s[i] = Some(text));
                            }
                            Err(s) => {
                                input.set_custom_validity(s);
                                submission.update(move |s| s[i] = None);
                            }
                        }
                        input.report_validity();
                    }
                />
            }
        }).collect::<Vec<_>>()}
        <div>
            <button
                class=button_class(ButtonStyle::Primary, "")
                disabled=move|| submit_args().is_none()
                on:click=move|_| submit()
            >
                "Submit"
            </button>
            <span class="px-2">
                {move|| if submit_action.version().get() > 0 {
                    last_submission.get_value().map(|s|
                        Some(view! {
                            <span>
                                "submitted: "
                                <span class="font-bold">{s}</span>
                            </span>
                        })
                    )
                } else {
                    None
                }}
            </span>
        </div>
    }
}

fn init_vec<T>(count: usize, f: impl Fn() -> T) -> Vec<T> {
    let mut vec = Vec::with_capacity(count);
    for _ in 0..count {
        vec.push(f());
    }
    vec
}

fn all_some<T: Clone>(v: &[Option<T>]) -> Option<Vec<T>> {
    if v.iter().any(|o| o.is_none()) {
        return None;
    }

    Some(
        v.iter()
            .map(|o| o.as_ref().unwrap().clone())
            .collect::<Vec<_>>(),
    )
}

#[cfg(feature = "ssr")]
fn validate_word(_lead: char, _word: &str) -> Result<(), String> {
    Ok(())
}

/// Validates leading character.
/// TODO: Should we enforce alphanumeric characters?
#[cfg(feature = "hydrate")]
fn validate_word(lead: char, word: &str) -> Result<(), String> {
    use js_sys::RegExp;
    let pattern = RegExp::new(&format!("^{}", lead), "i");
    if let Some(_) = pattern.exec(word) {
        Ok(())
    } else {
        Err(format!(
            "Should start with {}",
            lead.to_uppercase().collect::<String>(),
        ))
    }
}
