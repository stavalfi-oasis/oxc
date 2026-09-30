use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoComments;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows every comment except lint control, type-checker directives, triple-slash references, and the shebang.
    ///
    /// ### Why is this bad?
    ///
    /// A comment is not checked by anything, so it drifts from the code it describes. What it says belongs in a name the code already reads.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// // adds the two numbers
/// const sum = a + b;
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// const sum = a + b;
    /// ```
    NoComments(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow comments other than tool directives.",
);

impl Rule for NoComments {}
