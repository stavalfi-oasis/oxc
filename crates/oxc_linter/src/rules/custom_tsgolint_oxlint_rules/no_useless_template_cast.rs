use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoUselessTemplateCast;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows `String(value)` and `value.toString()` inside a template
    /// literal interpolation when the value's type already converts to the same
    /// string on its own.
    ///
    /// ### Why is this bad?
    ///
    /// A template span performs ToString on its expression, so the cast is dead
    /// code that only hides the value behind a function call.
    ///
    /// Symbols are exempt — `${sym}` throws a TypeError while `String(sym)`
    /// works — as is `toString(radix)`, which changes the output, and `?.`,
    /// which can yield `undefined` instead of `"undefined"`. Objects are left to
    /// `no-string-error`.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// declare const index: number;
    /// const id = `it-${String(index)}`;
    /// const other = `it-${index.toString()}`;
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// declare const index: number;
    /// const id = `it-${index}`;
    ///
    /// declare const sym: symbol;
    /// const name = `it-${String(sym)}`;
    /// ```
    NoUselessTemplateCast(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow a redundant string cast inside a template literal.",
);

impl Rule for NoUselessTemplateCast {}
