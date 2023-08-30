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

pub fn provide_typed_context<K>(value: K::R)
where
    K: ContextKey,
{
    provide_context(ContextWrapper {
        item: value,
        _marker: PhantomData::<K>,
    });
}

/// # Panics
/// Will panic if you neglected to call `provide_typed_context::<K>`
/// from within the owning reactive scope 
pub fn use_typed_context<K>() -> K::R
where
    K: ContextKey,
{
    use_typed_context_from::<K>(Owner::current().expect("cannot call use_typed_context without an active reactive runtime"))
}


/// # Panics
/// Will panic if you neglected to call `provide_typed_context::<K>`
/// from within the owning reactive scope 
pub fn use_typed_context_from<K>(owner: Owner) -> K::R
where
    K: ContextKey,
{
    with_owner(owner, move||
        use_context::<ContextWrapper<K, K::R>>()
            .unwrap_or_else(|| panic!("no context with key {k} exists, did you forget to call provide_typed_context::<{k}>?",
                k = std::any::type_name::<K>()))
            .item
    )
}



/// Example: `define_context_key!(PlayerId`, `RwSignal`<String>)
/// this defines a new context key called `PlayerId`,
/// that holds a value of type `RwSignal`<String>
///
/// `provide_typed_context` & `use_typed_context` can only be called with types that implement
/// `ContextKey` which enforces just a bit more sanity than the default `use_context`
/// provided by leptos
macro_rules! define_context {
    ($KEY: ident, $VALUE: ty) => {
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub enum $KEY {}
        impl crate::typed_context::ContextKey for $KEY {
            type R = $VALUE;
        }
    };
}
pub(crate) use define_context;

/* works but is probably too cute, not helpful
macro_rules! context_value {
    ($KEY: ident) => {
       <$KEY as ContextKey>::R
    }
}
pub(crate) use context_value;
*/
