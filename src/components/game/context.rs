use ::uuid::*;

use crate::sse::*;
use crate::typed_context::*;
use crate::*;


define_context!(Signal_PlayerId, RwSignal<Option<String>>);
define_context!(Signal_PlayerName, RwSignal<String>);
define_context!(Action_JoinGame, Action<(), Result<(), ServerFnError>>);


pub fn provide_game_context(cx: Scope) {
    provide_sse_stream(cx);
    let player_id = signal_player_id(cx);
    provide_typed_context::<Signal_PlayerId>(cx, player_id);

    let player_name = create_rw_signal(cx, "".to_string());
    provide_typed_context::<Signal_PlayerName>(cx, player_name);

    let join_game = create_action(cx, move |_: &()| {
        api::join_game(player_id().unwrap_or("".to_owned()), player_name())
    });
    provide_typed_context::<Action_JoinGame>(cx, join_game);
}


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
