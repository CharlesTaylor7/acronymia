use crate::components::state::*;
use leptos::prelude::*;

/// When the window regains focus, request the current server time.
pub fn auto_sync_with_server() {
    let action = create_ws_action();
    window_event_listener(ev::focus, move |_| action.dispatch(GetRemainingTime));
}
