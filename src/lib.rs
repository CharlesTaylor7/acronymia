#![feature(let_chains)]
#![feature(map_try_insert)]
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
#![allow(clippy::single_match_else)]
#![allow(clippy::range_minus_one)]
#![allow(clippy::unused_async)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::redundant_else)]
#![allow(clippy::similar_names)]
pub mod components;
pub mod constants;
pub mod extensions;
pub mod typed_context;
pub mod types;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub mod server;
    }
}

cfg_if! {
    if #[cfg(feature = "hydrate")] {
        pub mod client;

        use leptos::*;
        use wasm_bindgen::prelude::wasm_bindgen;
        use crate::components::app::App;

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
