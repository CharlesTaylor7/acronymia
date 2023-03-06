use core::time::Duration;
use leptos::html::Input;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

// Types

#[derive(Serialize, Deserialize, Clone)]
enum GameState {
    Setup,      // Player's joining and game config
    Submission, // Player's submit acronyms
    Judging,    // Judge judges
    Results,    // Scoreboard at game end
}

#[derive(Serialize, Deserialize, Clone)]
struct Player {
    id: u32,
    name: String,
}

type Res<T> = Resource<u32, T>;

// Apis
// get the players in the game
async fn fetch_players(room_code: &str) -> Vec<Player> {
    // pretend we're fetching people
    vec![
        Player {
            id: 0,
            name: "karl".to_string(),
        },
        Player {
            id: 1,
            name: "marx".to_string(),
        },
    ]
}

// get the players in the game
async fn fetch_game_state(room_code: &str) -> GameState {
    // pretend we're fetching game state
    GameState::Setup
}

// Components
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);
    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Acronymia"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route
                        path="timer-demo"
                        view=move |cx| {
                            let seconds = count_down(cx, 60);
                            view! { cx, "Seconds: "{seconds} }
                        }
                    />
                    <Route
                        path=""
                        view=move |cx| view! { cx, <HomePage/> }
                    />
                    <Route
                        path="game/:room_code"
                        view=move |cx| view! { cx, <Game/> }
                    />
                </Routes>
            </main>
        </Router>
    }
}

/// The home page allows you to:
/// - Set your nickname
/// - Join a game
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let name = create_rw_signal::<String>(cx, "boaty_mcboatface".to_owned());
    let room_code = create_rw_signal::<String>(cx, "abc".to_owned());

    view! {
        cx,
        <h1>"Welcome to Acronymia!"</h1>
        "Enter your nickname:"
        <TextInput signal=name />

        "Enter your room code: "
        <TextInput signal=room_code />
        <A
            href=move|| format!("/game/{}?name={}", room_code.get(), name.get())
        >
            "Join!"
        </A>
        <p>{ name }</p>
    }
}

fn count_down(cx: Scope, initial: u32) -> RwSignal<u32> {
    let seconds = create_rw_signal(cx, initial);
    create_effect(cx, move |_| {
        let handle = set_interval(
            move || {
                let s = seconds.get();
                if s > 0 {
                    seconds.set(s - 1);
                }
            },
            Duration::new(1, 0),
        );

        println!("{:?}", handle);
    });
    seconds
}

#[component]
fn Game(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let room_code = params.with(|p| p.get("room_code").cloned().unwrap_or_default());

    //let seconds = timer(cx);
    let seconds = create_rw_signal(cx, 0);

    // poll for the player names
    let players: Res<Vec<Player>> = create_resource(cx, seconds, move |_| fetch_players(""));

    provide_context(cx, players);

    // poll for the game state
    let game_state = create_resource(cx, seconds, move |_| fetch_game_state(""));

    let game_view = move || match game_state.read(cx) {
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

#[component]
fn TextInput(cx: Scope, signal: RwSignal<String>) -> impl IntoView {
    let input_ref = create_node_ref::<Input>(cx);
    let callback = move || {
        let val = input_ref.get().expect("input ref is rendered");

        let name = val.value();
        signal.set(name);
    };
    view! {
        cx,
        <div>
            <input
                type="text"
                node_ref=input_ref
                value=signal.get()
                on:blur=move|_| callback()
                on:keyup=move |event| {
                    let key = event.key();
                    if key == "Enter" {
                        callback();
                    }
                }
            />
        </div>
    }
}
