use core::fmt::{Display, Formatter};

pub trait Expecting {
    /// Generated the actual message of what we expected.
    fn expecting(&self, f: &mut Formatter<'_>) -> core::fmt::Result;

    /// Return a type that can be formatted from `self`.
    #[doc(hidden)]
    fn format(&self) -> &dyn Expecting
    where
        Self: Sized,
    {
        self
    }
}

#[repr(transparent)]
pub struct ExpectingWrapper<T>(T);

impl Expecting for str {
    #[inline]
    fn expecting(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.fmt(f)
    }
}

impl Display for &dyn Expecting {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.expecting(f)
    }
}

pub enum Except {
    Boolean,
    Character,
    Unsigned { bits: usize },
}

impl Expecting for Except {
    fn expecting(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Except::Boolean => f.write_str("boolean"),
            Except::Character => f.write_str("character"),
            Except::Unsigned { bits } => {
                write!(f, "u{}", bits)
            }
        }
    }
}
