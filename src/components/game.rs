use crate::api::*;
use crate::components::text_input::*;
use crate::components::timer::*;
use crate::components::utils::*;
use crate::types::*;
use crate::*;
use leptos::*;
use uuid::*;

#[derive(Clone)]
struct GameContext {
    player_id: RwSignal<Option<String>>,
    player_name: RwSignal<String>,
    action_join_game: Action<(), Result<Result<(), std::string::String>, leptos::ServerFnError>>,
    players: Res<Server<Vec<Player>>>,
    game_step: Res<Server<GameStep>>,
    seconds: RwSignal<u32>,
}

fn provide_game_context(cx: Scope) {
    let seconds = clock(cx, 0);
    // poll for the player names
    let players = create_resource(cx, seconds, move |_| fetch_players());

    // poll for the game state
    let game_step = create_resource(cx, seconds, move |_| fetch_game_step());

    let player_id: RwSignal<Option<String>> = create_rw_signal(cx, None);
    let player_name = create_rw_signal(cx, "".to_string());
    let action_join_game = create_action(cx, move |_: &()| {
        api::join_game(Player {
            id: player_id().unwrap(),
            name: player_name(),
        })
    });

    // this only runs once because it does not depend on any reactive values
    // but its wrapped in create_effect to ensure it runs on the client side
    create_effect(cx, move |_| {
        if player_id.get().is_some() {
            return ();
        }
        let new_player_id = move |storage: web_sys::Storage| {
            let id = Uuid::new_v4().to_string();
            storage.set_item(STORAGE_KEY, &id);
            player_id.set(Some(id));
        };
        match window().local_storage() {
            Ok(Some(storage)) => match storage.get_item(STORAGE_KEY) {
                Ok(Some(id)) => player_id.set(Some(id)),
                _ => new_player_id(storage),
            },
            _ => (),
        }
    });

    provide_context::<GameContext>(
        cx,
        GameContext {
            game_step: game_step,
            seconds: seconds,
            player_id: player_id,
            player_name: player_name,
            action_join_game: action_join_game,
            players: players,
        },
    );
}

fn use_game_context(cx: Scope) -> GameContext {
    use_context(cx).expect("did you forget to call provide_game_context?")
}

#[component]
pub fn Game(cx: Scope) -> impl IntoView {
    provide_game_context(cx);
    let context = use_game_context(cx);

    let game_view = move || match context.game_step.read(cx).and_then(|r| r.ok()) {
        None => view! {cx, <><GameNotFound /></>},
        Some(GameStep::Setup) => view! { cx, <><GameSetup /></> },
        Some(GameStep::Submission) => view! { cx, <><GameSubmission /></> },
        Some(GameStep::Judging) => view! { cx, <><GameJudging /></> },
        Some(GameStep::Results) => view! { cx, <><GameResults /></> },
    };
    view! {
        cx,
        <div>
            "Clock: "{context.seconds}
        </div>
        <Transition
            fallback=move || view! { cx, "Loading" }
        >
            {game_view}
        </Transition>
    }
}

#[component]
fn GameNotFound(_cx: Scope) -> impl IntoView {
    view! {
        cx,
        "Game not found!"
    }
}

const STORAGE_KEY: &str = "acronymia-player-id";

#[component]
fn GameSetup(cx: Scope) -> impl IntoView {
    let context = use_game_context(cx);
    view! {
        cx,
        <Debug>
            <div>
                "Override player id (Debug only): "
                <TextInput
                    default=context.player_id.get().unwrap_or("".to_string())
                    on_input=move |text| context.player_id.set(Some(text))
                />
            </div>
        </Debug>
        <div>
            "Pick a Nickname to join: "
            <TextInput
                default=context.player_name.get()
                disabled=MaybeSignal::derive(cx, move|| (context.player_id)().is_none())
                on_input=move|text| {
                    context.player_name.set(text);
                    context.action_join_game.dispatch(())
                }
            />

            "Players:"
            <Transition
                fallback=|| "loading players"
            >
                <ol>
                    <For
                        each=move || read_or(cx, context.players, Vec::new())
                        key=|p| p.id.clone()
                        view=move |cx, p| {
                            view! {
                                cx,
                                <li>{p.name}</li>
                            }
                        }
                    />
                </ol>
            </Transition>
        </div>
    }
}

#[component]
fn GameSubmission(_cx: Scope) -> impl IntoView {
    view! {
        cx,
        "Submission!"
    }
}

#[component]
fn GameJudging(_cx: Scope) -> impl IntoView {
    view! {
        cx,
        "Judging!"
    }
}

#[component]
fn GameResults(_cx: Scope) -> impl IntoView {
    view! {
        cx,
        "Results!"
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
