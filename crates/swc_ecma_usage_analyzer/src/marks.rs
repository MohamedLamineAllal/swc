#![allow(dead_code)]

use swc_common::Mark;

#[derive(Debug, Clone, Copy)]
pub struct Marks {
    /// [Mark] applied to non-top level variables which is injected while
    /// inlining.
    ///
    /// In other words, AST nodes marked with this mark will not be treated as a
    /// top level item, even if it's in the top level scope.
    pub non_top_level: Mark,

    /// Indicates that a sequence expression is generated by the minifier.
    ///
    /// This is required because `sequences` option is ignored for synthesized
    /// sequences.
    pub synthesized_seq: Mark,

    /// Treat this function as a top level module.
    ///
    /// If this mark is applied, the function will be treated as a black box. It
    /// will not be analyzed by usage analyzer.
    ///
    /// # Note
    ///
    /// Standalone functions should not depend on any other declarations in the
    /// outer scope.
    ///
    /// This is only applied to [swc_ecma_ast::Function] and it should not be
    /// nested.
    pub standalone: Mark,

    //// Applied to [swc_ecma_ast::Module].
    pub bundle_of_standalone: Mark,

    ///  `/** @const */`.
    pub const_ann: Mark,

    /// Check for `/*#__NOINLINE__*/`
    pub noinline: Mark,

    /// Check for `/*#__PURE__*/`
    pub pure: Mark,

    /// This is applied to [swc_ecma_ast::BlockStmt] which is injected to
    /// preserve the side effects.
    pub fake_block: Mark,

    pub unresolved_mark: Mark,
}

impl Marks {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        fn m() -> Mark {
            Mark::new()
        }

        Marks {
            non_top_level: m(),
            synthesized_seq: m(),
            standalone: m(),
            bundle_of_standalone: m(),
            const_ann: m(),
            noinline: m(),
            pure: m(),
            fake_block: m(),
            unresolved_mark: m(),
        }
    }
}
