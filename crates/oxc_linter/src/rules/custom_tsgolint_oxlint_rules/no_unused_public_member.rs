use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoUnusedPublicMember;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows a public or static method that nothing outside its own class
    /// references anywhere in the program.
    ///
    /// ### Why is this bad?
    ///
    /// A public member is a promise to the rest of the codebase. One that only
    /// its own class calls makes the class look bigger than it is, and every
    /// reader has to check whether changing it breaks something elsewhere. A `#`
    /// member answers that question by existing.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// export class Service {
    ///   public helper(): number { return 1; }
    ///   public run(): number { return this.helper(); }
    /// }
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// export class Service {
    ///   #helper(): number { return 1; }
    ///   public run(): number { return this.#helper(); }
    /// }
    /// ```
    NoUnusedPublicMember(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow public or static methods that nothing outside the class uses.",
);

impl Rule for NoUnusedPublicMember {}
