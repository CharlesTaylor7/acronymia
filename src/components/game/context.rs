use crate::constants::*;
use crate::*;
use ::leptos::*;
use leptos_dom::helpers::IntervalHandle;
pub use typed_context::use_typed_context;
use typed_context::{define_context, provide_typed_context};
use types::*;

define_context!(Signal_GameState, RwSignal<ClientGameState>);
define_context!(Signal_PlayerId, RwSignal<PlayerId>);
define_context!(Signal_PlayerName, RwSignal<PlayerName>);
define_context!(Memo_Players, Memo<Vec<Player>>);
define_context!(Memo_Judge, Memo<Option<Judge>>);
define_context!(Memo_IsHost, Memo<bool>);
define_context!(Memo_RoundCounter, Memo<String>);
define_context!(TimerHandle, StoredValue<Option<IntervalHandle>>);

#[derive(PartialEq, Eq, Clone)]
pub enum Judge {
    Me,
    Name(String),
}

pub fn provide_game_context() {
    let game_state = create_rw_signal(Default::default());
    provide_typed_context::<Signal_GameState>(game_state);

    let player_id = signal_player_id();
    provide_typed_context::<Signal_PlayerId>(player_id);

    #[cfg(feature = "hydrate")]
    crate::client::ws::connect_to_server(game_state, player_id);

    #[cfg(feature = "hydrate")]
    crate::client::timer::auto_sync_with_server();

    let player_name = signal_player_name();
    provide_typed_context::<Signal_PlayerName>(player_name);
    if DEV_MODE {
        // synchronize player id with player name
        // this ensures impersonation works properly
        create_effect(move |_| {
            player_id.set(player_name.get());
        });
    }

    let players = create_memo(move |_| game_state.with(|g| g.players.clone()));
    provide_typed_context::<Memo_Players>(players);

    let judge = judge_memo();
    provide_typed_context::<Memo_Judge>(judge);

    let is_host = memo_is_host();
    provide_typed_context::<Memo_IsHost>(is_host);

    let timer_handle = store_value(None);
    provide_typed_context::<TimerHandle>(timer_handle);

    let round_counter = create_memo(move |_| game_state.with(|g| g.round_counter.clone()));
    provide_typed_context::<Memo_RoundCounter>(round_counter);
}

fn judge_memo() -> Memo<Option<Judge>> {
    let player_id = use_typed_context::<Signal_PlayerId>();
    let players = use_typed_context::<Memo_Players>();
    let game_state = use_typed_context::<Signal_GameState>();

    create_memo(move |_| {
        game_state.with(|g| {
            g.judge.as_ref().and_then(|judge_id| {
                if player_id.with(|id| id == judge_id) {
                    Some(Judge::Me)
                } else {
                    players.with(|ps| {
                        ps.iter()
                            .find(|p| p.id == *judge_id)
                            .map(|p| Judge::Name(p.name.clone()))
                    })
                }
            })
        })
    })
}

fn memo_is_host() -> Memo<bool> {
    let player_id = use_typed_context::<Signal_PlayerId>();
    let game_state = use_typed_context::<Signal_GameState>();
    create_memo(move |_| {
        player_id
            .with(|me| {
                game_state
                    .get()
                    .players
                    .first()
                    .as_ref()
                    .map(|p| p.id == *me)
            })
            .unwrap_or(false)
    })
}

#[cfg(not(feature = "hydrate"))]
fn signal_player_id() -> RwSignal<PlayerId> {
    create_rw_signal(String::new())
}

/// a signal for the player id
/// that caches its value inside local storage
#[cfg(feature = "hydrate")]
fn signal_player_id() -> RwSignal<PlayerId> {
    const STORAGE_KEY: &str = "acronymia-player-id";

    let new_player_id = move |storage: web_sys::Storage| {
        let id = uuid::Uuid::new_v4().to_string();
        _ = storage.set_item(STORAGE_KEY, &id);
        id
    };
    let storage = window().local_storage().unwrap().unwrap();
    let id = match storage.get_item(STORAGE_KEY) {
        Ok(Some(id)) => id,
        _ => new_player_id(storage),
    };
    create_rw_signal(id)
}

/// a signal for the player name
/// that caches its value inside local storage
fn signal_player_name() -> RwSignal<PlayerName> {
    let player_name: RwSignal<String> = create_rw_signal(String::new());

    #[cfg(feature = "hydrate")]
    {
        const STORAGE_KEY: &str = "acronymia-player-name";

        if let Ok(Some(storage)) = window().local_storage() {
            if let Ok(Some(name)) = storage.get_item(STORAGE_KEY) {
                player_name.set(name);
            }

            create_effect(move |_| {
                player_name.with(|name| {
                    _ = storage.set_item(STORAGE_KEY, name);
                });
            });
        }
    }

    player_name
}
