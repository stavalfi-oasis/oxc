use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct RequireTestTimeout;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Requires every `it(...)` test from `node:test` to pass an options object
    /// declaring `timeout` as its second argument. `describe(...)` is exempt,
    /// and the value of `timeout` is not validated.
    ///
    /// ### Why is this bad?
    ///
    /// A test with no deadline hangs until the whole run is killed, which turns a
    /// one-test failure into a CI job with no diagnosis and no results for any
    /// other test.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// it('works', async () => {});
    /// it('works', { concurrency: 1 }, async () => {});
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// it('works', { timeout: 5_000 }, async () => {});
    /// describe('group', () => {});
    /// ```
    RequireTestTimeout(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Require a `timeout` option on every `it(...)` test.",
);

impl Rule for RequireTestTimeout {}
