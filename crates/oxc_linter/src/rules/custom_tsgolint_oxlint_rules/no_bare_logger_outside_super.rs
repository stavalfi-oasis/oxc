use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoBareLoggerOutsideSuper;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Inside an `ADisposable` constructor, allow the `logger` parameter only within the
    /// `super(...)` call; everywhere else use `this.logger`.
    ///
    /// ### Why is this bad?
    ///
    /// The logger `super()` built carries this class's bindings and the parameter does
    /// not, so a module handed the parameter logs under whoever constructed the instance.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// class Service extends ADisposable {
    ///   constructor({ logger }: { readonly logger: Logger }) {
    ///     super({ logger: logger.child({ name: Service.name }) });
    ///     this.#inner = new Inner({ logger });
    ///   }
    /// }
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// class Service extends ADisposable {
    ///   constructor({ logger }: { readonly logger: Logger }) {
    ///     super({ logger: logger.child({ name: Service.name }) });
    ///     this.#inner = new Inner({ logger: this.logger });
    ///   }
    /// }
    /// ```
    NoBareLoggerOutsideSuper(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Allow an ADisposable constructor's `logger` parameter only inside `super(...)`.",
);

impl Rule for NoBareLoggerOutsideSuper {}
