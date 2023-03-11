use crate::components::text_input::*;
use crate::components::timer::*;
use crate::sse::*;
use ::leptos::*;

#[component]
pub fn GameSubmission(cx: Scope) -> impl IntoView {
    apply_timer_to_game(cx);
    view! {
        cx,
        <p>
            "Seconds remaining: "{move|| game_state(cx).with(|g| g.round_timer)}
        </p>
        {move|| game_state(cx).with(|g| g.acronym.as_ref().map(|a|
            view! { cx,
                <p>
                    "What is "<span class="font-bold">{as_acronym(a)}</span>" ?"
                </p>
            }
        ))}
        {move||{
            let n = game_state(cx).with(|g| g.acronym.as_ref().map(|s| s.len()).unwrap_or(0));
            (0..n)
                .map(|_| view! { cx, <TextInput on_input=move|_| () /> })
                .collect::<Vec<_>>()

        }}
    }
}

/// capitalize each letter and insert periods
fn as_acronym(s: &str) -> String {
    use core::iter::once;
    s.chars()
        .flat_map(|c| c.to_uppercase().chain(once('.')))
        .collect()
}
