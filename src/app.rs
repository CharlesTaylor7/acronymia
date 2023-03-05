use leptos::html::Input;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);
    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route
                        path=""
                        view=move |cx| view! { cx, <HomePage/> }
                    />
                    <Route
                        path="game/:id"
                        view=move |cx| view! { cx, <Game/> }
                    />
                </Routes>
            </main>
        </Router>
    }
}

enum GameState {
    Setup, // Player's joining and game config
    Submission, // Player's submit acronyms
    Judging, // Judge judges
    Results, // Scoreboard at game end
}

/// The home page allows you to:
/// - Set your nickname
/// - Join a game
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let name = create_rw_signal::<String>(cx, "boaty mcboatface".to_owned());
    let room_code = create_rw_signal::<String>(cx, "".to_owned());
    let join = move |_| println!("joined");
    // provide_context(cx, set_name);

    view! {
        cx,
        <h1>"Welcome to Acronymia!"</h1>
        "Enter your nickname:"
        <TextInput signal=name />
        "Enter your room code: "
        //<TextInput set_text=set_room_code initial=room_code />
        <button on:click=join >
            "Join!"
        </button>
        <p>{ name }</p>
    }
}

#[component]
fn Game(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let game_id = params.with(|p| p.get("id").cloned().unwrap_or_default());

    view! {
        cx,
        <p>"Game Id: "{game_id}</p>

    }
}

#[component]
fn Counter(cx: Scope) -> impl IntoView {
    // create a reactive signal with the initial value
    let (value, set_value) = create_signal(cx, 0);

    // create event handlers for our buttons
    // note that `value` and `set_value` are `Copy`, so it's super easy to move them into closures
    let clear = move |_| set_value(0);
    let decrement = move |_| set_value.update(|value| *value -= 1);
    let increment = move |_| set_value.update(|value| *value += 1);

    // create user interfaces with the declarative `view!` macro
    view! {
        cx,
        <div>
            <button on:click=clear>"Clear"</button>
            <button on:click=decrement>"-1"</button>
            <span>"Value: " {value} "!"</span>
            <button on:click=increment>"+1"</button>
        </div>
    }
}

#[component]
fn TextInput(cx: Scope, signal: RwSignal<String>) -> impl IntoView {
    let input_ref = create_node_ref::<Input>(cx);
    let callback = move || {
        let val = input_ref.get().expect("input ref is rendered");

        let name = val.value();
        signal.set(name);
    };
    view! {
        cx,
        <div>
            <input
                type="text"
                node_ref=input_ref
                value=signal.get()
                on:blur=move|_| callback()
                on:keyup=move |event| {
                    let key = event.key();
                    if key == "Enter" {
                        callback();
                    }
                }
            />
        </div>
    }
}
