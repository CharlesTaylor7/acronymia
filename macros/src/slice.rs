use leptos::*;
pub use macros_impl::Sliceable;

/// Derivable for structs with named fields that implement
/// PartialEq & Clone.
pub trait Sliceable: Sized {
    type Sliced;
    fn slice(signal: RwSignal<Self>) -> Self::Sliced;
}

#[doc(hidden)]
pub fn __create_slice<T, O: PartialEq<O>>(
    signal: RwSignal<T>,
    getter: impl Fn(&T) -> O + Clone + Copy + 'static,
    setter: impl Fn(&mut T, O) + Clone + Copy + 'static,
) -> JoinedSignal<O> {
    let (getter, setter) = create_slice(signal, getter, setter);
    JoinedSignal { getter, setter }
}

/// Not intended to be implemented directly.
/// Import this to provide an extension method for RwSignals.
pub trait SignalSliceExt {
    type Output;
    fn slice(self) -> Self::Output;
}

impl<T: Sliceable> SignalSliceExt for RwSignal<T> {
    type Output = T::Sliced;

    /// Create a slice for each field of the struct
    #[inline]
    fn slice(self) -> T::Sliced {
        Sliceable::slice(self)
    }
}

pub struct JoinedSignal<T: 'static> {
    getter: Signal<T>,
    setter: SignalSetter<T>,
}

impl<T> SignalWith<T> for JoinedSignal<T> {
    fn try_with<O>(&self, f: impl FnOnce(&T) -> O) -> Option<O> {
        self.getter.try_with(f)
    }

    fn with<U>(&self, f: impl FnOnce(&T) -> U) -> U {
        self.getter.with(f)
    }
}

impl<T> SignalSet<T> for JoinedSignal<T> {
    fn try_set(&self, new_value: T) -> Option<T> {
        self.setter.try_set(new_value)
    }

    fn set(&self, new_value: T) {
        self.setter.set(new_value)
    }
}
