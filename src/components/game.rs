use leptos::*;
use leptos_router::*;

use crate::api::*;
use crate::types::*;

#[component]
pub fn Game(cx: Scope) -> impl IntoView {
    //let seconds = timer(cx);
    let seconds = create_rw_signal(cx, 0);

    provide_context::<u32>(cx, 18);

    // poll for the player names
    let players = create_resource(cx, seconds, move |_| fetch_players());

    // poll for the game state
    let game_step = create_resource(cx, seconds, move |_| fetch_game_step());

    let game_view = move || match game_step.read(cx).and_then(|r| r.ok()) {
        None => view! {cx, <><GameNotFound /></>},
        Some(GameStep::Setup) => view! { cx, <><GameSetup players=players /></> },
        Some(GameStep::Submission) => view! { cx, <><GameSubmission /></> },
        Some(GameStep::Judging) => view! { cx, <><GameJudging /></> },
        Some(GameStep::Results) => view! { cx, <><GameResults /></> },
    };
    view! {
        cx,
        <Transition
            fallback=move || view! { cx, "Loading" }
        >
            {game_view}
        </Transition>
    }
}

#[component]
fn GameNotFound(cx: Scope) -> impl IntoView {
    view! {
        cx,
        "Game not found!"
    }
}

fn read_or<S, T>(cx: Scope, resource: Resource<S, Result<T, ServerFnError>>, default: T) -> T
where
    S: Clone,
    T: Clone,
{
    resource
        .read(cx)
        .map(|n| n.ok())
        .flatten()
        .unwrap_or(default)
}

#[component]
fn GameSetup(cx: Scope, players: Res<Server<Vec<Player>>>) -> impl IntoView {
    let params = use_params_map(cx);
    let room_code = params.with(|p| p.get("room_code").cloned().unwrap_or_default());

    view! {
        cx,
        <Transition
            fallback=|| "loading players"
        >
            <ul>
                <For
                    each=move || read_or(cx, players, Vec::new())
                    key=|p| p.id
                    view=move |cx, p| {
                        view! {
                            cx,
                            <li>{p.name}</li>
                        }
                    }
                />
            </ul>
        </Transition>
    }
}

#[component]
fn GameSubmission(cx: Scope) -> impl IntoView {
    view! {
        cx,
        "Submission!"
    }
}

#[component]
fn GameJudging(cx: Scope) -> impl IntoView {
    view! {
        cx,
        "Judging!"
    }
}

#[component]
fn GameResults(cx: Scope) -> impl IntoView {
    view! {
        cx,
        "Results!"
    }
}
