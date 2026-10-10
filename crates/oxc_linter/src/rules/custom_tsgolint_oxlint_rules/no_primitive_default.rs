use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoPrimitiveDefault;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows `??`, `??=`, `||` and `||=` when the right-hand side is typed
    /// as a string or a number. The check reads the type, not the syntax, so a
    /// named constant or a call returning a string is caught just like a
    /// literal. An empty string is exempt.
    ///
    /// ### Why is this bad?
    ///
    /// `|| 0` and `?? "unknown"` invent a value for something that was
    /// genuinely absent, and the invented value is indistinguishable
    /// downstream from a real one — a count that was never measured reads as
    /// zero, a name that was never set reads as "unknown". The missing case is
    /// information, and substituting a primitive throws it away at the point
    /// furthest from anyone who could interpret it.
    ///
    /// `{}`, `[]` and `""` are allowed: an empty container or an empty string
    /// is not a made-up value, it is the identity of the thing being built.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// declare const name: string | undefined;
    /// declare const port: number | undefined;
    /// declare function fallbackName(): string;
    /// const DEFAULT_PORT = 8080;
    ///
    /// const a = name ?? "unknown";
    /// const b = port || 0;
    /// const c = port ?? DEFAULT_PORT;
    /// const d = name || fallbackName();
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// declare const config: { a: number } | undefined;
    /// declare const items: string[] | undefined;
    /// declare const enabled: boolean | undefined;
    /// declare const name: string | undefined;
    ///
    /// const a = config ?? {};
    /// const b = items ?? [];
    /// const c = enabled ?? false;
    /// const d = name ?? "";
    ///
    /// if (name === undefined) {
    ///   throw new Error("name is required");
    /// }
    /// ```
    NoPrimitiveDefault(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow `??` and `||` with a string or number fallback.",
);

impl Rule for NoPrimitiveDefault {}
