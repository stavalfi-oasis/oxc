use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct RequireAsyncDisposable;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Require services to extend ADisposable and close last in `[Symbol.asyncDispose]`.
    ///
    /// ### Why is this bad?
    ///
    /// `close()` aborts the AbortController and drains in-flight work. Disposing
    /// without it, or before the rest of teardown, leaves work running.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// class Service extends ADisposable {
    ///   async [Symbol.asyncDispose](): Promise<void> { await work(); }
    /// }
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// class Service extends ADisposable {
    ///   async [Symbol.asyncDispose](): Promise<void> { await this.close(); }
    /// }
    /// ```
    RequireAsyncDisposable(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Require services to extend ADisposable and close last in `[Symbol.asyncDispose]`.",
);

impl Rule for RequireAsyncDisposable {}
