use oxc_macros::declare_oxc_lint;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::rule::{DefaultRuleConfig, Rule};

#[derive(Debug, Default, Clone, Deserialize)]
pub struct NewOnlyInConstructor(Box<NewOnlyInConstructorConfig>);

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct NewOnlyInConstructorConfig {
    /// Class names that may be constructed anywhere, by the name written after `new`. Empty by default.
    pub allow: Vec<String>,
}

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows `new` inside a class outside its constructor or a property initializer.
    ///
    /// ### Why is this bad?
    ///
    /// A field built in `start()` is undefined until `start()` runs, so a disposal that lands while the owner is still starting reads nothing and silently skips the cleanup it was supposed to do.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// class A {
    ///   #dep?: Dep;
    ///   start(): void {
    ///     this.#dep = new Dep();
    ///   }
    /// }
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// class A {
    ///   readonly #dep = new Dep();
    ///   start(url: string): void {
    ///     this.#dep.run(url);
    ///   }
    /// }
    /// ```
    NewOnlyInConstructor(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    config = NewOnlyInConstructorConfig,
    version = "1.86.0",
    short_description = "Disallow constructing a class outside a constructor or property initializer.",
);

impl Rule for NewOnlyInConstructor {
    fn from_configuration(value: serde_json::Value) -> Result<Self, serde_json::error::Error> {
        DefaultRuleConfig::<Self>::from_value(value).map(DefaultRuleConfig::into_inner)
    }

    fn to_configuration(&self) -> Option<Result<serde_json::Value, serde_json::Error>> {
        Some(serde_json::to_value(&*self.0))
    }
}
