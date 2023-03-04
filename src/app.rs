#[allow(unused_imports)]

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos::html::Input;
use leptos::ev::Custom;
use leptos::ev::*;

// use uuid::Uuid;

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
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_join = move |_| set_count.update(|count| *count += 1);

    let input_ref = create_node_ref::<Input>(cx);
    let on_enter_name = move |event: web_sys::KeyboardEvent| {
        // let input = input_ref.get().unwrap();
        event.stop_propagation();
        let key_code = event.key_code();
        if key_code == ENTER_KEY {
            println!("Hey");
            // let title = input.value();
            // let title = title.trim();
            // if !title.is_empty() { }
        }
    };

    view! { 
        cx,
        <div>
            <h1>"Welcome to Leptos!"</h1>
            <button on:click=on_join>"Join"</button>
            <p>{count}" joined"</p>
            <input 
                type="text" 
                on:keyboard=on_enter_name 
                node_ref=input_ref
            >
            </input>
        </div>
    }
}
