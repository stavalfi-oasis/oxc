use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoProcessStreamWrite;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallow `process.stdout.write` / `process.stderr.write`.
    ///
    /// ### Why is this bad?
    ///
    /// `process.stdout.write` takes a raw chunk and does no formatting, so values
    /// reach the terminal unrendered. `console.log` / `console.error` format them.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// declare const process: NodeJS.Process;
    /// process.stdout.write('hello');
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// console.log('hello');
    /// ```
    NoProcessStreamWrite(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow `process.stdout.write` / `process.stderr.write`.",
);

impl Rule for NoProcessStreamWrite {}
