use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoInfiniteLoop;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows a loop whose header states no stop condition at all.
    ///
    /// ### Why is this bad?
    ///
    /// A loop with no condition cannot be stopped from the outside, so Ctrl-C and a shutdown signal both hang. Looping on an abort signal keeps the same behaviour and stays interruptible.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// for (;;) { poll(); }
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// while (!signal.aborted) { poll(); }
    /// ```
    NoInfiniteLoop(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow `for (;;)` and `while (true)`.",
);

impl Rule for NoInfiniteLoop {}
