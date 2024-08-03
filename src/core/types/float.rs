use crate::core::context::ContextRef;
use crate::core::types::TypeRef;
use crate::GetRef;
use llvm_sys::core;
use llvm_sys::prelude::LLVMTypeRef;

/// Wrapper `LLVMTypeRef` for floating point types.
#[derive(Debug)]
pub struct FloatTypeRef(LLVMTypeRef);

impl From<LLVMTypeRef> for FloatTypeRef {
    fn from(value: LLVMTypeRef) -> Self {
        Self(value)
    }
}

impl GetRef for FloatTypeRef {
    type RawRef = LLVMTypeRef;
    fn get_ref(&self) -> Self::RawRef {
        self.0
    }
}

impl FloatTypeRef {
    /// Obtain a 16-bit floating point type from a context.
    #[must_use]
    pub fn half_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMHalfTypeInContext(context.get_ref())) }
    }

    /// Obtain a 16-bit brain floating point type from a context.
    #[must_use]
    pub fn bfloat_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMBFloatTypeInContext(context.get_ref())) }
    }

    /// Obtain a 32-bit floating point type from a context.
    #[must_use]
    pub fn float_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMFloatTypeInContext(context.get_ref())) }
    }

    /// Obtain a 64-bit floating point type from a context.
    #[must_use]
    pub fn double_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMDoubleTypeInContext(context.get_ref())) }
    }

    /// Obtain an 80-bit floating point type (X87) from a context.
    #[must_use]
    pub fn x86_fp80_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMX86FP80TypeInContext(context.get_ref())) }
    }

    /// Obtain a 128-bit floating point type (112-bit mantissa) from a context.
    #[must_use]
    pub fn fp128_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMFP128TypeInContext(context.get_ref())) }
    }

    /// Obtain a 128-bit floating point type (two 64-bits) from a context.
    #[must_use]
    pub fn ppc_fp128_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMPPCFP128TypeInContext(context.get_ref())) }
    }
}

/// Obtain a floating point type from the global context.
///
/// These map to the functions in this group of the same name.

impl TypeRef {
    #[must_use]
    pub fn half_type() -> Self {
        unsafe { Self(core::LLVMHalfType()) }
    }

    #[must_use]
    pub fn bfloat_type() -> Self {
        unsafe { Self(core::LLVMBFloatType()) }
    }

    #[must_use]
    pub fn float_type() -> Self {
        unsafe { Self(core::LLVMFloatType()) }
    }

    #[must_use]
    pub fn double_type() -> Self {
        unsafe { Self(core::LLVMDoubleType()) }
    }

    #[must_use]
    pub fn x86_fp80_type() -> Self {
        unsafe { Self(core::LLVMX86FP80Type()) }
    }

    #[must_use]
    pub fn fp128_type() -> Self {
        unsafe { Self(core::LLVMFP128Type()) }
    }

    #[must_use]
    pub fn ppc_fp128_type() -> Self {
        unsafe { Self(core::LLVMPPCFP128Type()) }
    }
}
