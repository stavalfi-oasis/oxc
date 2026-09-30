use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoVoidPromise;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallow `void` on a promise.
    ///
    /// ### Why is this bad?
    ///
    /// `void promise` discards the promise without handling rejection, so a failure
    /// becomes an unhandled rejection instead of an error anyone sees.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// declare const promise: Promise<number>;
    /// void promise;
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// declare const promise: Promise<number>;
    /// await promise;
    /// ```
    NoVoidPromise(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow `void` on a promise.",
);

impl Rule for NoVoidPromise {}
