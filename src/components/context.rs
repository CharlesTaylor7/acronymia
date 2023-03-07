use leptos::*;
use std::marker::*;

#[derive(Clone)]
#[repr(transparent)]
struct ContextWrapper<K, T> {
    item: T,
    _marker: PhantomData<K>,
}

pub trait ContextKey: Clone + 'static {
    type R: Clone;
}

pub fn provide_typed_context<K>(cx: Scope, value: K::R)
where
    K: ContextKey,
{
    //provide_context(cx,
    provide_context(
        cx,
        ContextWrapper {
            item: value,
            _marker: PhantomData::<K>,
        },
    );
}

pub fn use_typed_context<K>(cx: Scope) -> K::R
where
    K: ContextKey,
{
    //provide_context(cx,
    use_context::<ContextWrapper<K, K::R>>(cx)
        .expect("did you forget to call provide_typed_context<K>(cx)?")
        .item
}


#[derive(Clone)]
enum PlayerId {}
impl ContextKey for PlayerId {
    type R = RwSignal<Option<String>>;
}
