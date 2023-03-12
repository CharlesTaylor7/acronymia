#![feature(type_alias_impl_trait)]
#![feature(unboxed_closures)]
#![feature(let_chains)]
#![feature(stmt_expr_attributes)]
// enable all clippy lints
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
// disable the clippy lints I don't like
#![allow(clippy::wildcard_imports)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::default_trait_access)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::used_underscore_binding)]
#![allow(clippy::redundant_closure_for_method_calls)]
pub mod api;
pub mod components;
pub mod sse;
pub mod typed_context;
pub mod types;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use leptos::*;
        use wasm_bindgen::prelude::wasm_bindgen;
        use crate::components::app::{App, AppProps};

        #[wasm_bindgen]
        pub fn hydrate() {
            // initializes logging using the `log` crate
            _ = console_log::init_with_level(log::Level::Debug);
            console_error_panic_hook::set_once();

            leptos::mount_to_body(move |cx| {
                view! { cx, <App/> }
            });
        }
    }
}
