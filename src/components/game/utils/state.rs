use crate::types::{ClientGameState, ClientMessage};
use leptos::{RwSignal, Scope};
pub use crate::types::ClientMessage::*;

#[cfg(feature = "hydrate")]
pub async fn send(cx: Scope, message: ClientMessage) {
    crate::client::ws::send(cx, message).await
}

#[cfg(feature = "ssr")]
pub async fn send(_cx: Scope, _message: ClientMessage) {}

#[cfg(feature = "hydrate")]
pub fn game_state(cx: Scope) -> RwSignal<ClientGameState> {
    crate::client::ws::game_state(cx)
}

#[cfg(feature = "ssr")]
pub fn game_state(cx: Scope) -> RwSignal<ClientGameState> {
    match leptos::use_context(cx) {
        Some(s) => s,
        None => {
            let signal = leptos::create_rw_signal(cx, Default::default());
            leptos::provide_context(cx, signal);
            signal
        }
    }
}
