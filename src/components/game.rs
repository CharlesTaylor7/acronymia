use leptos::*;
use leptos_router::*;

use crate::{
    api::{fetch_game_state, fetch_players},
    types::{GameState, Player, Res},
};

#[component]
pub fn Game(cx: Scope) -> impl IntoView {
    let get_room_code = move || {
        let params = use_params_map(cx);
        params.with(|p| p.get("room_code").cloned().unwrap_or_default())
    };

    //let seconds = timer(cx);
    let seconds = create_rw_signal(cx, 0);

    // poll for the player names
    let players = create_resource(cx, seconds, move |_| fetch_players(get_room_code()));

    provide_context(cx, players);

    // poll for the game state
    let game_state = create_resource(cx, seconds, move |_| fetch_game_state(get_room_code()));

    let game_view = move || match game_state.read(cx).and_then(|r| r.ok()) {
        None => view! {cx, <><GameNotFound /></>},
        Some(GameState::Setup) => view! { cx, <><GameSetup /></> },
        Some(GameState::Submission) => view! { cx, <><GameSubmission /></> },
        Some(GameState::Judging) => view! { cx, <><GameJudging /></> },
        Some(GameState::Results) => view! { cx, <><GameResults /></> },
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

#[component]
fn GameSetup(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let room_code = params.with(|p| p.get("room_code").cloned().unwrap_or_default());

    let players = move || use_context::<Res<Vec<Player>>>(cx)?.read(cx);

    view! {
        cx,
        <p>"Room Code: "{&room_code}</p>

        <Transition
            fallback=|| "loading players"
        >
            <ul>
                <For
                    each=move || players().unwrap()
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
