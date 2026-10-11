use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Requires the key of `process.env[...]` / `env[...]` to be a plain string literal — no template literal, no variable.
    ///
    /// ### Why is this bad?
    ///
    /// A template literal or a variable hides the env var name from plain text search, so `find . -type f -name '*.ts' -exec grep -l MY_VAR {} +` no longer lists every place the env var is used.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// const port = process.env[name];
    /// const host = process.env[`${prefix}_HOST`];
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// const port = process.env["PORT"];
    /// const host = process.env.HOST;
    /// ```
    RequireConstantEnvKey(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Require a constant string key on every env var access.",
);

#[derive(Debug, Default, Clone)]
pub struct RequireConstantEnvKey;

impl Rule for RequireConstantEnvKey {}
