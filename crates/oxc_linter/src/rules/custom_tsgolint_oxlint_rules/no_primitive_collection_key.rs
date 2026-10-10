use oxc_macros::declare_oxc_lint;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::rule::{DefaultRuleConfig, Rule};

#[derive(Debug, Default, Clone, Deserialize)]
pub struct NoPrimitiveCollectionKey(Box<NoPrimitiveCollectionKeyConfig>);

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct NoPrimitiveCollectionKeyConfig {
    /// Extra keyed container names to check, added to the built-in `Map`, `WeakMap`,
    /// `ReadonlyMap`, `Set`, `WeakSet`, `ReadonlySet` and `Record`.
    pub containers: Vec<String>,
    /// Extra alias names that name nothing, added to the built-in denylist
    /// (`Str`, `Value`, `Key`, `Data`, …). Compared case-insensitively.
    pub meaningless_names: Vec<String>,
}

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows keying `Map`, `Set`, `Record` and their read-only and weak
    /// variants on a bare `string`, `number` or `boolean`, on an alias that
    /// renames a primitive without naming it, or on nothing at all — a
    /// `new Map()` whose key type is left to inference.
    ///
    /// Only the key position is checked. Literal types and literal unions are
    /// allowed, since they already name every key they accept, and a type
    /// parameter is left to whoever instantiates it.
    ///
    /// ### Why is this bad?
    ///
    /// `Map<string, Session>` says a session is found by some string, but not
    /// which one — a tenant id, an account id and a session token are all
    /// `string`, so the compiler accepts any of them at every call site. A named
    /// alias puts the answer in the type and in every signature that passes it
    /// along.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// declare const sessions: Map<string, Session>;
    /// declare const payload: Record<string, unknown>;
    /// type Str = string;
    /// declare const byStr: Map<Str, Session>;
    /// const cache = new Map();
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// type TenantId = string;
    /// declare const sessions: Map<TenantId, Session>;
    /// declare const modes: Map<'read' | 'write', number>;
    /// declare function index<Key, Value>(entries: Map<Key, Value>): void;
    /// const cache = new Map<TenantId, Session>();
    /// ```
    NoPrimitiveCollectionKey(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    config = NoPrimitiveCollectionKeyConfig,
    version = "1.86.0",
    short_description = "Disallow keying `Map`, `Set` or `Record` on an unnamed primitive.",
);

impl Rule for NoPrimitiveCollectionKey {
    fn from_configuration(value: serde_json::Value) -> Result<Self, serde_json::error::Error> {
        DefaultRuleConfig::<Self>::from_value(value).map(DefaultRuleConfig::into_inner)
    }

    fn to_configuration(&self) -> Option<Result<serde_json::Value, serde_json::Error>> {
        Some(serde_json::to_value(&*self.0))
    }
}
