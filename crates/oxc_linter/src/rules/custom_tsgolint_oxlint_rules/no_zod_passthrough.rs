use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoZodPassthrough;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallow `ZodObject.passthrough()`.
    ///
    /// ### Why is this bad?
    ///
    /// `passthrough()` is deprecated in zod v4.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// z.object({}).passthrough();
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// z.looseObject({});
    /// ```
    NoZodPassthrough(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow `ZodObject.passthrough()`.",
);

impl Rule for NoZodPassthrough {}
