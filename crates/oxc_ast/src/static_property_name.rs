use std::{
    fmt::{self, Write},
    hash::{Hash, Hasher},
};

use oxc_str::{Ident, JSStr};

/// The JavaScript property name of a statically known property key.
///
/// Static means statically determinable, not a `static` class member.
/// Numeric keys keep their value and are named by the Rust `f64` `Display` text, so equality and
/// hashing treat `{ 1: a, "1": b }` as declaring the same name twice.
#[derive(Clone, Copy)]
pub enum StaticPropertyName<'a> {
    /// A name stored in the AST as a string.
    Str(JSStr<'a>),
    /// The value of a numeric key.
    Number(f64),
}

impl<'a> StaticPropertyName<'a> {
    /// Borrow the name as UTF-8, if it is a string without a lone surrogate.
    ///
    /// Returns `None` for a [`Number`] key, whose name exists only as text produced on demand.
    /// Use the [`Display`] implementation to get the name of a numeric key.
    ///
    /// [`Number`]: Self::Number
    /// [`Display`]: fmt::Display
    pub fn as_str(self) -> Option<&'a str> {
        match self {
            Self::Str(name) => name.as_str(),
            Self::Number(_) => None,
        }
    }
}

/// Call `f` with the full text of `name`, which is the Rust `f64` `Display` text for a numeric key.
///
/// A numeric key is formatted into a stack buffer, so this never allocates.
fn with_name_text<R>(name: StaticPropertyName<'_>, f: impl FnOnce(JSStr<'_>) -> R) -> R {
    match name {
        StaticPropertyName::Str(name) => f(name),
        StaticPropertyName::Number(value) => {
            let mut text = NumberText { buffer: [0; NumberText::CAPACITY], len: 0 };
            write!(text, "{value}").expect("`f64` Display text fits in `NumberText`");
            f(JSStr::from(text.as_str()))
        }
    }
}

/// A stack buffer for the Rust `Display` text of an `f64`.
struct NumberText {
    buffer: [u8; Self::CAPACITY],
    len: usize,
}

impl NumberText {
    /// The longest `f64` Display text is a negative subnormal written without an exponent,
    /// which is under 330 bytes.
    const CAPACITY: usize = 512;

    fn as_str(&self) -> &str {
        std::str::from_utf8(&self.buffer[..self.len]).expect("`f64` Display text is ASCII")
    }
}

impl Write for NumberText {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let end = self.len + s.len();
        self.buffer.get_mut(self.len..end).ok_or(fmt::Error)?.copy_from_slice(s.as_bytes());
        self.len = end;
        Ok(())
    }
}

impl<'a> From<&'a str> for StaticPropertyName<'a> {
    fn from(name: &'a str) -> Self {
        Self::Str(JSStr::from(name))
    }
}

impl<'a> From<JSStr<'a>> for StaticPropertyName<'a> {
    fn from(name: JSStr<'a>) -> Self {
        Self::Str(name)
    }
}

impl<'a> From<Ident<'a>> for StaticPropertyName<'a> {
    fn from(name: Ident<'a>) -> Self {
        Self::Str(name.as_js_str())
    }
}

impl PartialEq for StaticPropertyName<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Self::Str(a), Self::Str(b)) => a == b,
            // `f64` Display prints every NaN as "NaN" and is otherwise injective.
            (Self::Number(a), Self::Number(b)) => {
                a.to_bits() == b.to_bits() || (a.is_nan() && b.is_nan())
            }
            (Self::Str(name), number) | (number, Self::Str(name)) => {
                with_name_text(number, |number| number == name)
            }
        }
    }
}

impl Eq for StaticPropertyName<'_> {}

impl Hash for StaticPropertyName<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        with_name_text(*self, |name| name.hash(state));
    }
}

impl PartialEq<str> for StaticPropertyName<'_> {
    fn eq(&self, other: &str) -> bool {
        with_name_text(*self, |name| name == other)
    }
}

impl PartialEq<&str> for StaticPropertyName<'_> {
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<JSStr<'_>> for StaticPropertyName<'_> {
    fn eq(&self, other: &JSStr<'_>) -> bool {
        with_name_text(*self, |name| name == *other)
    }
}

impl PartialEq<Ident<'_>> for StaticPropertyName<'_> {
    fn eq(&self, other: &Ident<'_>) -> bool {
        self == other.as_str()
    }
}

impl fmt::Display for StaticPropertyName<'_> {
    /// Display a name for diagnostics, escaping lone surrogates in the same lowercase spelling as
    /// the Debug form and printed output.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        with_name_text(*self, |name| {
            if let Some(name) = name.as_str() {
                return f.write_str(name);
            }
            for ch in name.chars() {
                if let Some(ch) = ch.to_char() {
                    write!(f, "{ch}")?;
                } else {
                    write!(f, "\\u{:04x}", ch.to_u32())?;
                }
            }
            Ok(())
        })
    }
}

impl fmt::Debug for StaticPropertyName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        with_name_text(*self, |name| fmt::Debug::fmt(&name, f))
    }
}

#[cfg(test)]
mod tests {
    use super::StaticPropertyName;
    use oxc_allocator::Allocator;
    use oxc_str::{JSStr, JSStrBuilder};
    use rustc_hash::FxHashSet;

    #[test]
    fn names_preserve_identity_across_storage_and_surrogates() {
        let allocator = Allocator::new();
        let mut names = FxHashSet::default();
        assert!(names.insert(StaticPropertyName::Number(42.0)));
        assert!(!names.insert(StaticPropertyName::from("42")));
        for units in
            [&[0xD800][..], &[0xD801], &[0xDC00], &[0xD800, 0xDC00], &[0xDC00, 0xD800], &[0xFFFD]]
        {
            let mut builder = JSStrBuilder::new_in(&&allocator);
            builder.push_utf16(units);
            let name = StaticPropertyName::Str(builder.into_js_str());
            assert!(names.insert(name));
            assert!(!names.insert(name));
        }
        assert!(names.insert(StaticPropertyName::from(r"\uD800")));
        assert!(!names.insert(StaticPropertyName::Str(JSStr::from("𐀀"))));
        assert_eq!(names.len(), 8);
    }

    #[test]
    fn numbers_are_named_by_f64_display() {
        for value in [1e21, 1e-7, -f64::MAX, -5e-324, -f64::MIN_POSITIVE, f64::NAN] {
            let text = value.to_string();
            assert_eq!(StaticPropertyName::Number(value).to_string(), text);
            assert_eq!(StaticPropertyName::Number(value), StaticPropertyName::from(text.as_str()));
        }
    }
}
