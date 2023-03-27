pub use macros_impl::Tag;

/// Derivable for structs with named fields that implement
/// PartialEq & Clone.
pub trait Tag: Sized {
    type Tag: std::fmt::Debug + Clone + PartialEq;
    fn to_tag(&self) -> Self::Tag;
}
