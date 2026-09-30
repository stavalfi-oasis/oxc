use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct RequireZodCompile;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Require zod schemas to be AOT-compiled with `z.compile(...)`.
    ///
    /// ### Why is this bad?
    ///
    /// An uncompiled schema is validated interpretively on every parse.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// const schema = z.string();
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// const schema = z.compile(z.string());
    /// ```
    RequireZodCompile(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Require zod schemas to be AOT-compiled with `z.compile(...)`.",
);

impl Rule for RequireZodCompile {}
