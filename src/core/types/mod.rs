pub mod float;
pub mod function;
pub mod int;
pub mod other;
pub mod sequential;
pub mod structs;

use std::ops::Deref;

use crate::core::context::ContextRef;
use crate::CUint;
use crate::GetRef;
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::{core, LLVMTypeKind};

/// Represents the different kinds of types in LLVM IR.
///
/// The `TypeKind` enum categorizes the various types that can exist within LLVM IR. Each variant of this enum
/// corresponds to a specific kind of type that LLVM supports, such as integer types, floating-point types,
/// vector types, and others. This enum is useful for identifying and working with different types when
/// manipulating LLVM IR.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TypeKind {
    /// Represents the `void` type, which has no value.
    VoidTypeKind,
    /// Represents the 16-bit floating-point type (`half`).
    HalfTypeKind,
    /// Represents the 32-bit floating-point type (`float`).
    FloatTypeKind,
    /// Represents the 64-bit floating-point type (`double`).
    DoubleTypeKind,
    /// Represents the 80-bit floating-point type used in x86 architecture.
    X86_FP80TypeKind,
    /// Represents the 128-bit floating-point type (`fp128`).
    FP128TypeKind,
    /// Represents the 128-bit floating-point type used in `PowerPC` architecture.
    PPC_FP128TypeKind,
    /// Represents a label type, which is used for basic block labels.
    LabelTypeKind,
    /// Represents an integer type of arbitrary bit width.
    IntegerTypeKind,
    /// Represents a function type, including the return type and parameter types.
    FunctionTypeKind,
    /// Represents a structure type, which is a collection of fields.
    StructTypeKind,
    /// Represents an array type, which is a sequence of elements of a specified type.
    ArrayTypeKind,
    /// Represents a pointer type, pointing to a value of a specified type.
    PointerTypeKind,
    /// Represents a vector type, which is a sequence of elements of a specified type that can be processed in parallel.
    VectorTypeKind,
    /// Represents metadata type, which is used for attaching additional information to instructions.
    MetadataTypeKind,
    /// Represents the MMX type used in x86 architecture.
    X86_MMXTypeKind,
    /// Represents a token type, used for operand bundles.
    TokenTypeKind,
    /// Represents a scalable vector type, which can dynamically change in size.
    ScalableVectorTypeKind,
    /// Represents the `bfloat` floating-point type used in machine learning applications.
    BFloatTypeKind,
    /// Represents the AMX type used in x86 architecture for matrix multiplication.
    X86_AMXTypeKind,
    /// Represents a target-specific extended type.
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
