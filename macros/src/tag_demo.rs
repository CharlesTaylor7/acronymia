use crate::tag::*;
use ::leptos::*;

#[derive(Tag, Clone)]
pub enum WebEvent {
    OnLoad,
    Scroll(f64),
    Click { x: u64, y: u64 },
}

pub fn demo(demo: RwSignal<WebEvent>, cx: Scope) {
    let memo: Memo<WebEventTag> = create_memo(cx, move |_| demo.with(|e| e.to_tag()));

    use WebEventTag::*;
    let n = match memo() {
        OnLoad => 1,
        Scroll => 2,
        Click => 3,
    };
    println!("{}", n);
}
