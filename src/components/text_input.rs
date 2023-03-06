use leptos::html::Input;
use leptos::*;

#[component]
pub fn TextInput(cx: Scope, signal: RwSignal<String>) -> impl IntoView {
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
                class="border rounded border-slate-400"
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
