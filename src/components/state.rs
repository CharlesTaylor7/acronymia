use crate::types::ClientMessage;
pub use crate::types::ClientMessage::*;
use leptos::Action;

#[cfg(feature = "hydrate")]
pub fn create_ws_action() -> Action<ClientMessage, ()> {
    let owner = leptos::Owner::current().expect("");
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
