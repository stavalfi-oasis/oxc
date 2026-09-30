use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoStringError;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows `String(value)` when `value` is not already something `String()`
    /// renders faithfully.
    ///
    /// ### Why is this bad?
    ///
    /// `String()` on an object — an `Error`, a class instance, a plain object —
    /// produces `"[object Object]"`, which silently destroys the information the
    /// caller wanted to log.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// declare const error: Error;
    /// String(error);
    ///
    /// declare const payload: { code: number };
    /// String(payload);
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// declare const value: string;
    /// String(value);
    ///
    /// declare const error: Error;
    /// serializeError(error).message;
    /// ```
    NoStringError(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow `String()` on values that are not already stringable.",
);

impl Rule for NoStringError {}
