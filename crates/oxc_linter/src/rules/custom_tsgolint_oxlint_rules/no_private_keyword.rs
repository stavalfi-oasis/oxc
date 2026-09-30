use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoPrivateKeyword;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows TypeScript's `private` modifier outside constructor parameter properties.
    ///
    /// ### Why is this bad?
    ///
    /// `private` is erased at compile time, so the field is still reachable at runtime and still shows up in a serialized object. A `#` field is private in the language itself.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// class A { private value = 1; }
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// class A { #value = 1; }
    /// ```
    NoPrivateKeyword(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow the `private` modifier on class members.",
);

impl Rule for NoPrivateKeyword {}
