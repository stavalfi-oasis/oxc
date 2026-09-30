use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoAnonymousFunctions;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows calling a function expression or arrow function in place.
    ///
    /// ### Why is this bad?
    ///
    /// An IIFE has no name, so it never appears in a stack trace and cannot be tested on its own.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// (() => 1)();
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// class Value {
///   public static make(): number { return 1; }
/// }
/// Value.make();
    /// ```
    NoAnonymousFunctions(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow immediately-invoked anonymous functions.",
);

impl Rule for NoAnonymousFunctions {}
