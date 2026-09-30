use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoGlobalFunctions;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows function declarations, and consts initialized with a function, at the top level of a module.
    ///
    /// ### Why is this bad?
    ///
    /// A module whose exports are loose functions has no single name to import. Collecting them as static members of one class gives the file one export and one thing to mock.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// export function run(): void {}
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// export class Runner {
///   public static run(): void {}
/// }
    /// ```
    NoGlobalFunctions(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow functions declared at module scope.",
);

impl Rule for NoGlobalFunctions {}
