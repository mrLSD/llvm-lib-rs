pub mod float;
pub mod function;
pub mod int;
pub mod sequential;
pub mod structs;

use std::ops::Deref;

use crate::core::context::ContextRef;
use crate::CUint;
use crate::GetRef;
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::{core, LLVMTypeKind};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TypeKind {
    VoidTypeKind,
    HalfTypeKind,
    FloatTypeKind,
    DoubleTypeKind,
    X86_FP80TypeKind,
    FP128TypeKind,
    PPC_FP128TypeKind,
    LabelTypeKind,
    IntegerTypeKind,
    FunctionTypeKind,
    StructTypeKind,
    ArrayTypeKind,
    PointerTypeKind,
    VectorTypeKind,
    MetadataTypeKind,
    X86_MMXTypeKind,
    TokenTypeKind,
    ScalableVectorTypeKind,
    BFloatTypeKind,
    X86_AMXTypeKind,
    TargetExtTypeKind,
}

impl From<LLVMTypeKind> for TypeKind {
    fn from(llvm_type: LLVMTypeKind) -> Self {
        match llvm_type {
            LLVMTypeKind::LLVMVoidTypeKind => Self::VoidTypeKind,
            LLVMTypeKind::LLVMHalfTypeKind => Self::HalfTypeKind,
            LLVMTypeKind::LLVMFloatTypeKind => Self::FloatTypeKind,
            LLVMTypeKind::LLVMDoubleTypeKind => Self::DoubleTypeKind,
            LLVMTypeKind::LLVMX86_FP80TypeKind => Self::X86_FP80TypeKind,
            LLVMTypeKind::LLVMFP128TypeKind => Self::FP128TypeKind,
            LLVMTypeKind::LLVMPPC_FP128TypeKind => Self::PPC_FP128TypeKind,
            LLVMTypeKind::LLVMLabelTypeKind => Self::LabelTypeKind,
            LLVMTypeKind::LLVMIntegerTypeKind => Self::IntegerTypeKind,
            LLVMTypeKind::LLVMFunctionTypeKind => Self::FunctionTypeKind,
            LLVMTypeKind::LLVMStructTypeKind => Self::StructTypeKind,
            LLVMTypeKind::LLVMArrayTypeKind => Self::ArrayTypeKind,
            LLVMTypeKind::LLVMPointerTypeKind => Self::PointerTypeKind,
            LLVMTypeKind::LLVMVectorTypeKind => Self::VectorTypeKind,
            LLVMTypeKind::LLVMMetadataTypeKind => Self::MetadataTypeKind,
            LLVMTypeKind::LLVMX86_MMXTypeKind => Self::X86_MMXTypeKind,
            LLVMTypeKind::LLVMTokenTypeKind => Self::TokenTypeKind,
            LLVMTypeKind::LLVMScalableVectorTypeKind => Self::ScalableVectorTypeKind,
            LLVMTypeKind::LLVMBFloatTypeKind => Self::BFloatTypeKind,
            LLVMTypeKind::LLVMX86_AMXTypeKind => Self::X86_AMXTypeKind,
            LLVMTypeKind::LLVMTargetExtTypeKind => Self::TargetExtTypeKind,
        }
    }
}

impl From<TypeKind> for LLVMTypeKind {
    fn from(type_kind: TypeKind) -> Self {
        match type_kind {
            TypeKind::VoidTypeKind => Self::LLVMVoidTypeKind,
            TypeKind::HalfTypeKind => Self::LLVMHalfTypeKind,
            TypeKind::FloatTypeKind => Self::LLVMFloatTypeKind,
            TypeKind::DoubleTypeKind => Self::LLVMDoubleTypeKind,
            TypeKind::X86_FP80TypeKind => Self::LLVMX86_FP80TypeKind,
            TypeKind::FP128TypeKind => Self::LLVMFP128TypeKind,
            TypeKind::PPC_FP128TypeKind => Self::LLVMPPC_FP128TypeKind,
            TypeKind::LabelTypeKind => Self::LLVMLabelTypeKind,
            TypeKind::IntegerTypeKind => Self::LLVMIntegerTypeKind,
            TypeKind::FunctionTypeKind => Self::LLVMFunctionTypeKind,
            TypeKind::StructTypeKind => Self::LLVMStructTypeKind,
            TypeKind::ArrayTypeKind => Self::LLVMArrayTypeKind,
            TypeKind::PointerTypeKind => Self::LLVMPointerTypeKind,
            TypeKind::VectorTypeKind => Self::LLVMVectorTypeKind,
            TypeKind::MetadataTypeKind => Self::LLVMMetadataTypeKind,
            TypeKind::X86_MMXTypeKind => Self::LLVMX86_MMXTypeKind,
            TypeKind::TokenTypeKind => Self::LLVMTokenTypeKind,
            TypeKind::ScalableVectorTypeKind => Self::LLVMScalableVectorTypeKind,
            TypeKind::BFloatTypeKind => Self::LLVMBFloatTypeKind,
            TypeKind::X86_AMXTypeKind => Self::LLVMX86_AMXTypeKind,
            TypeKind::TargetExtTypeKind => Self::LLVMTargetExtTypeKind,
        }
    }
}

/// LLVM Type structure wrapper
/// Types represent the type of a value.
///
/// Types are associated with a context instance. The context internally
/// deduplicates types so there is only 1 instance of a specific type
/// alive at a time. In other words, a unique type is shared among all
/// consumers within a context.
#[derive(Debug)]
pub struct TypeRef(LLVMTypeRef);

impl From<LLVMTypeRef> for TypeRef {
    fn from(value: LLVMTypeRef) -> Self {
        Self(value)
    }
}

impl GetRef for TypeRef {
    type RawRef = LLVMTypeRef;
    fn get_ref(&self) -> Self::RawRef {
        self.0
    }
}

impl TypeRef {
    /// Obtain the enumerated type of a Type instance.
    #[must_use]
    pub fn get_kind(&self) -> TypeKind {
        unsafe { TypeKind::from(core::LLVMGetTypeKind(self.0)) }
    }

    /// Whether the type has a known size.
    ///
    /// Things that don't have a size are abstract types, labels, and void.
    #[must_use]
    pub fn is_sized(&self) -> bool {
        unsafe { core::LLVMTypeIsSized(self.0) != 0 }
    }

    /// Obtain the context to which this type instance is associated.
    #[must_use]
    pub fn get_context(&self) -> ContextRef {
        unsafe { ContextRef::from(core::LLVMGetTypeContext(self.0)) }
    }

    /// Dump a representation of a type to stderr.
    pub fn dump(&self) {
        unsafe { core::LLVMDumpType(self.0) }
    }

    /// Create Void type in context
    #[must_use]
    pub fn void_type(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMVoidTypeInContext(**context)) }
    }

    /// Create Ptr type in context
    #[must_use]
    pub fn ptr_type(ptr_raw_type: &Self, address_space: u32) -> Self {
        unsafe {
            Self(core::LLVMPointerType(
                **ptr_raw_type,
                *CUint::from(address_space),
            ))
        }
    }

    /// Create f32 type in context
    #[must_use]
    pub fn f32_type(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMFloatTypeInContext(**context)) }
    }

    /// Create f64 type in context
    #[must_use]
    pub fn f64_type(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMDoubleTypeInContext(**context)) }
    }

    /// Create bool type in context
    #[must_use]
    pub fn bool_type(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMInt1TypeInContext(**context)) }
    }

    /// Create i8 type in context
    #[must_use]
    pub fn i8_type(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMInt8TypeInContext(**context)) }
    }

    /// Create i16 type in context
    #[must_use]
    pub fn i16_type(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMInt16TypeInContext(**context)) }
    }

    /// Create i32 type in context
    #[must_use]
    pub fn i32_type(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMInt32TypeInContext(**context)) }
    }

    /// Create i64 type in context
    #[must_use]
    pub fn i64_type(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMInt64TypeInContext(**context)) }
    }

    /// Create array type in context based on Type
    #[must_use]
    pub fn array_type(array_type: &Self, size: u64) -> Self {
        unsafe { Self(core::LLVMArrayType2(**array_type, size)) }
    }

    /// Create function type based on argument types array, and function return type
    /// TODO: return error
    #[must_use]
    pub fn function_type(args_type: &[Self], return_type: &Self) -> Self {
        unsafe {
            let mut args_type = args_type.iter().map(|v| v.0).collect::<Vec<_>>();
            let args = if args_type.is_empty() {
                std::ptr::null_mut()
            } else {
                args_type.as_mut_ptr()
            };
            Self(core::LLVMFunctionType(
                return_type.0,
                args,
                *CUint::from(args_type.len()),
                0,
            ))
        }
    }
}

impl Deref for TypeRef {
    type Target = LLVMTypeRef;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
