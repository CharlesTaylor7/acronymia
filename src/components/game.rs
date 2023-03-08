use crate::api::*;
use crate::components::text_input::*;
use crate::components::timer::*;
use crate::components::utils::*;
use crate::typed_context::*;
use crate::types::*;
use crate::*;
use leptos::*;
use uuid::*;

define_context!(Signal_PlayerId, RwSignal<Option<String>>);
define_context!(Signal_PlayerName, RwSignal<String>);
define_context!(Action_JoinGame, Action<(), Result<(), ServerFnError>>);
define_context!(Resource_Players, Res<Server<Vec<Player>>>);
define_context!(Resource_GameStep, Res<Server<GameStep>>);
define_context!(Signal_Seconds, RwSignal<u32>);

/// a signal for the player id
/// that caches its value inside local storage
fn signal_player_id(cx: Scope) -> RwSignal<Option<String>> {
    const STORAGE_KEY: &str = "acronymia-player-id";

    let player_id: RwSignal<Option<String>> = create_rw_signal(cx, None);

    // this only runs once because it does not depend on any reactive values
    // but its wrapped in create_effect to ensure it runs on the client side
    create_effect(cx, move |_| {
        if player_id.get().is_some() {
            return ();
        }
        let new_player_id = move |storage: web_sys::Storage| {
            let id = Uuid::new_v4().to_string();
            _ = storage.set_item(STORAGE_KEY, &id);
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

    player_id
}

fn provide_game_context(cx: Scope) {
    // poll for the player names
    let seconds = create_rw_signal(cx, 0);
    let players = create_resource(cx, seconds, move |_| fetch_players());

    // poll for the game state
    let seconds = create_rw_signal(cx, 0);
    let game_step = create_resource(cx, seconds, move |_| fetch_game_step());

    let player_id = signal_player_id(cx);
    let player_name = create_rw_signal(cx, "".to_string());
    provide_typed_context::<Resource_Players>(cx, players);
    provide_typed_context::<Resource_GameStep>(cx, game_step);
    provide_typed_context::<Signal_PlayerId>(cx, player_id);
    provide_typed_context::<Signal_PlayerName>(cx, player_name);

    let seconds = clock(cx, 0);
    provide_typed_context::<Signal_Seconds>(cx, seconds);

    let join_game = create_action(cx, move |_: &()| {
        api::join_game(player_id().unwrap_or("".to_owned()), player_name())
    });
    provide_typed_context::<Action_JoinGame>(cx, join_game);
}

#[component]
pub fn Game(cx: Scope) -> impl IntoView {
    provide_game_context(cx);
    let demo = crate::sse::create_sse_signal::<api::Demo>(cx);
    view! {
        cx,
        <div>"SSE Player Count: "{move ||demo().map(|x| x.val)}</div>

        <Transition
            fallback=move || view! { cx, "Loading" }
        >
            { move || match use_typed_context::<Resource_GameStep>(cx)
                    .read(cx)
                    .and_then(|r| r.ok())
                {
                    None => view! {cx, <><GameNotFound /></>},
                    Some(GameStep::Setup) => view! { cx, <><GameSetup /></> },
                    Some(GameStep::Submission) => view! { cx, <><GameSubmission /></> },
                    Some(GameStep::Judging) => view! { cx, <><GameJudging /></> },
                    Some(GameStep::Results) => view! { cx, <><GameResults /></> },
                }
            }
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

#[component]
fn GameSetup(cx: Scope) -> impl IntoView {
    let player_id = use_typed_context::<Signal_PlayerId>(cx);
    let player_name = use_typed_context::<Signal_PlayerName>(cx);
    let players = use_typed_context::<Resource_Players>(cx);
    let join_game = use_typed_context::<Action_JoinGame>(cx);

    view! {
        cx,
        <Debug>
            <div>
                <h1 class="font-bold font-xl">"Begin Debug"</h1>
                "Override player id (Debug only): "
                <TextInput
                    default=player_id.get().unwrap_or("".to_string())
                    on_input=move |text| player_id.set(Some(text))
                />
                <h1 class="font-bold font-xl">"End Debug"</h1>
            </div>
        </Debug>
        <div>
            "Pick a Nickname to join: "
            <TextInput
                default=player_name()
                on_input=move |text| player_name.set(text)
            />

            <button
                class="border rounded p-2 m-2 bg-blue-300 border-slate-200"
                prop:disabled=MaybeSignal::derive(cx, move|| player_id().is_none())
                on:click=move |_| join_game.dispatch(())
            >
                "Join!"
            </button>

            <p>"Players:"</p>
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
