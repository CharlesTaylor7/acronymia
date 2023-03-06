pub mod api;
pub mod components;
pub mod types;

use std::time::Duration;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use cfg_if::cfg_if;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    use crate::components::{
        game::{Game, GameProps},
        home_page::{HomePage, HomePageProps},
    };

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);
    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/style.css"/>

        // sets the document title
        <Title text="Acronymia"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route
                        path="timer-demo"
                        view=move |cx| {
                            let seconds = timer(cx, 60);
                            view! { cx, "Seconds: "{seconds} }
                        }
                    />
                    <Route
                        path=""
                        view=move |cx| view! { cx, <HomePage/> }
                    />
                    <Route
                        path="game/:room_code"
                        view=move |cx| view! { cx, <Game/> }
                    />
                </Routes>
            </main>
        </Router>
    }
}

fn timer(cx: Scope, initial: u32) -> RwSignal<u32> {
    let seconds = create_rw_signal(cx, initial);
    create_effect(cx, move |_| {
        let handle = set_interval(
            move || {
                let s = seconds.get();
                if s > 0 {
                    seconds.set(s - 1);
                }
            },
            Duration::new(1, 0),
        );
        log::debug!("{:?}", &handle);
        on_cleanup(cx, move || {
            handle.map(|h| h.clear());
        });
    });

    seconds
}

cfg_if! {
  if #[cfg(feature = "hydrate")] {

    use wasm_bindgen::prelude::wasm_bindgen;

      #[wasm_bindgen]
      pub fn hydrate() {
        use leptos::*;

        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(move |cx| {
            view! { cx, <App/> }
        });
      }
  }
}
