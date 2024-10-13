//! Functions in this section operate on `other` types.
//!
//! ## Other types included
//! - `VoidTypeRef`
//! - `LabelTypeRef`
//! - `X86MMXTypeRef`
//! - `X86AMXTypeRef`
//! - `TokenTypeRef`
//! - `MetadataTypeRef`
//! - `TargetExtTypeRef`

use crate::core::context::ContextRef;
use crate::core::types::TypeRef;
use crate::{CString, CUint, GetRef};
use llvm_sys::core;
use llvm_sys::prelude::LLVMTypeRef;

/// These functions relate to `VoidTypeRef` of `LLVMTypeRef` instances.
#[derive(Debug)]
pub struct VoidTypeRef(LLVMTypeRef);

impl From<LLVMTypeRef> for VoidTypeRef {
    fn from(value: LLVMTypeRef) -> Self {
        Self(value)
    }
}

impl GetRef for VoidTypeRef {
    type RawRef = LLVMTypeRef;
    fn get_ref(&self) -> Self::RawRef {
        self.0
    }
}

impl From<VoidTypeRef> for TypeRef {
    fn from(value: VoidTypeRef) -> Self {
        Self(value.0)
    }
}

impl VoidTypeRef {
    /// Create a void type in a context.
    ///
    /// # Details
    ///
    /// Creates a `void` type in the specified LLVM context.
    ///
    /// This function wraps the `LLVMVoidTypeInContext` function from the LLVM core library. It creates and returns
    /// a `void` type within the specified LLVM context. The `void` type represents the absence of a value and is typically
    /// used as the return type for functions that do not return anything.
    ///
    /// # Parameters
    ///
    /// - `context`: A reference to the `ContextRef` in which the `void` type will be created.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `void` type in the specified context.
    #[must_use]
    pub fn void_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMVoidTypeInContext(context.get_ref())) }
    }

    /// Create a void type in a global context.
    ///
    /// # Details
    ///
    /// Creates a `void` type in the global LLVM context.
    ///
    /// This function wraps the `LLVMVoidType` function from the LLVM core library. It creates and returns
    /// a `void` type within the global LLVM context. The `void` type represents the absence of a value and is typically
    /// used as the return type for functions that do not return anything.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `void` type in the global context.
    #[must_use]
    pub fn void_type() -> Self {
        unsafe { Self(core::LLVMVoidType()) }
    }
}

/// These functions relate to `LabelTypeRef` of `LLVMTypeRef` instances.
#[derive(Debug)]
pub struct LabelTypeRef(LLVMTypeRef);

impl From<LLVMTypeRef> for LabelTypeRef {
    fn from(value: LLVMTypeRef) -> Self {
        Self(value)
    }
}

impl GetRef for LabelTypeRef {
    type RawRef = LLVMTypeRef;
    fn get_ref(&self) -> Self::RawRef {
        self.0
    }
}

impl From<LabelTypeRef> for TypeRef {
    fn from(value: LabelTypeRef) -> Self {
        Self(value.0)
    }
}

impl LabelTypeRef {
    /// Create a label type in a context.
    ///
    /// # details
    ///
    /// Creates a `label` type in the specified LLVM context.
    ///
    /// This function wraps the `LLVMLabelTypeInContext` function from the LLVM core library. It creates and returns
    /// a `label` type within the specified LLVM context. The `label` type is used in LLVM IR to represent labels, which
    /// are markers for basic blocks that can be targeted by branch instructions.
    ///
    /// # Parameters
    ///
    /// - `context`: A reference to the `ContextRef` in which the `label` type will be created.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `label` type in the specified context.
    #[must_use]
    pub fn label_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMLabelTypeInContext(context.get_ref())) }
    }

    /// Create a label type in a global context.
    ///
    /// # Details
    ///
    /// Creates a `label` type in the global LLVM context.
    ///
    /// This function wraps the `LLVMLabelType` function from the LLVM core library. It creates and returns
    /// a `label` type within the global LLVM context. The `label` type is used in LLVM IR to represent labels, which
    /// are markers for basic blocks that can be targeted by branch instructions.
    ///
    /// # Returns
    ///
    /// Returns an instance of `TypeRef` representing the `label` type in the global context.
    #[must_use]
    pub fn label_type() -> TypeRef {
        unsafe { TypeRef(core::LLVMLabelType()) }
    }
}

/// These functions relate to `X86MMXTypeRef` of `X86MMXTypeRef` instances.
#[derive(Debug)]
pub struct X86MMXTypeRef(LLVMTypeRef);

impl From<LLVMTypeRef> for X86MMXTypeRef {
    fn from(value: LLVMTypeRef) -> Self {
        Self(value)
    }
}

impl GetRef for X86MMXTypeRef {
    type RawRef = LLVMTypeRef;
    fn get_ref(&self) -> Self::RawRef {
        self.0
    }
}

impl From<X86MMXTypeRef> for TypeRef {
    fn from(value: X86MMXTypeRef) -> Self {
        Self(value.0)
    }
}

impl X86MMXTypeRef {
    /// Create an X86 MMX type in a context.
    ///
    /// # Details
    ///
    /// Creates an `x86_mmx` type in the specified LLVM context.
    ///
    /// This function wraps the `LLVMX86MMXTypeInContext` function from the LLVM core library. It creates and returns
    /// an `x86_mmx` type within the specified LLVM context. The `x86_mmx` type is used in LLVM IR to represent
    /// MMX registers, which are used for SIMD (Single Instruction, Multiple Data) operations on x86 architectures.
    ///
    /// # Parameters
    ///
    /// - `context`: A reference to the `ContextRef` in which the `x86_mmx` type will be created.
    ///
    /// # Returns
    ///
    /// Returns an instance of `TypeRef` representing the `x86_mmx` type in the specified context.
    #[must_use]
    pub fn x86_mmx_type_in_context(context: &ContextRef) -> TypeRef {
        unsafe { TypeRef(core::LLVMX86MMXTypeInContext(context.get_ref())) }
    }

    /// Create a X86 MMX type in a global context.
    ///
    /// # Details
    ///
    /// Creates an `x86_mmx` type in the global LLVM context.
    ///
    /// This function wraps the `LLVMX86MMXType` function from the LLVM core library. It creates and returns
    /// an `x86_mmx` type within the global LLVM context. The `x86_mmx` type is used in LLVM IR to represent
    /// MMX registers, which are used for SIMD (Single Instruction, Multiple Data) operations on x86 architectures.
    ///
    /// # Returns
    ///
    /// Returns an instance of `TypeRef` representing the `x86_mmx` type in the global context.
    #[must_use]
    pub fn x86_mmx_type() -> TypeRef {
        unsafe { TypeRef(core::LLVMX86MMXType()) }
    }
}

/// These functions relate to `X86AMXTypeRef` of `X86MMXTypeRef` instances.
#[derive(Debug)]
pub struct X86AMXTypeRef(LLVMTypeRef);

impl From<LLVMTypeRef> for X86AMXTypeRef {
    fn from(value: LLVMTypeRef) -> Self {
        Self(value)
    }
}

impl GetRef for X86AMXTypeRef {
    type RawRef = LLVMTypeRef;
    fn get_ref(&self) -> Self::RawRef {
        self.0
    }
}

impl From<X86AMXTypeRef> for TypeRef {
    fn from(value: X86AMXTypeRef) -> Self {
        Self(value.0)
    }
}

impl X86AMXTypeRef {
    /// Create an X86 AMX type in a context.
    ///
    /// # Details
    ///
    /// Creates an `x86_amx` type in the specified LLVM context.
    ///
    /// This function wraps the `LLVMX86AMXTypeInContext` function from the LLVM core library. It creates and returns
    /// an `x86_amx` type within the specified LLVM context. The `x86_amx` type is used in LLVM IR to represent
    /// AMX (Advanced Matrix Extensions) registers, which are used for advanced matrix operations on x86 architectures.
    ///
    /// # Parameters
    ///
    /// - `context`: A reference to the `ContextRef` in which the `x86_amx` type will be created.
    ///
    /// # Returns
    ///
    /// Returns an instance of `TypeRef` representing the `x86_amx` type in the specified context.
    #[must_use]
    pub fn x86_amx_type_in_context(context: &ContextRef) -> TypeRef {
        unsafe { TypeRef(core::LLVMX86AMXTypeInContext(context.get_ref())) }
    }

    /// Create a X86 AMX type in a global context.
    ///
    /// # Details
    ///
    /// Creates an `x86_amx` type in the global LLVM context.
    ///
    /// This function wraps the `LLVMX86AMXType` function from the LLVM core library. It creates and returns
    /// an `x86_amx` type within the global LLVM context. The `x86_amx` type is used in LLVM IR to represent
    /// AMX (Advanced Matrix Extensions) registers, which are used for advanced matrix operations on x86 architectures.
    ///
    /// # Returns
    ///
    /// Returns an instance of `TypeRef` representing the `x86_amx` type in the global context.
    #[must_use]
    pub fn x86_amx_type() -> TypeRef {
        unsafe { TypeRef(core::LLVMX86AMXType()) }
    }
}

/// These functions relate to `TokenTypeRef` of `X86MMXTypeRef` instances.
#[derive(Debug)]
pub struct TokenTypeRef(LLVMTypeRef);

impl From<LLVMTypeRef> for TokenTypeRef {
    fn from(value: LLVMTypeRef) -> Self {
        Self(value)
    }
}

impl GetRef for TokenTypeRef {
    type RawRef = LLVMTypeRef;
    fn get_ref(&self) -> Self::RawRef {
        self.0
    }
}

impl From<TokenTypeRef> for TypeRef {
    fn from(value: TokenTypeRef) -> Self {
        Self(value.0)
    }
}

impl TokenTypeRef {
    /// Create a token type in a context.
    ///
    /// # Details
    ///
    /// Creates a `token` type in the specified LLVM context.
    ///
    /// This function wraps the `LLVMTokenTypeInContext` function from the LLVM core library. It creates and returns
    /// a `token` type within the specified LLVM context. The `token` type in LLVM IR is used to represent opaque values
    /// that are used in certain operations, such as operand bundles, without carrying any type information.
    ///
    /// # Parameters
    ///
    /// - `context`: A reference to the `ContextRef` in which the `token` type will be created.
    ///
    /// # Returns
    ///
    /// Returns an instance of `TypeRef` representing the `token` type in the specified context.
    #[must_use]
    pub fn token_type_in_context(context: &ContextRef) -> TypeRef {
        unsafe { TypeRef(core::LLVMTokenTypeInContext(context.get_ref())) }
    }
}

/// These functions relate to `MetadataTypeRef` of `X86MMXTypeRef` instances.
#[derive(Debug)]
pub struct MetadataTypeRef(LLVMTypeRef);

impl From<LLVMTypeRef> for MetadataTypeRef {
    fn from(value: LLVMTypeRef) -> Self {
        Self(value)
    }
}

impl GetRef for MetadataTypeRef {
    type RawRef = LLVMTypeRef;
    fn get_ref(&self) -> Self::RawRef {
        self.0
    }
}

impl From<MetadataTypeRef> for TypeRef {
    fn from(value: MetadataTypeRef) -> Self {
        Self(value.0)
    }
}

impl MetadataTypeRef {
    /// Create a metadata type in a context.
    ///
    /// # Details
    ///
    /// Creates a `metadata` type in the specified LLVM context.
    ///
    /// This function wraps the `LLVMMetadataTypeInContext` function from the LLVM core library. It creates and returns
    /// a `metadata` type within the specified LLVM context. The `metadata` type in LLVM IR is used to represent
    /// metadata nodes, which store additional information that is not part of the program's code but is used for purposes
    /// such as debugging, optimization, and analysis.
    ///
    /// # Parameters
    ///
    /// - `context`: A reference to the `ContextRef` in which the `metadata` type will be created.
    ///
    /// # Returns
    ///
    /// Returns an instance of `TypeRef` representing the `metadata` type in the specified context.
    #[must_use]
    pub fn metadata_type_in_context(context: &ContextRef) -> TypeRef {
        unsafe { TypeRef(core::LLVMMetadataTypeInContext(context.get_ref())) }
    }
}

/// These functions relate to `TargetExtTypeRef` of `X86MMXTypeRef` instances.
#[derive(Debug)]
pub struct TargetExtTypeRef(LLVMTypeRef);

impl From<LLVMTypeRef> for TargetExtTypeRef {
    fn from(value: LLVMTypeRef) -> Self {
        Self(value)
    }
}

impl GetRef for TargetExtTypeRef {
    type RawRef = LLVMTypeRef;
    fn get_ref(&self) -> Self::RawRef {
        self.0
    }
}

impl From<TargetExtTypeRef> for TypeRef {
    fn from(value: TargetExtTypeRef) -> Self {
        Self(value.0)
    }
}

impl TargetExtTypeRef {
    /// Create a target extension type in LLVM context.
    ///
    /// # Details
    ///
    /// Creates a target-specific extension type in the specified LLVM context.
    ///
    /// This function wraps the `LLVMTargetExtTypeInContext` function from the LLVM core library. It creates and returns
    /// a target extension type within the specified LLVM context. Target extension types are custom types defined by
    /// specific target architectures that may require specialized handling in LLVM IR. This function allows you to define
    /// such types with custom type and integer parameters.
    ///
    /// # Parameters
    ///
    /// - `context`: A reference to the `ContextRef` in which the target extension type will be created.
    /// - `name`: A string slice (`&str`) representing the name of the target extension type.
    /// - `type_params`: A slice of `TypeRef` representing type parameters for the target extension type. This slice can be empty if no type parameters are required.
    /// - `int_params`: A slice of `u32` representing integer parameters for the target extension type. This slice can be empty if no integer parameters are required.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the target extension type in the specified context.
    #[must_use]
    pub fn target_ext_type_in_context(
        context: &ContextRef,
        name: &str,
        type_params: &[TypeRef],
        int_params: &[u32],
    ) -> Self {
        let type_params_ptr = crate::to_mut_ptr!(type_params);
        let int_params_ptr = crate::map_mut_ptr!(int_params, |v| *CUint::from(*v));

        let c_name = CString::from(name);
        unsafe {
            Self(core::LLVMTargetExtTypeInContext(
                context.get_ref(),
                c_name.as_ptr(),
                type_params_ptr,
                *CUint::from(type_params.len()),
                int_params_ptr,
                *CUint::from(int_params.len()),
            ))
        }
    }
}
