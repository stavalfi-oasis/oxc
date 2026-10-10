use oxc_macros::declare_oxc_lint;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::rule::{DefaultRuleConfig, Rule};

#[derive(Debug, Default, Clone, Deserialize)]
pub struct NoPrimitiveDefault(Box<NoPrimitiveDefaultConfig>);

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct NoPrimitiveDefaultConfig {
    /// Exact string constants that are part of the codebase's own vocabulary
    /// rather than a stand-in for a missing value, and so may be used as a
    /// fallback. `""` and `0` are always allowed and need no entry.
    pub allow: Vec<String>,
}

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows `??`, `??=`, `||` and `||=` when the right-hand side is a
    /// compile-time string or number constant other than `""` or `0`. The
    /// check reads the type, so a named `const`, and a template literal whose
    /// every substitution is itself known at compile time, are caught just
    /// like a bare literal.
    ///
    /// ### Why is this bad?
    ///
    /// `?? "unknown"` and `|| 8080` invent a value for something that was
    /// genuinely absent, and the invented value is indistinguishable
    /// downstream from a real one — a port that was never configured reads as
    /// 8080, a name that was never set reads as "unknown". The missing case is
    /// information, and substituting a constant throws it away at the point
    /// furthest from anyone who could interpret it.
    ///
    /// A value computed at runtime is exempt: it is the caller's data, not a
    /// value this code made up. So are `{}`, `[]`, `""` and `0` — an empty
    /// container, an empty string and a zero are the identity of the thing
    /// being built, not a guess about it.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// declare const name: string | undefined;
    /// declare const port: number | undefined;
    /// const DEFAULT_PORT = 8080;
    /// const owner = "acme";
    ///
    /// const a = name ?? "unknown";
    /// const b = port || 1000;
    /// const c = port ?? DEFAULT_PORT;
    /// const d = name ?? `tenant-${owner}`;
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// declare const config: { a: number } | undefined;
    /// declare const items: string[] | undefined;
    /// declare const name: string | undefined;
    /// declare const count: number | undefined;
    /// declare function fallbackName(): string;
    ///
    /// const a = config ?? {};
    /// const b = items ?? [];
    /// const c = name ?? "";
    /// const d = count ?? 0;
    /// const e = name ?? fallbackName();
    ///
    /// if (name === undefined) {
    ///   throw new Error("name is required");
    /// }
    /// ```
    NoPrimitiveDefault(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    config = NoPrimitiveDefaultConfig,
    version = "1.86.0",
    short_description = "Disallow `??` and `||` with a made-up string or number constant.",
);

impl Rule for NoPrimitiveDefault {
    fn from_configuration(value: serde_json::Value) -> Result<Self, serde_json::error::Error> {
        DefaultRuleConfig::<Self>::from_value(value).map(DefaultRuleConfig::into_inner)
    }

    fn to_configuration(&self) -> Option<Result<serde_json::Value, serde_json::Error>> {
        Some(serde_json::to_value(&*self.0))
    }
}
