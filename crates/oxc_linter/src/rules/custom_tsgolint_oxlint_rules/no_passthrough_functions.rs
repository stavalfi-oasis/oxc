use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoPassthroughFunctions;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallow functions that only forward their arguments to another local function.
    ///
    /// ### Why is this bad?
    ///
    /// A wrapper that adds nothing is an extra name, an extra stack frame and an extra
    /// place to keep in sync.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// function wrapper(value: number) { return target(value); }
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// // call target(value) directly
    /// ```
    NoPassthroughFunctions(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow functions that only forward their arguments to another local function.",
);

impl Rule for NoPassthroughFunctions {}
