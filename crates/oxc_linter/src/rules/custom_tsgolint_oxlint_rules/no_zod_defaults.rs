use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoZodDefaults;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallow `.default()` / `.prefault()` on a zod schema.
    ///
    /// ### Why is this bad?
    ///
    /// A schema that invents a value hides a missing input. The program starts with a
    /// silently wrong config instead of failing loudly.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// z.string().default('dev');
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// z.string();
    /// ```
    NoZodDefaults(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow `.default()` / `.prefault()` on a zod schema.",
);

impl Rule for NoZodDefaults {}
