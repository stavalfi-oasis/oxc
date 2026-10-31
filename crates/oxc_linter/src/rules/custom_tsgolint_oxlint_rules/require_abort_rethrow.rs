use oxc_macros::declare_oxc_lint;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::rule::{DefaultRuleConfig, Rule};

#[derive(Debug, Default, Clone, Deserialize)]
pub struct RequireAbortRethrow(Box<RequireAbortRethrowConfig>);

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct RequireAbortRethrowConfig {
    /// File, relative to the working directory, that the autofix imports `isAbortError` from by a relative path. Wins over `importSource`.
    pub import_file: String,
    /// Module the autofix imports `isAbortError` from when the file does not have it yet. Defaults to `@poc/libs/utils.ts`.
    pub import_source: String,
}

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Requires every catch block to start with `if (isAbortError(error)) { throw error; }`.
    ///
    /// ### Why is this bad?
    ///
    /// A catch block that handles every error also handles an abort, so a cancelled operation keeps running as if nothing happened.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// try { await work(); } catch (error) { logger.warn(error); }
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// try { await work(); } catch (error) { if (isAbortError(error)) { throw error; } logger.warn(error); }
    /// ```
    RequireAbortRethrow(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    config = RequireAbortRethrowConfig,
    version = "1.86.0",
    short_description = "Require catch blocks to rethrow abort errors first.",
);

impl Rule for RequireAbortRethrow {
    fn from_configuration(value: serde_json::Value) -> Result<Self, serde_json::error::Error> {
        DefaultRuleConfig::<Self>::from_value(value).map(DefaultRuleConfig::into_inner)
    }

    fn to_configuration(&self) -> Option<Result<serde_json::Value, serde_json::Error>> {
        Some(serde_json::to_value(&*self.0))
    }
}
