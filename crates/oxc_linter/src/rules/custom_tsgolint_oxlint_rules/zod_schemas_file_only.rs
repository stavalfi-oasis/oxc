use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct ZodSchemasFileOnly;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Require zod schemas to live in the package's single `zod-schemas.ts`.
    ///
    /// ### Why is this bad?
    ///
    /// Schemas scattered across a package get duplicated and drift. One file per
    /// package — identified by its tsconfig.json — keeps them findable.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// // src/server.ts
    /// const body = z.object({});
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// // src/zod-schemas.ts
    /// export const body = z.object({});
    /// ```
    ZodSchemasFileOnly(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Require zod schemas to live in the package's single `zod-schemas.ts`.",
);

impl Rule for ZodSchemasFileOnly {}
