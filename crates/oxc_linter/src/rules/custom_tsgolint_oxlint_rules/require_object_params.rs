use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct RequireObjectParams;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Requires a function, method, or constructor to declare at most one parameter besides `this`.
    ///
    /// ### Why is this bad?
    ///
    /// Positional parameters are unnamed at the call site, so two arguments of the same type can be swapped without any error. An object parameter names each one where it is passed.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// function move(x: number, y: number): void {}
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// function move({ x, y }: { x: number; y: number }): void {}
    /// ```
    RequireObjectParams(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Require a single object parameter instead of several positional ones.",
);

impl Rule for RequireObjectParams {}
