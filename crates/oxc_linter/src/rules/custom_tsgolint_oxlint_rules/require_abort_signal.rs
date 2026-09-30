use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct RequireAbortSignal;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Require an abort signal on calls whose signature accepts one.
    ///
    /// ### Why is this bad?
    ///
    /// Without a signal the call cannot be cancelled, so shutdown waits on work nobody
    /// wants any more.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// await fetch(url);
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// await fetch(url, { signal });
    /// ```
    RequireAbortSignal(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Require an abort signal on calls whose signature accepts one.",
);

impl Rule for RequireAbortSignal {}
