use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct RequireZodDescribe;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Require `.describe(...)` on every root zod schema passed to `z.compile(...)`.
    ///
    /// ### Why is this bad?
    ///
    /// A schema without a description gives no context in generated JSON schemas,
    /// OpenAPI docs, or validation errors.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// const schema = z.compile(z.string());
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// const schema = z.compile(z.string().describe("user name"));
    /// ```
    RequireZodDescribe(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Require `.describe(...)` on every root zod schema.",
);

impl Rule for RequireZodDescribe {}
