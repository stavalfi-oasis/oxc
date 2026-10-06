use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct RequireAbortSignalInWhile;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Requires every `while` and `do ... while` condition to test an `AbortSignal` with `!<signal>.aborted`. The receiver's type decides, so an unrelated object carrying an `aborted` flag does not satisfy the rule.
    ///
    /// ### Why is this bad?
    ///
    /// A loop that only tests its own progress keeps running after shutdown starts, so Ctrl-C and a cancelled request both wait for it to finish on its own. Testing the signal in the condition ends the loop at the next iteration without changing what the loop does.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// while (taken < pending.length) { taken += 1; }
    /// while (!job.aborted) { taken += 1; }
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// while (!signal.aborted && taken < pending.length) { taken += 1; }
    /// ```
    RequireAbortSignalInWhile(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Require `!<signal>.aborted` in every `while` condition.",
);

impl Rule for RequireAbortSignalInWhile {}
