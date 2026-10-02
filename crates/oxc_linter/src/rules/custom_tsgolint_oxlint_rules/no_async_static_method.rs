use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoAsyncStaticMethod;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallow `async` `static` class methods.
    ///
    /// ### Why is this bad?
    ///
    /// A static method has no instance `this`, so the promises it awaits cannot be
    /// handed to `this.track(...)` and `close()` will never wait for them.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// class Service {
    ///   static async run(): Promise<void> {}
    ///   static send = async (): Promise<void> => {};
    /// }
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// class Service {
    ///   async run(): Promise<void> {
    ///     await this.track(send());
    ///   }
    /// }
    /// ```
    NoAsyncStaticMethod(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow `async` `static` class methods.",
);

impl Rule for NoAsyncStaticMethod {}
