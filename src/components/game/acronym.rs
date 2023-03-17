use ::leptos::*;

/// Applies bold, capitalizes letters, inserts periods.
#[component]
pub fn Acronym(cx: Scope, letters: String) -> impl IntoView {
    use core::iter::once;
    view! { cx,
        <span class="inline font-bold">
        {
            letters
                .chars()
                .flat_map(|c| c.to_uppercase().chain(once('.')))
                .collect::<String>()
        }
        </span>
    }
}
