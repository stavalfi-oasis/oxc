use oxc_macros::declare_oxc_lint;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::rule::{DefaultRuleConfig, Rule};

#[derive(Debug, Default, Clone, Deserialize)]
pub struct NoCurl(Box<NoCurlConfig>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BannedCommandGroup {
    /// Command names banned as the first argument of an exec helper.
    pub commands: Vec<String>,
    /// Reported in place of the rule's own message when one of `commands` is found.
    pub message: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct NoCurlConfig {
    /// Names of the functions that spawn a process. Defaults to `execFile`, `execFileAsync`, and `spawn`.
    pub exec_functions: Vec<String>,
    /// Groups of banned commands, each carrying the message to report for them.
    pub groups: Vec<BannedCommandGroup>,
}

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
    config = NoCurlConfig,
    version = "1.86.0",
    short_description = "Disallow spawning configured commands through exec helpers.",
);

impl Rule for NoCurl {
    fn from_configuration(value: serde_json::Value) -> Result<Self, serde_json::error::Error> {
        DefaultRuleConfig::<Self>::from_value(value).map(DefaultRuleConfig::into_inner)
    }

    fn to_configuration(&self) -> Option<Result<serde_json::Value, serde_json::Error>> {
        Some(serde_json::to_value(&*self.0))
    }
}
