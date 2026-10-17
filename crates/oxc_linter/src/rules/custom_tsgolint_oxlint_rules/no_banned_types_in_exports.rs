use oxc_macros::declare_oxc_lint;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::rule::{DefaultRuleConfig, Rule};

#[derive(Debug, Default, Clone, Deserialize)]
pub struct NoBannedTypesInExports(Box<NoBannedTypesInExportsConfig>);

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct NoBannedTypesInExportsConfig {
    /// Type names that must not be referenced inside an exported type alias or interface.
    pub types: Vec<String>,
}

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows the configured type names from being referenced inside an exported type alias or interface.
    ///
    /// ### Why is this bad?
    ///
    /// An exported type is a module's public surface. Wrapping it in a heavy utility type such as `DeepReadonly` leaks that wrapper into every consumer.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule with `{ "types": ["DeepReadonly"] }`:
    /// ```ts
    /// export interface Change { readonly value: DeepReadonly<Value> }
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// export interface Change { readonly value: Value }
    /// export function apply(change: DeepReadonly<Change>): void {}
    /// ```
    NoBannedTypesInExports(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    config = NoBannedTypesInExportsConfig,
    version = "1.86.0",
    short_description = "Disallow configured types inside exported type aliases and interfaces.",
);

impl Rule for NoBannedTypesInExports {
    fn from_configuration(value: serde_json::Value) -> Result<Self, serde_json::error::Error> {
        DefaultRuleConfig::<Self>::from_value(value).map(DefaultRuleConfig::into_inner)
    }

    fn to_configuration(&self) -> Option<Result<serde_json::Value, serde_json::Error>> {
        Some(serde_json::to_value(&*self.0))
    }
}
