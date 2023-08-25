use crate::components::state::*;
use leptos::*;

/// When the window regains focus, request the current server time.
pub fn auto_sync_with_server(cx: Scope) {
    let request = create_action(cx, move |_| send(cx, GetRemainingTime));
    window_event_listener(ev::focus, move |_| request.dispatch(()));
}
