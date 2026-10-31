use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct RequireLinkSignal;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// In a class that inherits `linkSignal`, disallows using a method's
    /// `AbortSignal` parameter anywhere except as the argument of
    /// `super.linkSignal({ signal })`.
    ///
    /// ### Why is this bad?
    ///
    /// The caller's signal alone ignores the instance being closed: work keeps
    /// running after `close()` aborted the class signal.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// class A extends ADisposable {
    ///   run({ signal }: { signal: AbortSignal }) {
    ///     return fetch(url, { signal });
    ///   }
    /// }
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// class A extends ADisposable {
    ///   run({ signal }: { signal: AbortSignal }) {
    ///     return fetch(url, { signal: super.linkSignal({ signal }) });
    ///   }
    /// }
    /// ```
    RequireLinkSignal(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Require method signal parameters to go through `super.linkSignal`.",
);

impl Rule for RequireLinkSignal {}
