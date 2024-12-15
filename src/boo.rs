/// A "Borrowed or Owned" (Boo) type that allows seamless handling of data that may be either
/// owned or borrowed.
///
/// This enum is similar to `std::borrow::Cow`, but without any support for mutation.
/// It is designed to provide an immutable reference to the contained value, regardless
/// of whether the value is owned or borrowed.
///
/// # Variants
/// - `Borrowed(&'a T)`: Contains a borrowed reference to a value.
/// - `Owned(T)`: Contains an owned value.
///
/// # Key Features
/// - Implements `Deref` to provide immutable access to the underlying value.
/// - Supports conversion from both owned (`T`) and borrowed (`&'a T`) values via `From`.
/// - Provides the `get_ref` method to retrieve a reference to the contained value.
#[derive(Debug, Clone)]
pub(crate) enum Boo<'a, T: Sized> {
    Borrowed(&'a T),
    Owned(T),
}

impl<'a, T: Sized> Boo<'a, T> {
    pub fn get_ref(&'a self) -> &'a T {
        match self {
            Boo::Borrowed(obj) => *obj,
            Boo::Owned(obj) => obj,
        }
    }
}

impl<'a, T: Sized> From<T> for Boo<'a, T> {
    fn from(value: T) -> Self {
        Self::Owned(value)
    }
}

impl<'a, T: Sized> From<&'a T> for Boo<'a, T> {
    fn from(value: &'a T) -> Self {
        Self::Borrowed(value)
    }
}

impl<'a, T: Sized> std::ops::Deref for Boo<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get_ref()
    }
}
