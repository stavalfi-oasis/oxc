use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoClientResponseJson;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallow `json()` on a typed hono client response.
    ///
    /// ### Why is this bad?
    ///
    /// The returned type is the shape this build was compiled against. Services deploy
    /// independently, so the body on the wire can be a different shape, and nothing
    /// checks it.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// const body: unknown = await response.json();
    /// return SCHEMA.parse(body).bytes;
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// return (await this.parsedJson({ response, schema: SCHEMA })).bytes;
    /// ```
    NoClientResponseJson(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow `json()` on a typed hono client response.",
);

impl Rule for NoClientResponseJson {}
