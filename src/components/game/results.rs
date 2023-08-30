use crate::components::game::utils::state::*;
use ::leptos::*;

#[component]
pub fn GameResults() -> impl IntoView {
    view! {

        <p>"Scoreboard"</p>
        <table class="rounded border border-separate border-spacing-0 border-slate-400">
            <tbody>
                {expect_context().with(|g|
                    g.scores.iter().enumerate().map(|(i, (name, score))|
                        view! {
                            <tr>
                                <td
                                    class="border-r border-slate-400 p-4"
                                    class=("border-t", i != 0)
                                >
                                    {name}
                                </td>
                                <td
                                    class="border-slate-400 p-4"
                                    class=("border-t", i != 0)
                                >
                                    {score.to_string()}
                                </td>
                            </tr>
                        }
                    ).collect::<Vec<_>>()
                )}
            </tbody>
        </table>
    }
}
