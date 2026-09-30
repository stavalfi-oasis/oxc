use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoSingleUseConst;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows a non-exported const initialized with a string, number, or template literal when it is read exactly once.
    ///
    /// ### Why is this bad?
    ///
    /// A constant read once adds a name and a jump without adding meaning; the value at the single use site says the same thing in place.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// const label = "x";
/// send(label);
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// send("x");
    /// ```
    NoSingleUseConst(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow a literal constant that is read exactly once.",
);

impl Rule for NoSingleUseConst {}
