use oxc_macros::declare_oxc_lint;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::rule::{DefaultRuleConfig, Rule};

#[derive(Debug, Default, Clone, Deserialize)]
pub struct PreferAwaitToCallbacks(Box<PreferAwaitToCallbacksConfig>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AllowedMethod {
    /// Name of the receiver's class, e.g. `Hono`.
    pub class: String,
    /// Name of the method on that class whose callback is allowed, e.g. `onError`.
    pub method: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct PreferAwaitToCallbacksConfig {
    /// Class methods whose last-argument callback is a framework hook, not an IO callback.
    pub allowed: Vec<AllowedMethod>,
}

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Type-aware port of `promise/prefer-await-to-callbacks` that can allow a callback per class method.
    ///
    /// ### Why is this bad?
    ///
    /// Callbacks lead to nested control flow and awkward error handling that `async`/`await` avoids.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// doSomething(arg, (err) => {});
    /// function doSomethingElse(cb) {}
    /// ```
    ///
    /// Examples of **correct** code for this rule, with `{ "allowed": [{ "class": "Hono", "method": "onError" }] }`:
    /// ```ts
    /// await doSomething(arg);
    /// new Hono().onError((error, c) => c.text(error.message));
    /// ```
    PreferAwaitToCallbacks(tsgolint),
    custom_tsgolint_oxlint_rules,
    style,
    config = PreferAwaitToCallbacksConfig,
    version = "1.86.0",
    short_description = "Prefer `async`/`await` over callbacks, allowing configured class methods.",
);

impl Rule for PreferAwaitToCallbacks {
    fn from_configuration(value: serde_json::Value) -> Result<Self, serde_json::error::Error> {
        DefaultRuleConfig::<Self>::from_value(value).map(DefaultRuleConfig::into_inner)
    }

    fn to_configuration(&self) -> Option<Result<serde_json::Value, serde_json::Error>> {
        Some(serde_json::to_value(&*self.0))
    }
}
