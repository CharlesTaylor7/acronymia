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
        .expect("did you forget to call provide_typed_context::<K>(cx)?")
        .item
}

/// Example: define_context_key!(PlayerId, RwSignal<String>)
/// this defines a new context key called PlayerId,
/// that holds a value of type RwSignal<Value>
///
/// provide_typed_context & use_typed_context can only be called with types that implement
/// ContextKey which enforces just a bit more sanity than the default use_context
/// provided by leptos
macro_rules! define_context_key {
    ($KEY: ident, $VALUE: ty) => {
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        enum $KEY {}
        impl ContextKey for $KEY {
            type R = $VALUE;
        }
    };
}

pub(crate) use define_context_key;
