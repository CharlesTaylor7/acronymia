use crate::components::game::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);
    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/acronymia.css"/>
        <Title text="Acronymia"/>
        <Body class="font-sans bg-slate-700 text-slate-400"/>
        <Router>
            <main>
                <Routes>
                    <Route
                        path=""
                        view=move |cx| view! { cx, <Game/> }
                    />
                </Routes>
            </main>
        </Router>
    }
}
