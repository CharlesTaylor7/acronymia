use crate::components::game::utils::state::game_state;
use crate::*;
use ::leptos::*;
use leptos_dom::helpers::IntervalHandle;
pub use typed_context::use_typed_context;
use typed_context::{define_context, provide_typed_context};
use types::*;

define_context!(Signal_PlayerId, RwSignal<Option<PlayerId>>);
define_context!(Signal_PlayerName, RwSignal<String>);
define_context!(Memo_Players, Memo<Vec<Player>>);
define_context!(Memo_Judge, Memo<Option<Judge>>);
define_context!(Action_JoinGame, Action<(), Result<(), ServerFnError>>);
define_context!(TimerHandle, StoredValue<Option<IntervalHandle>>);

#[derive(PartialEq, Eq, Clone)]
pub enum Judge {
    Me,
    Name(String),
}

pub fn provide_game_context(cx: Scope) {
    #[cfg(feature = "hydrate")]
    crate::client::ws::connect_to_server(cx);

    let player_id = signal_player_id(cx);
    provide_typed_context::<Signal_PlayerId>(cx, player_id);

    let player_name = signal_player_name(cx);
    provide_typed_context::<Signal_PlayerName>(cx, player_name);

    let players = create_memo(cx, move |_| game_state(cx).with(|g| g.players.clone()));
    provide_typed_context::<Memo_Players>(cx, players);

    let judge = judge_memo(cx);
    provide_typed_context::<Memo_Judge>(cx, judge);

    let timer_handle = store_value(cx, None);
    provide_typed_context::<TimerHandle>(cx, timer_handle);
}

fn judge_memo(cx: Scope) -> Memo<Option<Judge>> {
    let player_id = use_typed_context::<Signal_PlayerId>(cx);
    let players = use_typed_context::<Memo_Players>(cx);

    create_memo(cx, move |_| {
        game_state(cx).with(|g| {
            g.judge
                .as_ref()
                .map(|judge_id| {
                    if player_id.with(|id| id.as_ref() == Some(judge_id)) {
                        Some(Judge::Me)
                    } else {
                        players.with(|ps| {
                            ps.iter()
                                .find(|p| p.id == *judge_id)
                                .map(|p| Judge::Name(p.name.clone()))
                        })
                    }
                })
                .flatten()
        })
    })
}

/// a signal for the player id
/// that caches its value inside local storage
fn signal_player_id(cx: Scope) -> RwSignal<Option<PlayerId>> {
    let player_id: RwSignal<Option<String>> = create_rw_signal(cx, None);

    #[cfg(feature = "local-storage")]
    if player_id().is_none() {
        use ::uuid::*;
        const STORAGE_KEY: &str = "acronymia-player-id";

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
    }

    player_id
}

/// a signal for the player name
/// that caches its value inside local storage
fn signal_player_name(cx: Scope) -> RwSignal<PlayerName> {
    let player_name: RwSignal<String> = create_rw_signal(cx, String::new());

    #[cfg(feature = "local-storage")]
    {
        const STORAGE_KEY: &str = "acronymia-player-name";

        if let Ok(Some(storage)) = window().local_storage() {
            if let Ok(Some(name)) = storage.get_item(STORAGE_KEY) {
                player_name.set(name);
            }

            create_effect(cx, move |_| {
                player_name.with(|name| {
                    _ = storage.set_item(STORAGE_KEY, name);
                });
            });
        }
    }

    player_name
}
