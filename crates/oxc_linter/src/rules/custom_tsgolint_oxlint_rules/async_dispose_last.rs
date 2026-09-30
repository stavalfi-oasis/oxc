use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct AsyncDisposeLast;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Requires `[Symbol.asyncDispose]` to be declared after every other class member.
    ///
    /// ### Why is this bad?
    ///
    /// Disposal is the last thing a class does at runtime. Declaring it anywhere else means a reader has to scan past it to find the members it tears down.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// class A {
///   public async [Symbol.asyncDispose](): Promise<void> {}
///   public run(): void {}
/// }
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// class A {
///   public run(): void {}
///   public async [Symbol.asyncDispose](): Promise<void> {}
/// }
    /// ```
    AsyncDisposeLast(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Require `[Symbol.asyncDispose]` to be the last member of its class.",
);

impl Rule for AsyncDisposeLast {}
