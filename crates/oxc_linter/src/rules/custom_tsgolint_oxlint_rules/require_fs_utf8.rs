use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct RequireFsUtf8;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Require an explicit `utf8` encoding on node `fs` read/write calls.
    ///
    /// ### Why is this bad?
    ///
    /// Without an encoding these return a `Buffer`, or write one. Callers then get a
    /// `Buffer` where they expected a string.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// await readFile('a.txt');
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// await readFile('a.txt', 'utf8');
    /// ```
    RequireFsUtf8(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Require an explicit `utf8` encoding on node `fs` read/write calls.",
);

impl Rule for RequireFsUtf8 {}
