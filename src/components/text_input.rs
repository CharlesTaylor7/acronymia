use leptos::html::Input;
use leptos::*;

#[component]
pub fn TextInput<F>(
    cx: Scope,
    on_input: F,
    #[prop(optional)] default: Option<String>,
    #[prop(optional)] disabled: Option<MaybeSignal<bool>>,
    #[prop(optional)] focus: Option<MaybeSignal<bool>>,
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

    let focus1 = focus.unwrap_or(Default::default());
    let focus2 = clone_maybe_signal(&focus1);

    create_effect(cx, move |_| {
        let focus = focus1();
        let s = input_ref.get();
        if focus && let Some(el) = s {
            el.focus();
        }
    });

    view! {
        cx,
        <div>
            <input
                type="text"
                node_ref=input_ref
                value=default
                class="border rounded border-slate-400 px-3"
                autofocus=focus2()
                prop:disabled=disabled.unwrap_or(Default::default())
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

// TODO: temporary until MaybeSignal implements Clone
// https://github.com/leptos-rs/leptos/pull/660
fn clone_maybe_signal<T: Clone>(signal: &MaybeSignal<T>) -> MaybeSignal<T> {
    match signal {
        MaybeSignal::Static(item) => MaybeSignal::Static(item.clone()),
        MaybeSignal::Dynamic(signal) => MaybeSignal::Dynamic(signal.clone()),
    }
}
