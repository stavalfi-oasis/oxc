use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoDisposeAssignment;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows writing to an instance property inside `[Symbol.dispose]` or
    /// `[Symbol.asyncDispose]`. Covers plain and compound assignment, `++` /
    /// `--`, `delete`, destructuring targets, writes reached through a property
    /// (`this.#workers.length = 0`), and writes made from arrow callbacks that
    /// share the method's `this`.
    ///
    /// ### Why is this bad?
    ///
    /// Disposal is the end of the instance's life, so nothing reads the value
    /// that was just written. What the write actually buys is a second,
    /// half-torn-down state the class now has to be correct in: a field the
    /// reader expects to be set is `undefined`, and a use-after-dispose turns
    /// into a confusing null error somewhere else instead of failing at the
    /// resource it touched. Copying the field into a local and tearing the
    /// local down does the same work without inventing that state.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// class Pool {
    ///   #workers: Worker[] = [];
    ///   #connection: Connection | undefined;
    ///   public async [Symbol.asyncDispose](): Promise<void> {
    ///     this.#workers.length = 0;
    ///     this.#connection = undefined;
    ///   }
    /// }
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// class Pool {
    ///   #workers: Worker[] = [];
    ///   #connection: Connection | undefined;
    ///   public async [Symbol.asyncDispose](): Promise<void> {
    ///     for (const worker of this.#workers) {
    ///       worker.shutdown();
    ///     }
    ///     await this.#connection?.close();
    ///   }
    /// }
    /// ```
    NoDisposeAssignment(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow writing to instance properties inside `[Symbol.dispose]` / `[Symbol.asyncDispose]`.",
);

impl Rule for NoDisposeAssignment {}
