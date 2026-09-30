use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoProtected;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows TypeScript's `protected` modifier on class members.
    ///
    /// ### Why is this bad?
    ///
    /// A protected member is shared mutable surface between a class and everything that ever extends it, which makes it impossible to change without auditing every subclass.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// class A { protected value = 1; }
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// class A { #value = 1; }
    /// ```
    NoProtected(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow the `protected` modifier on class members.",
);

impl Rule for NoProtected {}
