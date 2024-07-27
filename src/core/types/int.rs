//! Functions in this section operate on integer types.

use crate::core::context::ContextRef;
use crate::core::types::TypeRef;
use crate::GetRef;
use llvm_sys::core;

/// Obtain an integer type from a context with specified bit width.
impl TypeRef {
    #[must_use]
    pub fn int1_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMInt1TypeInContext(context.get_ref())) }
    }

    #[must_use]
    pub fn int8_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMInt8TypeInContext(context.get_ref())) }
    }

    #[must_use]
    pub fn int16_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMInt16TypeInContext(context.get_ref())) }
    }

    #[must_use]
    pub fn int32_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMInt32TypeInContext(context.get_ref())) }
    }

    #[must_use]
    pub fn int64_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMInt64TypeInContext(context.get_ref())) }
    }

    #[must_use]
    pub fn int128_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMInt128TypeInContext(context.get_ref())) }
    }

    #[must_use]
    pub fn int_type_in_context(context: &ContextRef, num_bits: u32) -> Self {
        unsafe { Self(core::LLVMIntTypeInContext(context.get_ref(), num_bits)) }
    }
}

/// Obtain an integer type from the global context with a specified bit width.
impl TypeRef {
    #[must_use]
    pub fn int1_type() -> Self {
        unsafe { Self(core::LLVMInt1Type()) }
    }

    #[must_use]
    pub fn int8_type() -> Self {
        unsafe { Self(core::LLVMInt8Type()) }
    }

    #[must_use]
    pub fn int16_type() -> Self {
        unsafe { Self(core::LLVMInt16Type()) }
    }

    #[must_use]
    pub fn int32_type() -> Self {
        unsafe { Self(core::LLVMInt32Type()) }
    }

    #[must_use]
    pub fn int64_type() -> Self {
        unsafe { Self(core::LLVMInt64Type()) }
    }

    #[must_use]
    pub fn int128_type() -> Self {
        unsafe { Self(core::LLVMInt128Type()) }
    }

    #[must_use]
    pub fn int_type(num_bits: u32) -> Self {
        unsafe { Self(core::LLVMIntType(num_bits)) }
    }

    #[must_use]
    pub fn get_int_type_width(&self) -> u32 {
        unsafe { core::LLVMGetIntTypeWidth(self.0) }
    }
}
