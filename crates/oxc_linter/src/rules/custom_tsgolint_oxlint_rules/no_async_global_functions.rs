use oxc_macros::declare_oxc_lint;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::rule::{DefaultRuleConfig, Rule};

#[derive(Debug, Default, Clone, Deserialize)]
pub struct NoAsyncGlobalFunctions(Box<NoAsyncGlobalFunctionsConfig>);

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct NoAsyncGlobalFunctionsConfig {
    /// Function names that may be called directly. Matched against both the called name
    /// (`readFile`) and the whole callee text (`fs.readFile`).
    pub exclude_functions: Vec<String>,
    /// Module specifiers whose every export may be called directly, such as `node:fs/promises`.
    /// Matched exactly against the import's specifier.
    pub exclude_modules: Vec<String>,
}

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows calling a function declared outside this codebase — an ambient global or an import from a package or node builtin — when the call returns a promise or is handed a callback.
    ///
    /// ### Why is this bad?
    ///
    /// A promise nothing holds outlives the scope that started it, and a callback handed to a global outlives it too. Neither has an owner that can cancel it, so shutdown cannot wait for the work or stop it. Wrapping the call in a class gives the pending work an owner that disposal can reach.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// setTimeout(() => poll(), 1000);
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// class Poller extends ADisposable {
///   public start(): void {
///     this.track(this.timers.delay(1000).then(() => this.poll()));
///   }
/// }
    /// ```
    NoAsyncGlobalFunctions(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    config = NoAsyncGlobalFunctionsConfig,
    version = "1.86.0",
    short_description = "Disallow calling promise-returning or callback-taking functions declared outside this codebase.",
);

impl Rule for NoAsyncGlobalFunctions {
    fn from_configuration(value: serde_json::Value) -> Result<Self, serde_json::error::Error> {
        DefaultRuleConfig::<Self>::from_value(value).map(DefaultRuleConfig::into_inner)
    }

    fn to_configuration(&self) -> Option<Result<serde_json::Value, serde_json::Error>> {
        Some(serde_json::to_value(&*self.0))
    }
}
