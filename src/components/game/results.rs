use crate::components::game::utils::state::*;
use ::leptos::*;

#[component]
pub fn GameResults(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <p>"Scoreboard"</p>
        <table class="border border-separate border-spacing-0 rounded border-slate-400">
            <tbody>
                {game_state(cx).with(|g|
                    g.scores.iter().enumerate().map(|(i, (name, score))|
                        view! { cx,
                            <tr>
                                <td
                                    class="border-r border-slate-400 p-4"
                                    class:border-t=i != 0
                                >
                                    {name}
                                </td>
                                <td
                                    class="border-slate-400 p-4"
                                    class:border-t=i != 0
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
