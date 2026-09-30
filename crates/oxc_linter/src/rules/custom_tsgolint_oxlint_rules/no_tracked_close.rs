use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoTrackedClose;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows `this.track(this.close())`, where the tracked promise is the one that drains the tracking set.
    ///
    /// ### Why is this bad?
    ///
    /// `close()` waits for the in-flight set to empty. Tracking it puts it in the set it is waiting on, so the wait never finishes.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// await this.track(this.close());
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// await this.close();
    /// ```
    NoTrackedClose(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow passing `close()` to `track()`.",
);

impl Rule for NoTrackedClose {}
