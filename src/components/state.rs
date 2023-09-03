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

#[cfg(not(feature = "hydrate"))]
pub fn create_ws_action() -> Action<ClientMessage, ()> {
    async fn async_do_nothing() {}
    leptos::create_action(move |_| async_do_nothing())
}
