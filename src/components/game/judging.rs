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

    view! {
        cx,
        <>
            <header>"Judging!"</header>

            <main>
                <fieldset>
                    <legend>"Please select the best submission:"</legend>

                    <For
                        each=submissions
                        key=|(i, _)| i.clone()
                        view=move |cx, (i, words): (usize, Vec<String>)| {
                            view! {
                                cx,
                                <div>
                                    <input type="radio" name="best" id={ format!("submission_{i}") } value={ words.clone().join(" ") } checked={ i == 0 } required="required" />
                                    <label for={ format!("submission_{i}") }>{ words.join(" ") }</label>
                                </div>
                            }
                        }
                    />
                </fieldset>

                <button>"Select"</button>
            </main>
        </>
    }
}
