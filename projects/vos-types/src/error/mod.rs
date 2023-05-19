//! Trait governing what error types associated with the encoding framework must
//! support.
//!
//! The most important component in here is `Error::custom` which allows custom
//! encoding implementations to raise custom errors based on types that
//! implement [Display][core::fmt::Display].

#[cfg(feature = "alloc")]
use alloc::string::{String, ToString};
use core::fmt::{Debug, Display};

/// Trait governing errors raised during encodeing or decoding.
pub trait Report: Sized + 'static + Send + Sync + Display + Debug {
    /// Construct a custom error.
    fn custom<T>(error: T) -> Self
    where
        T: 'static + Send + Sync + Display + Debug;

    /// Collect an error from something that can be displayed.
    ///
    /// This is made available to format custom error messages in `no_std`
    /// environments. The error message is to be collected by formatting `T`.
    fn message<T>(message: T) -> Self
    where
        T: Display;

    /// Trying to decode an uninhabitable type.
    #[inline]
    fn uninhabitable(type_name: &'static str) -> Self {
        Self::message(format_args!("{type_name}: cannot decode uninhabitable types",))
    }

    /// The value for the given tag could not be collected.
    #[inline]
    fn expected_tag<T>(type_name: &'static str, tag: T) -> Self
    where
        T: Debug,
    {
        Self::message(format_args!("{type_name}: expected tag: {tag:?}"))
    }

    /// Encountered an unsupported number tag.
    #[inline]
    fn invalid_field_tag<T>(type_name: &'static str, tag: T) -> Self
    where
        T: Debug,
    {
        Self::message(format_args!("{type_name}: invalid field tag: {tag:?}"))
    }

    /// Indicate that a variant wasn't supported by tag.
    #[inline]
    fn invalid_variant_tag<T>(type_name: &'static str, tag: T) -> Self
    where
        T: Debug,
    {
        Self::message(format_args!("{type_name}: invalid variant tag: {tag:?}",))
    }

    /// Missing variant field required to decode.
    #[inline]
    fn missing_variant_field<T>(type_name: &'static str, tag: T) -> Self
    where
        T: Debug,
    {
        Self::message(format_args!("{type_name}: missing variant field: {tag:?}"))
    }

    /// Indicate that a variant tag could not be determined.
    #[inline]
    fn missing_variant_tag(type_name: &'static str) -> Self {
        Self::message(format_args!("{type_name}: missing variant tag"))
    }

    /// Encountered an unsupported variant field.
    #[inline]
    fn invalid_variant_field_tag<V, T>(type_name: &'static str, variant: V, tag: T) -> Self
    where
        V: Debug,
        T: Debug,
    {
        Self::message(format_args!("{type_name}: invalid variant field tag: variant: {variant:?}, tag: {tag:?}",))
    }
}

#[cfg(feature = "alloc")]
impl Report for String {
    fn custom<T>(message: T) -> Self
    where
        T: 'static + Send + Sync + Display + Debug,
    {
        message.to_string()
    }

    fn message<T>(message: T) -> Self
    where
        T: Display,
    {
        message.to_string()
    }
}
