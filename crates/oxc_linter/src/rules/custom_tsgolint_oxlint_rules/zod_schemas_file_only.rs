use oxc_macros::declare_oxc_lint;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::rule::{DefaultRuleConfig, Rule};

#[derive(Debug, Default, Clone, Deserialize)]
pub struct ZodSchemasFileOnly(Box<ZodSchemasFileOnlyConfig>);

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct ZodSchemasFileOnlyConfig {
    /// Globs, relative to a tsconfig.json directory, naming directories that count as
    /// their own package and may hold their own `zod-schemas.ts` (e.g. `src/commands/*`).
    pub sub_package_dirs: Vec<String>,
}

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Require zod schemas to live in the package's single `zod-schemas.ts`.
    ///
    /// ### Why is this bad?
    ///
    /// Schemas scattered across a package get duplicated and drift. One file per
    /// package — identified by its tsconfig.json — keeps them findable.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// // src/server.ts
    /// const body = z.object({});
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// // src/zod-schemas.ts
    /// export const body = z.object({});
    /// ```
    ZodSchemasFileOnly(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    config = ZodSchemasFileOnlyConfig,
    version = "1.86.0",
    short_description = "Require zod schemas to live in the package's single `zod-schemas.ts`.",
);

impl Rule for ZodSchemasFileOnly {
    fn from_configuration(value: serde_json::Value) -> Result<Self, serde_json::error::Error> {
        DefaultRuleConfig::<Self>::from_value(value).map(DefaultRuleConfig::into_inner)
    }

    fn to_configuration(&self) -> Option<Result<serde_json::Value, serde_json::Error>> {
        Some(serde_json::to_value(&*self.0))
    }
}
