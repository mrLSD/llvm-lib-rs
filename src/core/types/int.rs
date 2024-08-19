//! Functions in this section operate on integer types.

use crate::core::context::ContextRef;
use crate::core::types::TypeRef;
use crate::GetRef;
use llvm_sys::core;
use llvm_sys::prelude::LLVMTypeRef;

/// Wrapper `LLVMTypeRef` for integer types.
#[derive(Debug)]
pub struct IntTypeRef(LLVMTypeRef);

impl From<LLVMTypeRef> for IntTypeRef {
    fn from(value: LLVMTypeRef) -> Self {
        Self(value)
    }
}

impl From<IntTypeRef> for TypeRef {
    fn from(value: IntTypeRef) -> Self {
        Self(value.0)
    }
}

impl GetRef for IntTypeRef {
    type RawRef = LLVMTypeRef;
    fn get_ref(&self) -> Self::RawRef {
        self.0
    }
}

/// Obtain an integer type from a context with specified bit width.
impl IntTypeRef {
    /// Creates a 1-bit integer (`i1`) type in the specified LLVM context.
    ///
    /// This function wraps the `LLVMInt1TypeInContext` function from the LLVM core library. It creates and returns
    /// a 1-bit integer type, known as `i1`, within the specified LLVM context. The `i1` type is typically used to represent
    /// boolean values (true/false) in LLVM IR.
    ///
    /// # Parameters
    ///
    /// - `context`: A reference to the `ContextRef` in which the `i1` type will be created.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `i1` type in the specified context.
    #[must_use]
    pub fn int1_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMInt1TypeInContext(context.get_ref())) }
    }

    /// Creates an 8-bit integer (`i8`) type in the specified LLVM context.
    ///
    /// This function wraps the `LLVMInt8TypeInContext` function from the LLVM core library. It creates and returns
    /// an 8-bit integer type, known as `i8`, within the specified LLVM context. The `i8` type is commonly used to
    /// represent small integer values, such as bytes or characters, in LLVM IR.
    ///
    /// # Parameters
    ///
    /// - `context`: A reference to the `ContextRef` in which the `i8` type will be created.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `i8` type in the specified context.
    #[must_use]
    pub fn int8_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMInt8TypeInContext(context.get_ref())) }
    }

    /// Creates a 16-bit integer (`i16`) type in the specified LLVM context.
    ///
    /// This function wraps the `LLVMInt16TypeInContext` function from the LLVM core library. It creates and returns
    /// a 16-bit integer type, known as `i16`, within the specified LLVM context. The `i16` type is commonly used to
    /// represent short integer values in LLVM IR.
    ///
    /// # Parameters
    ///
    /// - `context`: A reference to the `ContextRef` in which the `i16` type will be created.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `i16` type in the specified context.
    #[must_use]
    pub fn int16_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMInt16TypeInContext(context.get_ref())) }
    }

    /// Creates a 32-bit integer (`i32`) type in the specified LLVM context.
    ///
    /// This function wraps the `LLVMInt32TypeInContext` function from the LLVM core library. It creates and returns
    /// a 32-bit integer type, known as `i32`, within the specified LLVM context. The `i32` type is one of the most commonly
    /// used integer types in LLVM IR, representing standard integer values in many applications.
    ///
    /// # Parameters
    ///
    /// - `context`: A reference to the `ContextRef` in which the `i32` type will be created.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `i32` type in the specified context.
    #[must_use]
    pub fn int32_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMInt32TypeInContext(context.get_ref())) }
    }

    /// Creates a 64-bit integer (`i64`) type in the specified LLVM context.
    ///
    /// This function wraps the `LLVMInt64TypeInContext` function from the LLVM core library. It creates and returns
    /// a 64-bit integer type, known as `i64`, within the specified LLVM context. The `i64` type is commonly used in
    /// LLVM IR to represent large integer values, including pointers on 64-bit architectures.
    ///
    /// # Parameters
    ///
    /// - `context`: A reference to the `ContextRef` in which the `i64` type will be created.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `i64` type in the specified context.
    #[must_use]
    pub fn int64_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMInt64TypeInContext(context.get_ref())) }
    }

    /// Creates a 128-bit integer (`i128`) type in the specified LLVM context.
    ///
    /// This function wraps the `LLVMInt128TypeInContext` function from the LLVM core library. It creates and returns
    /// a 128-bit integer type, known as `i128`, within the specified LLVM context. The `i128` type is used in LLVM IR to represent
    /// very large integer values, providing extended precision for certain applications that require it.
    ///
    /// # Parameters
    ///
    /// - `context`: A reference to the `ContextRef` in which the `i128` type will be created.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `i128` type in the specified context.
    #[must_use]
    pub fn int128_type_in_context(context: &ContextRef) -> Self {
        unsafe { Self(core::LLVMInt128TypeInContext(context.get_ref())) }
    }

    /// Creates an integer type with a specified bit width in the given LLVM context.
    ///
    /// This function wraps the `LLVMIntTypeInContext` function from the LLVM core library. It creates and returns
    /// an integer type with the specified number of bits (`num_bits`) within the specified LLVM context. This allows
    /// the creation of integer types of arbitrary bit widths, tailored to the specific needs of the application.
    ///
    /// # Parameters
    ///
    /// - `context`: A reference to the `ContextRef` in which the integer type will be created.
    /// - `num_bits`: The bit width of the integer type to be created. For example, 8 for `i8`, 32 for `i32`, etc.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the integer type with the specified bit width in the given context.
    #[must_use]
    pub fn int_type_in_context(context: &ContextRef, num_bits: u32) -> Self {
        unsafe { Self(core::LLVMIntTypeInContext(context.get_ref(), num_bits)) }
    }
}

/// Obtain an integer type from the global context with a specified bit width.
impl IntTypeRef {
    /// Creates a 1-bit integer (`i1`) type in the global LLVM context.
    ///
    /// This function wraps the `LLVMInt1Type` function from the LLVM core library. It creates and returns
    /// a 1-bit integer type, known as `i1`, within the global LLVM context. The `i1` type is typically used to represent
    /// boolean values (true/false) in LLVM IR.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `i1` type in the global context.
    #[must_use]
    pub fn int1_type() -> Self {
        unsafe { Self(core::LLVMInt1Type()) }
    }

    /// Creates an 8-bit integer (`i8`) type in the global LLVM context.
    ///
    /// This function wraps the `LLVMInt8Type` function from the LLVM core library. It creates and returns
    /// an 8-bit integer type, known as `i8`, within the global LLVM context. The `i8` type is commonly used to
    /// represent small integer values, such as bytes or characters, in LLVM IR.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `i8` type in the global context.
    #[must_use]
    pub fn int8_type() -> Self {
        unsafe { Self(core::LLVMInt8Type()) }
    }

    /// Creates a 16-bit integer (`i16`) type in the global LLVM context.
    ///
    /// This function wraps the `LLVMInt16Type` function from the LLVM core library. It creates and returns
    /// a 16-bit integer type, known as `i16`, within the global LLVM context. The `i16` type is commonly used to
    /// represent short integer values in LLVM IR.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `i16` type in the global context.
    #[must_use]
    pub fn int16_type() -> Self {
        unsafe { Self(core::LLVMInt16Type()) }
    }

    /// Creates a 32-bit integer (`i32`) type in the global LLVM context.
    ///
    /// This function wraps the `LLVMInt32Type` function from the LLVM core library. It creates and returns
    /// a 32-bit integer type, known as `i32`, within the global LLVM context. The `i32` type is one of the most commonly
    /// used integer types in LLVM IR, representing standard integer values in many applications.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `i32` type in the global context.
    #[must_use]
    pub fn int32_type() -> Self {
        unsafe { Self(core::LLVMInt32Type()) }
    }

    /// Creates a 64-bit integer (`i64`) type in the global LLVM context.
    ///
    /// This function wraps the `LLVMInt64Type` function from the LLVM core library. It creates and returns
    /// a 64-bit integer type, known as `i64`, within the global LLVM context. The `i64` type is commonly used in
    /// LLVM IR to represent large integer values, including pointers on 64-bit architectures.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `i64` type in the global context.
    #[must_use]
    pub fn int64_type() -> Self {
        unsafe { Self(core::LLVMInt64Type()) }
    }

    /// Creates a 128-bit integer (`i128`) type in the global LLVM context.
    ///
    /// This function wraps the `LLVMInt128Type` function from the LLVM core library. It creates and returns
    /// a 128-bit integer type, known as `i128`, within the global LLVM context. The `i128` type is used in LLVM IR to represent
    /// very large integer values, providing extended precision for certain applications that require it.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the `i128` type in the global context.
    #[must_use]
    pub fn int128_type() -> Self {
        unsafe { Self(core::LLVMInt128Type()) }
    }

    /// Creates an integer type with a specified bit width in the global LLVM context.
    ///
    /// This function wraps the `LLVMIntType` function from the LLVM core library. It creates and returns
    /// an integer type with the specified number of bits (`num_bits`) within the global LLVM context. This allows
    /// the creation of integer types of arbitrary bit widths, tailored to the specific needs of the application.
    ///
    /// # Parameters
    ///
    /// - `num_bits`: The bit width of the integer type to be created. For example, 8 for `i8`, 32 for `i32`, etc.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the integer type with the specified bit width in the global context.
    #[must_use]
    pub fn int_type(num_bits: u32) -> Self {
        unsafe { Self(core::LLVMIntType(num_bits)) }
    }

    /// Retrieves the bit width of an integer type.
    ///
    /// This function wraps the `LLVMGetIntTypeWidth` function from the LLVM core library. It returns the bit width
    /// of the integer type represented by `self`. This is useful for determining the size of an integer type in bits,
    /// such as whether it is an 8-bit, 32-bit, 64-bit, or other integer type.
    ///
    /// # Returns
    ///
    /// Returns a `u32` representing the bit width of the integer type.
    #[must_use]
    pub fn get_int_type_width(&self) -> u32 {
        unsafe { core::LLVMGetIntTypeWidth(self.0) }
    }
}
