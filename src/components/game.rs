use leptos::*;

use crate::api::*;
use crate::components::text_input::*;
use crate::components::utils::*;
use crate::types::*;
use crate::*;
use uuid::*;

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
fn GameNotFound(_cx: Scope) -> impl IntoView {
    view! {
        cx,
        "Game not found!"
    }
}

const STORAGE_KEY: &str = "acronymia-player-id";

#[component]
fn GameSetup(cx: Scope, players: Res<Server<Vec<Player>>>) -> impl IntoView {
    let player_id: RwSignal<Option<String>> = create_rw_signal(cx, None);
    let player_name = create_rw_signal(cx, "".to_string());
    let action_join = create_action(cx, |player: &Player| api::join_game(player.clone()));

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

    view! {
        cx,
        <Debug>
            <div>
                "Override player id (Debug only): "
                <TextInput
                    on_input=move |text| player_id.set(Some(text))
                    default=player_id.get().unwrap_or("".to_string())
                />
            </div>
        </Debug>
        <div>
            "Pick a Nickname to join: "
            <TextInput
                default=player_name.get()
                on_input=move|text| {
                    player_name.set(text);
                    player_id.get().map(|id|
                        action_join.dispatch(Player{id: id, name: player_name.get()})
                    );

                }
            />

            "Players:"
            <Transition
                fallback=|| "loading players"
            >
                <ol>
                    <For
                        each=move || read_or(cx, players, Vec::new())
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
