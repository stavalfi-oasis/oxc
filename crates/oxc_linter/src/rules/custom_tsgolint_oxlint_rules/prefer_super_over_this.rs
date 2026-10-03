use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct PreferSuperOverThis;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Requires `super.x` instead of `this.x` when `x` is a method or accessor
    /// inherited from a base class and not redeclared on the current class.
    ///
    /// ### Why is this bad?
    ///
    /// `this.x` and `super.x` read the same but say different things. Writing `super.x` makes it visible at the call site that the member lives on the base class, so a reader does not have to open the class to find out where it came from.
    ///
    /// Only prototype members are reported. An instance field, a static member, an abstract method and a member redeclared on the current class are all left alone, because `super` either does not reach them or would change which implementation runs.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// class A {
    ///   run(): void {}
    /// }
    /// class B extends A {
    ///   go(): void {
    ///     this.run();
    ///   }
    /// }
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// class A {
    ///   run(): void {}
    /// }
    /// class B extends A {
    ///   go(): void {
    ///     super.run();
    ///   }
    /// }
    /// ```
    PreferSuperOverThis(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Require `super.x` over `this.x` for inherited members.",
);

impl Rule for PreferSuperOverThis {}
