use oxc_macros::declare_oxc_lint;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::rule::{DefaultRuleConfig, Rule};

#[derive(Debug, Default, Clone, Deserialize)]
pub struct RequireAbortSignal(Box<RequireAbortSignalConfig>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CancellableCall {
    /// Name of the called function or method.
    pub name: String,
    /// Property the options object must carry, e.g. `signal` or `abortSignal`.
    pub option_name: String,
    /// Argument index of the options object.
    pub options_index: usize,
    /// Only report when the resolved signature really declares `optionName` at
    /// `optionsIndex`. Set this for a name other APIs also use, like `execute`.
    #[serde(default)]
    pub require_declared_option: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct RequireAbortSignalConfig {
    /// The call shapes that must be cancellable. The rule watches nothing else.
    pub calls: Vec<CancellableCall>,
}

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Require an abort signal on the configured calls whose signature accepts one.
    ///
    /// ### Why is this bad?
    ///
    /// Without a signal the call cannot be cancelled, so shutdown waits on work nobody
    /// wants any more.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// await fetch(url);
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// await fetch(url, { signal });
    /// ```
    RequireAbortSignal(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    config = RequireAbortSignalConfig,
    version = "1.86.0",
    short_description = "Require an abort signal on calls whose signature accepts one.",
);

impl Rule for RequireAbortSignal {
    fn from_configuration(value: serde_json::Value) -> Result<Self, serde_json::error::Error> {
        DefaultRuleConfig::<Self>::from_value(value).map(DefaultRuleConfig::into_inner)
    }

    fn to_configuration(&self) -> Option<Result<serde_json::Value, serde_json::Error>> {
        Some(serde_json::to_value(&*self.0))
    }
}
