use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoArrayLengthAssignment;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows assigning to `.length`, including `arr['length']` and compound
    /// forms like `arr.length -= 1`.
    ///
    /// ### Why is this bad?
    ///
    /// `arr.length = 0` is an in-place truncation: every other holder of that
    /// same array sees its contents vanish. Handing out a fresh array instead
    /// keeps the old one intact for whoever is still reading it, and makes the
    /// reassignment visible at the field rather than hidden behind a property
    /// write.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// declare const workers: string[];
    /// workers.length = 0;
    /// workers['length'] = 0;
    /// workers.length -= 1;
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// let workers: string[] = [];
    /// workers = [];
    ///
    /// const count = workers.length;
    /// ```
    NoArrayLengthAssignment(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow assigning to `.length` instead of assigning a fresh array.",
);

impl Rule for NoArrayLengthAssignment {}
