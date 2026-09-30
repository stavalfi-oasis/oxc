use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct LoggerNameMatchesClass;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Requires `logger.child({ name })` inside a constructor to bind `name` to the enclosing class's `name`.
    ///
    /// ### Why is this bad?
    ///
    /// The `name` binding is what a log line is filtered by. A hand-written string drifts from the class as soon as the class is renamed, and the logs then point at code that no longer exists.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// class GatewayService {
///   public constructor() { logger.child({ name: "gateway" }); }
/// }
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// class GatewayService {
///   public constructor() { logger.child({ name: GatewayService.name }); }
/// }
    /// ```
    LoggerNameMatchesClass(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Require a child logger's `name` binding to be its owning class.",
);

impl Rule for LoggerNameMatchesClass {}
