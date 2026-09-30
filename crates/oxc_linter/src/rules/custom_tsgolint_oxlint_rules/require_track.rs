use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct RequireTrack;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Require promises inside an ADisposable to go through `this.track(...)`.
    ///
    /// ### Why is this bad?
    ///
    /// A promise that is not in the in-flight set is not waited for by `close()`, so
    /// shutdown can race it. This includes promises that are never awaited, which a
    /// syntactic rule cannot see at all.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// class Service extends ADisposable {
    ///   run(): void { work(); }
    /// }
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// class Service extends ADisposable {
    ///   async run(): Promise<void> { await this.track(work()); }
    /// }
    /// ```
    RequireTrack(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Require promises inside an ADisposable to go through `this.track(...)`.",
);

impl Rule for RequireTrack {}
