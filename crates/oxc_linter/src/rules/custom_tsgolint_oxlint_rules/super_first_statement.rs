use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct SuperFirstStatement;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Require `super()` to be the first statement of a constructor.
    ///
    /// ### Why is this bad?
    ///
    /// Until `super()` has run, `this` does not exist, so anything above it cannot
    /// reach the base class's state.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// class Service extends Base {
    ///   constructor() {
    ///     const value = work();
    ///     super({ value });
    ///   }
    /// }
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// class Service extends Base {
    ///   constructor() {
    ///     super({ value: work() });
    ///   }
    /// }
    /// ```
    SuperFirstStatement(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Require `super()` to be the first statement of a constructor.",
);

impl Rule for SuperFirstStatement {}
