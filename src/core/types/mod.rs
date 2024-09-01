pub mod float;
pub mod function;
pub mod int;
pub mod other;
pub mod sequential;
pub mod structs;

use std::ops::Deref;

use crate::core::context::ContextRef;
use crate::{CStr, GetRef};
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::{core, LLVMTypeKind};

/// Represents the different kinds of types in LLVM IR.
///
/// The `TypeKind` enum categorizes the various types that can exist within LLVM IR. Each variant of this enum
/// corresponds to a specific kind of type that LLVM supports, such as integer types, floating-point types,
/// vector types, and others. This enum is useful for identifying and working with different types when
/// manipulating LLVM IR.
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
    ///
    /// # Details
    ///
    /// Retrieves the kind of the LLVM type.
    ///
    /// This function wraps the `LLVMGetTypeKind` function from the LLVM core library. It returns the `TypeKind`
    /// representing the specific kind of the type associated with `self`. The kind of a type indicates whether it is,
    /// for example, an integer, floating-point, function, array, pointer, or other type in LLVM IR.
    ///
    /// # Returns
    ///
    /// Returns a `TypeKind` representing the kind of the type.
    #[must_use]
    pub fn get_type_kind(&self) -> TypeKind {
        unsafe { TypeKind::from(core::LLVMGetTypeKind(self.0)) }
    }

    /// Whether the type has a known size.
    ///
    /// Things that don't have a size are abstract types, labels, and void.
    ///
    /// # Details
    ///
    /// Checks whether the LLVM type has a known size.
    ///
    /// This function wraps the `LLVMTypeIsSized` function from the LLVM core library. It determines whether
    /// the type represented by `self` has a known size in memory. Some types, such as opaque types, may not have
    /// a defined size until more information is provided.
    ///
    /// # Returns
    ///
    /// Returns `true` if the type has a known size, otherwise returns `false`.
    #[must_use]
    pub fn type_is_sized(&self) -> bool {
        unsafe { core::LLVMTypeIsSized(self.0) != 0 }
    }

    /// Obtain the context to which this type instance is associated.
    ///
    /// # Details
    ///
    /// Dumps a textual representation of the LLVM type to standard output.
    ///
    /// This function wraps the `LLVMDumpType` function from the LLVM core library. It prints a human-readable
    /// representation of the type represented by `self` to standard output. This function is primarily useful for
    /// debugging purposes, allowing you to inspect the type information directly in the console or terminal.
    #[must_use]
    pub fn get_type_context(&self) -> ContextRef {
        unsafe { ContextRef::from(core::LLVMGetTypeContext(self.0)) }
    }

    /// Dump a representation of a type to stderr.
    pub fn dump_type(&self) {
        unsafe { core::LLVMDumpType(self.0) }
    }

    /// Return a string representation of the type. Use
    /// `LLVMDisposeMessage` to free the string.
    ///
    /// # Details
    ///
    /// Converts the LLVM type to a human-readable string representation.
    ///
    /// This function wraps the `LLVMPrintTypeToString` function from the LLVM core library. It returns a `String`
    /// containing a human-readable representation of the type represented by `self`. This is useful for debugging
    /// or logging the type information in a readable format.
    ///
    /// If the conversion fails, the function returns an empty string.
    ///
    /// # Returns
    ///
    /// Returns a `String` containing the string representation of the type.
    #[must_use]
    pub fn print_type_to_string(&self) -> String {
        unsafe {
            let c_str = core::LLVMPrintTypeToString(self.0);
            if c_str.is_null() {
                return String::new();
            }
            let rust_string = CStr::new(c_str).to_string();
            core::LLVMDisposeMessage(c_str);
            rust_string
        }
    }
}

impl Deref for TypeRef {
    type Target = LLVMTypeRef;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
