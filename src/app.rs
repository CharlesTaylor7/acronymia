#[allow(unused_imports)]

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos::html::Input;


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
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

const ENTER_KEY: u32 = 13;

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    view! { 
        cx,
        <h1>"Welcome to Leptos!"</h1>
        <Counter initial_value=0 />
        <Name />
    }
}

#[component]
fn Counter(cx: Scope, initial_value: i32) -> impl IntoView {
    // create a reactive signal with the initial value
    let (value, set_value) = create_signal(cx, initial_value);

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
fn Name(cx: Scope) -> impl IntoView {
    let input_ref = create_node_ref::<Input>(cx);
    let (text, set_text) = create_signal(cx, "");
    let on_enter_name = move |event: web_sys::KeyboardEvent| {
        event.stop_propagation();
        let key_code = event.key_code();
        if key_code == ENTER_KEY {
            println! ("{}", "32")
            // let el = input_ref.get().unwrap()
            // let val = el.value();
            // set_text(val.trim())
        }
    };
    view! { 
        cx,
        <input 
            type="text"
            node_ref=input_ref 
            on:keyboard=on_enter_name
        />
        "Echo:"
        <p>{text}</p>
    }
}

