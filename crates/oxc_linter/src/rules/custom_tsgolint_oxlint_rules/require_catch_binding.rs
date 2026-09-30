use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct RequireCatchBinding;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Requires `catch (error)` rather than the optional-catch-binding form.
    ///
    /// ### Why is this bad?
    ///
    /// An unbound `catch` discards the failure before anything can log or inspect it, which turns a diagnosable error into silence.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// try { run(); } catch {}
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// try { run(); } catch (error) { logger.error({ error }); }
    /// ```
    RequireCatchBinding(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Require a binding on every `catch` clause.",
);

impl Rule for RequireCatchBinding {}
