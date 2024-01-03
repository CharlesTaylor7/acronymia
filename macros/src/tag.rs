pub use macros_impl::Tag;

/// Derivable for any enum
/// Generates a enum with unitary variants for each case of the original enum
pub trait Tag: Sized {
    type Tag: std::fmt::Debug + Clone + PartialEq;
    fn to_tag(&self) -> Self::Tag;
}
