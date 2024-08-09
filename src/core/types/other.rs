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

impl VoidTypeRef {
    /// Create a void type in a context.
    #[must_use]
    pub fn void_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMVoidTypeInContext(context.get_ref())) }
    }

    /// Create a void type in a global context.
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

impl LabelTypeRef {
    /// Create a label type in a context.
    #[must_use]
    pub fn label_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMLabelTypeInContext(context.get_ref())) }
    }

    /// Create a label type in a global context.
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

impl X86MMXTypeRef {
    /// Create an X86 MMX type in a context.
    #[must_use]
    pub fn x86_mmx_type_in_context(context: &ContextRef) -> TypeRef {
        unsafe { TypeRef(core::LLVMX86MMXTypeInContext(context.get_ref())) }
    }

    /// Create a X86 MMX type in a global context.
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

impl X86AMXTypeRef {
    /// Create an X86 AMX type in a context.
    #[must_use]
    pub fn x86_amx_type_in_context(context: &ContextRef) -> TypeRef {
        unsafe { TypeRef(core::LLVMX86AMXTypeInContext(context.get_ref())) }
    }

    /// Create a X86 AMX type in a global context.
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

impl TokenTypeRef {
    /// Create a token type in a context.
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

impl MetadataTypeRef {
    /// Create a metadata type in a context.
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

impl TargetExtTypeRef {
    /// Create a target extension type in LLVM context.
    #[must_use]
    pub fn target_ext_type_in_context(
        context: &ContextRef,
        name: &str,
        type_params: &[TypeRef],
        int_params: &[u32],
    ) -> Self {
        let type_params_ptr = if type_params.is_empty() {
            std::ptr::null_mut()
        } else {
            let mut type_params = type_params.iter().map(|v| v.0).collect::<Vec<_>>();
            type_params.as_mut_ptr()
        };
        let int_params_ptr = if int_params.is_empty() {
            std::ptr::null_mut()
        } else {
            let mut int_params = int_params
                .iter()
                .map(|v| *CUint::from(*v))
                .collect::<Vec<_>>();
            int_params.as_mut_ptr()
        };

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
