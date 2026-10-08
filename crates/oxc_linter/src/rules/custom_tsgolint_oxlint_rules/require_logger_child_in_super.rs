use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct RequireLoggerChildInSuper;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Require an `ADisposable` subclass to pass `logger: logger.child({ name: X.name })`
    /// to `super(...)`.
    ///
    /// ### Why is this bad?
    ///
    /// Without the child binding, every line the instance logs is labelled by whoever
    /// constructed it, so a stack of modules all log under the outermost one.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// class Service extends ADisposable {
    ///   constructor({ logger }: { readonly logger: Logger }) {
    ///     super({ logger });
    ///   }
    /// }
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// class Service extends ADisposable {
    ///   constructor({ logger }: { readonly logger: Logger }) {
    ///     super({ logger: logger.child({ name: Service.name }) });
    ///   }
    /// }
    /// ```
    RequireLoggerChildInSuper(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Require `logger: logger.child({ name: X.name })` in an ADisposable's `super(...)` call.",
);

impl Rule for RequireLoggerChildInSuper {}
