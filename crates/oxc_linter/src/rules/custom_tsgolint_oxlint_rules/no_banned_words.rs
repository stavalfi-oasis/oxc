use oxc_macros::declare_oxc_lint;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::rule::{DefaultRuleConfig, Rule};

#[derive(Debug, Default, Clone, Deserialize)]
pub struct NoBannedWords(Box<NoBannedWordsConfig>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BannedWordGroup {
    /// Reported in place of the rule's own message when one of `words` is found.
    pub message: String,
    /// Words to ban. Matched whole-word and case-insensitively.
    pub words: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct NoBannedWordsConfig {
    /// Groups of banned words, each carrying the message to report for them.
    pub groups: Vec<BannedWordGroup>,
}

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows whole words listed in the rule's configured groups from appearing in any string or template literal.
    ///
    /// ### Why is this bad?
    ///
    /// User-facing strings carry product vocabulary. A banned word is one the organization has decided not to ship, and a literal is where it slips through.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// const label = "open the app";
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// const label = "open the Oasis Platform";
    /// ```
    NoBannedWords(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    config = NoBannedWordsConfig,
    version = "1.86.0",
    short_description = "Disallow configured words in string and template literals.",
);

impl Rule for NoBannedWords {
    fn from_configuration(value: serde_json::Value) -> Result<Self, serde_json::error::Error> {
        DefaultRuleConfig::<Self>::from_value(value).map(DefaultRuleConfig::into_inner)
    }

    fn to_configuration(&self) -> Option<Result<serde_json::Value, serde_json::Error>> {
        Some(serde_json::to_value(&*self.0))
    }
}
