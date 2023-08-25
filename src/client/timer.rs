use crate::components::state::*;
use leptos::*;

/// When the window regains focus, request the current server time.
pub fn auto_sync_with_server() {
    let request = create_action(move |_| send(GetRemainingTime));
    window_event_listener(ev::focus, move |_| request.dispatch(()));
}
