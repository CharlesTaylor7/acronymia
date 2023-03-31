use super::{acronym::*, context::*, timer::*};
use crate::components::game::utils::state::*;
use crate::components::styles::*;
use crate::types::{ClientMessage::*, PlayerId, Submission};
use ::leptos::*;

#[component]
pub fn GameSubmission(cx: Scope) -> impl IntoView {
    let judge = use_typed_context::<Memo_Judge>(cx);
    let round_counter = use_typed_context::<Memo_RoundCounter>(cx);
    let submissions = create_memo(cx, move |_| game_state(cx).with(|g| g.submission_count));
    let player_count = game_state(cx).with(|g| g.players.len());

    view! {
        cx,
        <h2 class="text-l font-bold">
            {round_counter}
        </h2>
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
    let acronym = create_memo(cx, move|_| game_state(cx).with(|g| g.acronym.clone()));
    let num_of_words = acronym().len();
    let input_refs = store_value(
        cx,
        init_vec(6, move || create_node_ref::<html::Input>(cx)),
    );
    let get_ref = move |i| input_refs.with_value(|r| r[i]);
    let submission = create_rw_signal::<Vec<Option<String>>>(cx, vec![None; num_of_words]);

    let submit_args =
        move || player_id().and_then(|id| submission.with(|s| all_some(s).map(|s| (id, s))));
    let submit = create_action(cx, move |(id, s): &(PlayerId, Submission)| {
        send_and_save(cx, id.clone(), s.clone())
    });

    view! { cx,
        <p><span class="inline font-bold">{judge_name}</span>" will be judging."</p>
        <p>
            "What is "{view! {cx, <Acronym />}}" ?"
        </p>
        {move|| acronym().chars().enumerate().map(|(i, c)|{
            // the macro gets confused and doesn't notice this variable is used
            #[allow(unused_variables)]
            let node_ref = get_ref(i);
            view! {cx,
                <input
                    type="text"
                    class=text_input_class("invalid:border-red-300")
                    node_ref=node_ref
                    autofocus={i == 0}
                    on:keydown=move |e| {
                        if e.key() == "Enter" {
                            if i == num_of_words - 1 {
                                if let Some(args) = submit_args() {
                                    submit.dispatch(args);
                                }
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
                class=button_class("bg-green-300")
                disabled=move|| submit_args().is_none()
                on:click=move|_| if let Some(args) = submit_args() { submit.dispatch(args) }
            >
                "Submit!"
            </button>
            <span class="px-2">
                {move|| if submit.version()() > 0 {
                    submit.value().get().map(|s|
                        Some(view! {cx,
                            <span>
                                "submitted: "
                                <span class="font-bold">{s.join(" ")}</span>
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

    Some(v.iter().map(|o| o.as_ref().unwrap().clone()).collect::<Vec<_>>())
}

async fn send_and_save(cx: Scope, id: PlayerId, s: Submission) -> Submission {
    send(cx, SubmitAcronym(id, s.clone())).await;
    s
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

