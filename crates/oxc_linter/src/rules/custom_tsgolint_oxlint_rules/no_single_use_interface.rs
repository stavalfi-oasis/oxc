use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoSingleUseInterface;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallow local interfaces referenced exactly once.
    ///
    /// ### Why is this bad?
    ///
    /// A name used in one place adds indirection without adding meaning — the reader
    /// has to jump to learn what is already at hand.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// interface Options { port: number }
    /// declare const a: Options;
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// declare const a: { port: number };
    /// ```
    NoSingleUseInterface(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow local interfaces referenced exactly once.",
);

impl Rule for NoSingleUseInterface {}
