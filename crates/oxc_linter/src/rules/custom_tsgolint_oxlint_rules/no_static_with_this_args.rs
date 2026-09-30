use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoStaticWithThisArgs;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallow statics that every caller feeds the same instance state.
    ///
    /// ### Why is this bad?
    ///
    /// A static that always needs instance state is an instance method whose receiver
    /// is passed by hand.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// class Service {
    ///   run() { return Service.build(this.#port); }
    /// }
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// class Service {
    ///   run() { return this.build(); }
    /// }
    /// ```
    NoStaticWithThisArgs(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow statics that every caller feeds the same instance state.",
);

impl Rule for NoStaticWithThisArgs {}
