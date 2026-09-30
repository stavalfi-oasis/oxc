use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct RequireAccessModifiers;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Requires every class member that is not a `#` private to declare `public`, `private`, or `protected`.
    ///
    /// ### Why is this bad?
    ///
    /// Without a modifier the member's visibility is implied rather than stated, so a reader cannot tell a deliberate part of the class's surface from an internal that was never marked.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// class A { value = 1; }
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// class A { public value = 1; }
    /// ```
    RequireAccessModifiers(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Require an explicit access modifier on class members.",
);

impl Rule for RequireAccessModifiers {}
