use leptos::*;

#[component]
pub fn GameJudging(cx: Scope) -> impl IntoView {
    let (submissions, _set_submissions) = create_signal::<Vec<(usize, String)>>(
        cx,
        vec![
            "A Stupid Submission",
            "Another Sexy Shape",
            "haha butts",
            "Adamant, Stubborn Soil-till",
        ]
        .iter()
        .map(|&s| s.to_owned())
        .enumerate()
        .collect(),
    );

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
                        view=move |cx, (i, submission): (usize, String)| {
                            view! {
                                cx,
                                <div>
                                    <input type="radio" name="best" id={ format!("submission_{i}") } value={ submission.clone().to_owned() } checked={ i == 0 } required="required" />
                                    <label for={ format!("submission_{i}") }>{ submission.to_owned() }</label>
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
