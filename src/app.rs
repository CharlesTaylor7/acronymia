use core::time::Duration;
use leptos::html::Input;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Components
#[component]
pub fn App(cx: Scope) -> impl IntoView {
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
