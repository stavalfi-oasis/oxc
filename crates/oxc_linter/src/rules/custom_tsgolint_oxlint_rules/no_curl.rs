use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoCurl;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows passing a configured command name as the first argument to an exec or spawn helper.
    ///
    /// ### Why is this bad?
    ///
    /// Shelling out to a command bypasses every type the codebase has for that operation, and turns a build-time error into a runtime one on a machine that happens not to have the binary.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// execFile("curl", ["https://example.com"]);
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// await client.get("/resource");
    /// ```
    NoCurl(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow spawning configured commands through exec helpers.",
);

impl Rule for NoCurl {}
