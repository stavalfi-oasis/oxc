use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoBannedWords;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows whole words listed in the rule's configured groups from appearing in any string or template literal.
    ///
    /// ### Why is this bad?
    ///
    /// User-facing strings carry product vocabulary. A banned word is one the organization has decided not to ship, and a literal is where it slips through.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// const label = "open the app";
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// const label = "open the Oasis Platform";
    /// ```
    NoBannedWords(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow configured words in string and template literals.",
);

impl Rule for NoBannedWords {}
