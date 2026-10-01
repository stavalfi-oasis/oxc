use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct RequireAsyncQueue;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Require a class extending ADisposable to accept `asyncQueue: AsyncQueue`
    /// in its constructor options and pass it to `super()`.
    ///
    /// ### Why is this bad?
    ///
    /// An app owns exactly one AsyncQueue, which is what bounds its concurrency.
    /// A class that builds its own queue, or none at all, runs outside that bound.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// class Service extends ADisposable {
    ///   constructor({ logger }: { readonly logger: Logger }) { super(); }
    /// }
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// class Service extends ADisposable {
    ///   constructor({ asyncQueue, logger }: { readonly asyncQueue: AsyncQueue; readonly logger: Logger }) {
    ///     super({ asyncQueue });
    ///   }
    /// }
    /// ```
    RequireAsyncQueue(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Require a class extending ADisposable to accept `asyncQueue: AsyncQueue` in its constructor options.",
);

impl Rule for RequireAsyncQueue {}
