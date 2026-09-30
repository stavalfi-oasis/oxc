use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct ClassNameMatchesFilename;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Requires the top-level class declaration in a file to carry the PascalCase form of the file name.
    ///
    /// ### Why is this bad?
    ///
    /// One class per file, named after the file, means an import site already names the file the symbol comes from. A mismatch forces a jump to definition to find out.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// // gh-gateway.service.ts
/// export class Gateway {}
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// // gh-gateway.service.ts
/// export class GhGatewayService {}
    /// ```
    ClassNameMatchesFilename(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Require a file's top-level class to be named after the file.",
);

impl Rule for ClassNameMatchesFilename {}
