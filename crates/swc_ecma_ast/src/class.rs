use is_macro::Is;
#[cfg(feature = "rkyv-bytecheck-impl")]
use rkyv_latest as rkyv;
use serde::{Deserialize, Serialize};
use swc_common::{ast_node, util::take::Take, EqIgnoreSpan, Span, DUMMY_SP};

use crate::{
    expr::Expr,
    function::{Function, ParamOrTsParamProp},
    ident::PrivateName,
    prop::PropName,
    stmt::BlockStmt,
    typescript::{
        Accessibility, TsExprWithTypeArgs, TsIndexSignature, TsTypeAnn, TsTypeParamDecl,
        TsTypeParamInstantiation,
    },
    EmptyStmt,
};

#[ast_node]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Class {
    pub span: Span,

    #[serde(default)]
    pub decorators: Vec<Decorator>,

    #[serde(default)]
    pub body: Vec<ClassMember>,

    #[serde(default)]
    pub super_class: Option<Box<Expr>>,

    #[serde(default)]
    pub is_abstract: bool,

    #[serde(default)]
    pub type_params: Option<Box<TsTypeParamDecl>>,

    #[serde(default)]
    pub super_type_params: Option<Box<TsTypeParamInstantiation>>,

    /// Typescript extension.
    #[serde(default)]
    pub implements: Vec<TsExprWithTypeArgs>,
}

impl Take for Class {
    fn dummy() -> Self {
        Class {
            span: DUMMY_SP,
            decorators: Default::default(),
            body: Default::default(),
            super_class: Default::default(),
            is_abstract: Default::default(),
            type_params: Default::default(),
            super_type_params: Default::default(),
            implements: Default::default(),
        }
    }
}

#[ast_node]
#[derive(Eq, Hash, Is, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum ClassMember {
    #[tag("Constructor")]
    Constructor(Constructor),
    /// `es2015`
    #[tag("ClassMethod")]
    Method(ClassMethod),
    #[tag("PrivateMethod")]
    PrivateMethod(PrivateMethod),
    /// stage 0 / Typescript
    #[tag("ClassProperty")]
    ClassProp(ClassProp),
    #[tag("PrivateProperty")]
    PrivateProp(PrivateProp),
    #[tag("TsIndexSignature")]
    TsIndexSignature(TsIndexSignature),
    #[tag("EmptyStatement")]
    Empty(EmptyStmt),

    // Stage 3
    #[tag("StaticBlock")]
    StaticBlock(StaticBlock),
}

impl Take for ClassMember {
    fn dummy() -> Self {
        ClassMember::Empty(EmptyStmt { span: DUMMY_SP })
    }
}

#[ast_node("ClassProperty")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct ClassProp {
    #[serde(default)]
    pub span: Span,

    pub key: PropName,

    #[serde(default)]
    pub value: Option<Box<Expr>>,

    #[serde(default, rename = "typeAnnotation")]
    pub type_ann: Option<Box<TsTypeAnn>>,

    #[serde(default)]
    pub is_static: bool,

    #[serde(default)]
    pub decorators: Vec<Decorator>,

    /// Typescript extension.
    #[serde(default)]
    pub accessibility: Option<Accessibility>,

    /// Typescript extension.
    #[serde(default)]
    pub is_abstract: bool,

    #[serde(default)]
    pub is_optional: bool,

    #[serde(default)]
    pub is_override: bool,

    #[serde(default)]
    pub readonly: bool,

    #[serde(default)]
    pub declare: bool,

    #[serde(default)]
    pub definite: bool,
}

#[ast_node("PrivateProperty")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct PrivateProp {
    #[serde(default)]
    pub span: Span,

    pub key: PrivateName,

    #[serde(default)]
    pub value: Option<Box<Expr>>,

    #[serde(default, rename = "typeAnnotation")]
    pub type_ann: Option<Box<TsTypeAnn>>,

    #[serde(default)]
    pub is_static: bool,

    #[serde(default)]
    pub decorators: Vec<Decorator>,

    /// Typescript extension.
    #[serde(default)]
    pub accessibility: Option<Accessibility>,

    #[serde(default)]
    pub is_optional: bool,

    #[serde(default)]
    pub is_override: bool,

    #[serde(default)]
    pub readonly: bool,

    #[serde(default)]
    pub definite: bool,
}

macro_rules! method {
    ($name:ident, $ty:literal, $KEY:ty) => {
        #[ast_node($ty)]
        #[derive(Eq, Hash, EqIgnoreSpan)]
        #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
        pub struct $name {
            #[serde(default)]
            pub span: Span,

            pub key: $KEY,

            pub function: Box<Function>,

            pub kind: MethodKind,

            #[serde(default)]
            pub is_static: bool,

            /// Typescript extension.
            #[serde(default)]
            pub accessibility: Option<Accessibility>,

            /// Typescript extension.
            #[serde(default)]
            pub is_abstract: bool,

            #[serde(default)]
            pub is_optional: bool,

            #[serde(default)]
            pub is_override: bool,
        }
    };
}

method!(ClassMethod, "ClassMethod", PropName);
method!(PrivateMethod, "PrivateMethod", PrivateName);

#[ast_node("Constructor")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Constructor {
    pub span: Span,

    pub key: PropName,

    pub params: Vec<ParamOrTsParamProp>,

    #[serde(default)]
    pub body: Option<BlockStmt>,

    #[serde(default)]
    pub accessibility: Option<Accessibility>,

    #[serde(default)]
    pub is_optional: bool,
}

#[ast_node("Decorator")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Decorator {
    pub span: Span,

    #[serde(rename = "expression")]
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(
    any(feature = "rkyv-impl", feature = "rkyv-bytecheck-impl"),
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
pub enum MethodKind {
    #[serde(rename = "method")]
    Method,
    #[serde(rename = "getter")]
    Getter,
    #[serde(rename = "setter")]
    Setter,
}

#[ast_node("StaticBlock")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct StaticBlock {
    pub span: Span,
    pub body: BlockStmt,
}

impl Take for StaticBlock {
    fn dummy() -> Self {
        StaticBlock {
            span: DUMMY_SP,
            body: Take::dummy(),
        }
    }
}
