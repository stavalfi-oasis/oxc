use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoStringRaw;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows the `String.raw` tagged template.
    ///
    /// ### Why is this bad?
    ///
    /// `String.raw` changes what a backslash means inside the template, so the same-looking string means two different things depending on a tag several characters away.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// const pattern = String.raw`\d+`;
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// const pattern = `\\d+`;
    /// ```
    NoStringRaw(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow `String.raw`.",
);

impl Rule for NoStringRaw {}
