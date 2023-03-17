use crate::components::game::utils::state::*;
use ::leptos::*;

#[component]
pub fn GameResults(cx: Scope) -> impl IntoView {
    view! {
        cx,
        "Scoreboard "
        <table>
            <caption>"Scores"</caption>
                {game_state(cx).with(|g|
                    g.scores.iter().map(|(name, score)|
                        view! { cx,
                            <tr>
                                <td>{name}</td>
                                <td>{score.to_string()}</td>
                            </tr>
                        }
                    ).collect::<Vec<_>>()
                )}
        </table>
    }
}
