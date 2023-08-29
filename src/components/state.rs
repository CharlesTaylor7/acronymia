pub use crate::types::ClientMessage::*;
use crate::types::{ClientGameState, ClientMessage};
use leptos::{RwSignal, Owner};

pub fn current_owner() -> Owner {
    Owner::current().expect("")
}

#[cfg(feature = "hydrate")]
pub async fn send_from(owner: Owner, message: ClientMessage) {
    crate::client::ws::send_from(owner, message).await
}

#[cfg(feature = "ssr")]
pub async fn send_from(_owner: Owner, _message: ClientMessage) {}


#[cfg(feature = "hydrate")]
pub async fn send(message: ClientMessage) {
    crate::client::ws::send_from(current_owner(), message).await
}

#[cfg(feature = "ssr")]
pub async fn send(_message: ClientMessage) {}

#[cfg(feature = "hydrate")]
pub fn game_state() -> RwSignal<ClientGameState> {
    crate::client::ws::game_state()
}

#[cfg(feature = "ssr")]
pub fn game_state() -> RwSignal<ClientGameState> {
    match leptos::use_context() {
        Some(s) => s,
        None => {
            let signal = leptos::create_rw_signal(Default::default());
            leptos::provide_context(signal);
            signal
        }
    }
}
