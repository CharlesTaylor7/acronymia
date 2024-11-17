use crate::types::ClientMessage;
pub use crate::types::ClientMessage::*;
use leptos::prelude::*;

#[cfg(feature = "hydrate")]
/// # Panics
/// Panics if called from outside of a reactive context
pub fn create_ws_action() -> Action<ClientMessage, ()> {
    let owner = Owner::current().expect("");
    create_action(move |message: &ClientMessage| {
        crate::client::ws::send_from(owner, message.clone())
    })
}

#[cfg(not(feature = "hydrate"))]
pub fn create_ws_action() -> Action<ClientMessage, ()> {
    create_action(move |_| async {})
}
