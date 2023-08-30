pub use crate::types::ClientMessage::*;
use crate::types::{ClientGameState, ClientMessage};
use leptos::{log, Action, Owner, RwSignal};

#[cfg(feature = "hydrate")]
pub fn create_ws_action() -> Action<ClientMessage, ()> {
    let owner = Owner::current().expect("");
    leptos::create_action(move |message: &ClientMessage| {
        crate::client::ws::send_from(owner, message.clone())
    })
}

#[cfg(feature = "ssr")]
pub fn create_ws_action() -> Action<ClientMessage, ()> {
    leptos::create_action(move |_| async_do_nothing())
}

#[cfg(feature = "ssr")]
pub async fn async_do_nothing() {}
