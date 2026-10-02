use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct RequireTypeAnnotation;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Requires a type annotation on any class property or variable declared
    /// without an initializer on the same line.
    ///
    /// ### Why is this bad?
    ///
    /// `public readonly app;` compiles — TypeScript infers the property type from
    /// whatever the constructor assigns — but the declaration tells the reader
    /// nothing, and the inferred type silently changes whenever that far-away
    /// assignment changes.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// class Routes {
    ///   public readonly app;
    ///   public constructor() {
    ///     this.app = buildApp();
    ///   }
    /// }
    ///
    /// let port;
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// class Routes {
    ///   public readonly app: Hono;
    ///   public constructor() {
    ///     this.app = buildApp();
    ///   }
    /// }
    ///
    /// let port: number;
    /// const host = "127.0.0.1";
    /// ```
    RequireTypeAnnotation(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Require a type annotation when a declaration has no initializer.",
);

impl Rule for RequireTypeAnnotation {}
