use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct RequireOsEol;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Requires `EOL` from `node:os` in place of a literal `\n` or `\r\n` escape.
    ///
    /// ### Why is this bad?
    ///
    /// A hardcoded `\n` produces the wrong line ending on Windows, which shows up as a file that reads correctly on the author's machine and as one long line everywhere else.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// const joined = "a\nb";
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// import { EOL } from "node:os";
/// const joined = `a${EOL}b`;
    /// ```
    RequireOsEol(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Require `EOL` instead of a hardcoded newline escape.",
);

impl Rule for RequireOsEol {}
