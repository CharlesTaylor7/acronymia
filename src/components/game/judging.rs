use leptos::*;

use crate::components::game::utils::state::*;

#[component]
pub fn GameJudging(cx: Scope) -> impl IntoView {
    let submissions = Signal::derive(cx, move || {
        game_state(cx)()
            .submissions
            .into_iter()
            .map(|t| t.1)
            .enumerate()
            .collect::<Vec<(usize, Vec<String>)>>()
    });

    // I still need to grab the select size from the number of submissions

    view! {
        cx,
        <>
            <header>"Judging!"</header>

            <main>
                <select size="2" required="required">
                    <For
                        each=submissions
                        key=|(i, _)| i.clone()
                        view=move |cx, (_i, words): (usize, Vec<String>)| {
                            view! {
                                cx,
                                <option value={ words.clone().join(" ") }>{ words.join(" ") }</option>
                            }
                        }
                    />
                </select>

                <button>"Select"</button>
            </main>
        </>
    }
}
