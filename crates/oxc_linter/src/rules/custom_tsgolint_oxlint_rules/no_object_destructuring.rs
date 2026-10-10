use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoObjectDestructuring;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows object destructuring of a value that already sits under a name, including the assignment form `({ a } = b)` and a `catch` binding pattern. Destructuring a call result, a parameter and `for (const { x } of xs)` stay allowed.
    ///
    /// ### Why is this bad?
    ///
    /// Copying a member into a bare local drops the owner from every later line, so the reader can no longer tell what `etl` or `logger` belongs to. Reading through the owner keeps that at the use site.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// const { etl } = testHelper;
    /// const { logger } = this;
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// testHelper.etl.repo(repoName);
    /// const { apiUrl, tenantId } = await testHelper.newTenant();
    /// ```
    NoObjectDestructuring(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow object destructuring of an already-named value.",
);

impl Rule for NoObjectDestructuring {}
