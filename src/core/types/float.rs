//! Functions in this section operate on floating point types.

use crate::core::context::ContextRef;
use crate::core::types::TypeRef;
use crate::GetRef;
use llvm_sys::core;
use llvm_sys::prelude::LLVMTypeRef;

/// Wrapper `LLVMTypeRef` for floating point types.
#[derive(Debug, Clone)]
pub struct FloatTypeRef(LLVMTypeRef);

impl From<LLVMTypeRef> for FloatTypeRef {
    fn from(value: LLVMTypeRef) -> Self {
        Self(value)
    }
}

impl From<FloatTypeRef> for TypeRef {
    fn from(value: FloatTypeRef) -> Self {
        Self(value.0)
    }
}

impl GetRef for FloatTypeRef {
    type RawRef = LLVMTypeRef;
    fn get_ref(&self) -> Self::RawRef {
        self.0
    }
}

/// Obtain a floating point type from the context.
impl FloatTypeRef {
    /// Obtain a 16-bit floating point type from a context.
    ///
    /// # Details
    ///
    /// Creates a 16-bit floating-point (`half`) type in the specified LLVM context.
    ///
    /// This function wraps the `LLVMHalfTypeInContext` function from the LLVM core library. It creates and returns
    /// a 16-bit floating-point type, commonly known as `half`, within the specified LLVM context. The `half` type is
    /// often used in graphics and machine learning applications where reduced precision is acceptable.
    ///
    /// # Parameters
    ///
    /// - `context`: A reference to the `ContextRef` in which the `half` type will be created.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `half` type in the specified context.
    #[must_use]
    pub fn half_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMHalfTypeInContext(context.get_ref())) }
    }

    /// Obtain a 16-bit brain floating point type from a context.
    ///
    /// # Details
    ///
    /// Creates a 16-bit floating-point (`bfloat`) type in the specified LLVM context.
    ///
    /// This function wraps the `LLVMBFloatTypeInContext` function from the LLVM core library. It creates and returns
    /// a 16-bit floating-point type, known as `bfloat`, within the specified LLVM context. The `bfloat` type is commonly
    /// used in machine learning and other applications where reduced precision is sufficient, but a wider exponent range
    /// than `half` is required.
    ///
    /// # Parameters
    ///
    /// - `context`: A reference to the `ContextRef` in which the `bfloat` type will be created.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `bfloat` type in the specified context.
    #[must_use]
    pub fn bfloat_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMBFloatTypeInContext(context.get_ref())) }
    }

    /// Obtain a 32-bit floating point type from a context.
    ///
    /// # Details
    ///
    /// Creates a 32-bit floating-point (`float`) type in the specified LLVM context.
    ///
    /// This function wraps the `LLVMFloatTypeInContext` function from the LLVM core library. It creates and returns
    /// a 32-bit floating-point type, commonly known as `float`, within the specified LLVM context. The `float` type is
    /// the standard single-precision floating-point type used in many computations.
    ///
    /// # Parameters
    ///
    /// - `context`: A reference to the `ContextRef` in which the `float` type will be created.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `float` type in the specified context.
    #[must_use]
    pub fn float_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMFloatTypeInContext(context.get_ref())) }
    }

    /// Obtain a 64-bit floating point type from a context.
    ///
    /// # Details
    ///
    /// Creates a 64-bit floating-point (`double`) type in the specified LLVM context.
    ///
    /// This function wraps the `LLVMDoubleTypeInContext` function from the LLVM core library. It creates and returns
    /// a 64-bit floating-point type, commonly known as `double`, within the specified LLVM context. The `double` type is
    /// the standard double-precision floating-point type used for higher precision calculations in many applications.
    ///
    /// # Parameters
    ///
    /// - `context`: A reference to the `ContextRef` in which the `double` type will be created.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `double` type in the specified context.
    #[must_use]
    pub fn double_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMDoubleTypeInContext(context.get_ref())) }
    }

    /// Obtain an 80-bit floating point type (X87) from a context.
    ///
    /// # Details
    ///
    /// Creates an 80-bit floating-point (`x86_fp80`) type in the specified LLVM context.
    ///
    /// This function wraps the `LLVMX86FP80TypeInContext` function from the LLVM core library. It creates and returns
    /// an 80-bit floating-point type, known as `x86_fp80`, within the specified LLVM context. This type is specific to
    /// x86 architecture and provides extended precision floating-point arithmetic, commonly used in legacy systems
    /// and certain numerical applications.
    ///
    /// # Parameters
    ///
    /// - `context`: A reference to the `ContextRef` in which the `x86_fp80` type will be created.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `x86_fp80` type in the specified context.
    #[must_use]
    pub fn x86_fp80_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMX86FP80TypeInContext(context.get_ref())) }
    }

    /// Obtain a 128-bit floating point type (112-bit mantissa) from a context.
    ///
    /// Creates a 128-bit floating-point (`fp128`) type in the specified LLVM context.
    ///
    /// This function wraps the `LLVMFP128TypeInContext` function from the LLVM core library. It creates and returns
    /// a 128-bit floating-point type, known as `fp128`, within the specified LLVM context. The `fp128` type is used for
    /// very high precision floating-point arithmetic, though it is less commonly used compared to other floating-point types.
    ///
    /// # Parameters
    ///
    /// - `context`: A reference to the `ContextRef` in which the `fp128` type will be created.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `fp128` type in the specified context.
    #[must_use]
    pub fn fp128_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMFP128TypeInContext(context.get_ref())) }
    }

    /// Obtain a 128-bit floating point type (two 64-bits) from a context.
    ///
    /// # Details
    ///
    /// Creates a 128-bit floating-point (`ppc_fp128`) type in the specified LLVM context, specific to `PowerPC` architecture.
    ///
    /// This function wraps the `LLVMPPCFP128TypeInContext` function from the LLVM core library. It creates and returns
    /// a 128-bit floating-point type, known as `ppc_fp128`, within the specified LLVM context. This type is specific to
    /// the `PowerPC` architecture and is used for high-precision floating-point arithmetic on `PowerPC` systems.
    ///
    /// # Parameters
    ///
    /// - `context`: A reference to the `ContextRef` in which the `ppc_fp128` type will be created.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `ppc_fp128` type in the specified context.
    #[must_use]
    pub fn ppc_fp128_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMPPCFP128TypeInContext(context.get_ref())) }
    }
}

/// Obtain a floating point type from the global context.
impl FloatTypeRef {
    /// Creates a 16-bit floating-point (`half`) type in the global LLVM context.
    ///
    /// This function wraps the `LLVMHalfType` function from the LLVM core library. It creates and returns
    /// a 16-bit floating-point type, commonly known as `half`, within the global LLVM context. The `half` type is
    /// often used in graphics and machine learning applications where reduced precision is acceptable.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `half` type in the global context.
    #[must_use]
    pub fn half_type() -> Self {
        unsafe { Self(core::LLVMHalfType()) }
    }

    /// Creates a 16-bit floating-point (`bfloat`) type in the global LLVM context.
    ///
    /// This function wraps the `LLVMBFloatType` function from the LLVM core library. It creates and returns
    /// a 16-bit floating-point type, known as `bfloat`, within the global LLVM context. The `bfloat` type is commonly
    /// used in machine learning and other applications where reduced precision is sufficient, but a wider exponent range
    /// than `half` is required.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `bfloat` type in the global context.
    #[must_use]
    pub fn bfloat_type() -> Self {
        unsafe { Self(core::LLVMBFloatType()) }
    }

    /// Creates a 32-bit floating-point (`float`) type in the global LLVM context.
    ///
    /// This function wraps the `LLVMFloatType` function from the LLVM core library. It creates and returns
    /// a 32-bit floating-point type, commonly known as `float`, within the global LLVM context. The `float` type is
    /// the standard single-precision floating-point type used in many computations.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `float` type in the global context.
    #[must_use]
    pub fn float_type() -> Self {
        unsafe { Self(core::LLVMFloatType()) }
    }

    /// Creates a 64-bit floating-point (`double`) type in the global LLVM context.
    ///
    /// This function wraps the `LLVMDoubleType` function from the LLVM core library. It creates and returns
    /// a 64-bit floating-point type, commonly known as `double`, within the global LLVM context. The `double` type is
    /// the standard double-precision floating-point type used for higher precision calculations in many applications.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `double` type in the global context.
    #[must_use]
    pub fn double_type() -> Self {
        unsafe { Self(core::LLVMDoubleType()) }
    }

    /// Creates an 80-bit floating-point (`x86_fp80`) type in the global LLVM context.
    ///
    /// This function wraps the `LLVMX86FP80Type` function from the LLVM core library. It creates and returns
    /// an 80-bit floating-point type, known as `x86_fp80`, within the global LLVM context. This type is specific to
    /// x86 architecture and provides extended precision floating-point arithmetic, commonly used in legacy systems
    /// and certain numerical applications.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `x86_fp80` type in the global context.
    #[must_use]
    pub fn x86_fp80_type() -> Self {
        unsafe { Self(core::LLVMX86FP80Type()) }
    }

    /// Creates a 128-bit floating-point (`fp128`) type in the global LLVM context.
    ///
    /// This function wraps the `LLVMFP128Type` function from the LLVM core library. It creates and returns
    /// a 128-bit floating-point type, known as `fp128`, within the global LLVM context. The `fp128` type is used for
    /// very high precision floating-point arithmetic, though it is less commonly used compared to other floating-point types.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `fp128` type in the global context.
    #[must_use]
    pub fn fp128_type() -> Self {
        unsafe { Self(core::LLVMFP128Type()) }
    }

    /// Creates a 128-bit floating-point (`ppc_fp128`) type in the global LLVM context, specific to `PowerPC` architecture.
    ///
    /// This function wraps the `LLVMPPCFP128Type` function from the LLVM core library. It creates and returns
    /// a 128-bit floating-point type, known as `ppc_fp128`, within the global LLVM context. This type is specific to
    /// the `PowerPC` architecture and is used for high-precision floating-point arithmetic on `PowerPC` systems.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `ppc_fp128` type in the global context.
    #[must_use]
    pub fn ppc_fp128_type() -> Self {
        unsafe { Self(core::LLVMPPCFP128Type()) }
    }
}
