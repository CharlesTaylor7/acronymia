use leptos::html::Input;
use leptos::*;

#[component]
pub fn TextInput<F>(
    cx: Scope,
    on_input: F,
    #[prop(optional)] default: Option<String>,
    #[prop(optional)] disabled: Option<MaybeSignal<bool>>,
) -> impl IntoView
where
    F: FnOnce(String) -> () + 'static + Copy,
{
    let input_ref = create_node_ref::<Input>(cx);
    let callback = move || {
        let element = input_ref.get().expect("input ref wasn't rendered");
        let value = element.value();
        on_input(value);
    };
    view! {
        cx,
        <div>
            <input
                type="text"
                node_ref=input_ref
                value=default
                class="border rounded border-slate-400 px-3"
                disabled=disabled.map(|s| s.get()).unwrap_or(false)
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
