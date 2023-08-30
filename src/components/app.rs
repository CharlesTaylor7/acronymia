use crate::components::pages;
use crate::components::game::Game;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    view! {
        <Html lang="en"/>
        <Stylesheet id="leptos" href="/pkg/acronymia.css"/>
        <Title text="Acronymia"/>
        <Body class="h-100 font-sans bg-slate-700 text-slate-400"/>
        <Router>
            <div class="flex flex-row justify-center m-4">
                <div class="mx-2 flex flex-col items-start gap-4">
                    <h1 class="self-center text-4xl font-bold tracking-wide">
                        "Acronymia"
                    </h1>
                    <Routes>
                        <Route
                            path="/"
                            view=Game
                        />
                        <Route
                            path="/lobby"
                            view=pages::Lobby
                        />
                        <Route
                            path="/room/:code"
                            view=pages::Room
                        />
                    </Routes>
                </div>
            </div>
        </Router>
    }
}
