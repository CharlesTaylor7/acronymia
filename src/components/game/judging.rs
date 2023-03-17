use crate::components::game::utils::state::*;
use crate::types::*;
use leptos::*;
use super::acronym::*;

#[component]
pub fn GameJudging(cx: Scope) -> impl IntoView {
    let selected = create_rw_signal(cx, None);
    let acronym = game_state(cx).with(|g| g.acronym.clone());
    let submissions = move || game_state(cx).with(|g| g.submissions.clone());
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
            >
            "Submit"
            </button>
        </div>
    }
}
