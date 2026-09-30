use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoImportSideEffects;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows calls, constructions, awaits and static property initializers evaluated at module scope, and side-effect-only imports.
    ///
    /// ### Why is this bad?
    ///
    /// Import-time work runs in whatever order the module graph resolves, before any configuration is loaded and with no way for a caller to opt out. It also makes the module impossible to import from a test without paying for it.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// const client = new Client();
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// export class Clients {
///   public static make(): Client { return new Client(); }
/// }
    /// ```
    NoImportSideEffects(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow work that runs when a module is imported.",
);

impl Rule for NoImportSideEffects {}
