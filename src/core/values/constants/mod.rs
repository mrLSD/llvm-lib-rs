//! This section contains APIs for interacting with `MValueRef` that
//! correspond to `LLVM Constant` instances.

pub mod composite;
pub mod expressions;
pub mod scalar;

use super::ValueRef;
use crate::core::types::TypeRef;
use crate::GetRef;
use llvm_sys::core;

impl ValueRef {
    /// Obtain a constant value referring to the null instance of a type.
    ///
    /// # Details
    ///
    /// Creates a constant 'null' value of the specified type.
    ///
    /// This function wraps the `LLVMConstNull` function from the LLVM core library. It generates a constant
    /// 'null' value for the specified type (`ty`). This is typically used for pointer types, where the null value
    /// represents a pointer to no valid memory location, but it can also be used for other types where a zero-initialized
    /// constant is required.
    ///
    /// # Parameters
    ///
    /// - `ty`: A reference to the type (`TypeRef`) for which the null value should be created. This type determines the kind of null value that is generated (e.g., null pointer, zero-initialized struct).
    ///
    /// # Returns
    ///
    /// Returns an instance of `ValueRef`, which encapsulates the constant null value for the specified type.
    #[must_use]
    pub fn const_null(ty: &TypeRef) -> Self {
        unsafe { Self(core::LLVMConstNull(ty.get_ref())) }
    }

    /// Obtain a constant value referring to the instance of a type
    /// consisting of all ones.
    ///
    /// This is only valid for integer types.
    ///
    /// # Details
    ///
    /// Creates a constant value with all bits set to one for the specified type.
    ///
    /// This function wraps the `LLVMConstAllOnes` function from the LLVM core library. It generates a constant value
    /// for the specified type (`ty`) where all the bits are set to one. This is often used to represent a value where
    /// all bits are enabled, such as `-1` for signed integers or the maximum possible value for unsigned integers.
    ///
    /// # Parameters
    ///
    /// - `ty`: A reference to the type (`TypeRef`) for which the all-ones value should be created. This type determines the size and nature of the value (e.g., integer, vector).
    ///
    /// # Returns
    ///
    /// Returns an instance of `ValueRef`, which encapsulates the constant all-ones value for the specified type.
    #[must_use]
    pub fn const_all_ones(ty: &TypeRef) -> Self {
        unsafe { Self(core::LLVMConstAllOnes(ty.get_ref())) }
    }

    /// Obtain a constant value referring to an undefined value of a type.
    ///
    /// # Details
    ///
    /// Creates an 'undefined' value of the specified type.
    ///
    /// This function wraps the `LLVMGetUndef` function from the LLVM core library. It generates a constant
    /// 'undefined' value for the specified type (`ty`). An undefined value is a placeholder that can take any value of the specified type
    /// during program execution, typically used in optimization phases or when a value does not need to be initialized.
    ///
    /// # Parameters
    ///
    /// - `ty`: A reference to the type (`TypeRef`) for which the undefined value should be created. This type determines the nature of the undefined value (e.g., integer, floating-point, vector).
    ///
    /// # Returns
    ///
    /// Returns an instance of `ValueRef`, which encapsulates the constant undefined value for the specified type.
    #[must_use]
    pub fn get_undef(ty: &TypeRef) -> Self {
        unsafe { Self(core::LLVMGetUndef(ty.get_ref())) }
    }

    /// Obtain a constant value referring to a poison value of a type.
    ///
    /// # Details
    ///
    /// Creates a 'poison' value of the specified type.
    ///
    /// This function wraps the `LLVMGetPoison` function from the LLVM core library. It generates a constant
    /// 'poison' value for the specified type (`ty`). A poison value is similar to an undefined value but more strict;
    /// it is used to represent a value that results from an operation with undefined behavior. Using a poison value in further
    /// operations can propagate the poison, potentially leading to further undefined behavior.
    ///
    /// # Parameters
    ///
    /// - `ty`: A reference to the type (`TypeRef`) for which the poison value should be created. This type determines the nature of the poison value (e.g., integer, floating-point, vector).
    ///
    /// # Returns
    ///
    /// Returns an instance of `ValueRef`, which encapsulates the constant poison value for the specified type.
    #[must_use]
    pub fn get_poison(ty: &TypeRef) -> Self {
        unsafe { Self(core::LLVMGetPoison(ty.get_ref())) }
    }

    /// Determine whether a value instance is null.
    ///
    /// # Details
    ///
    /// Checks if the value is a constant 'null' value.
    ///
    /// This function wraps the `LLVMIsNull` function from the LLVM core library. It determines whether
    /// the value represented by `self` is a constant 'null' value. This is typically used to check if a pointer
    /// or other nullable type is explicitly set to null within the LLVM IR.
    ///
    /// # Returns
    ///
    /// Returns `true` if the value is a constant null value, otherwise returns `false`.
    #[must_use]
    pub fn is_null(&self) -> bool {
        unsafe { core::LLVMIsNull(self.0) != 0 }
    }

    /// Obtain a constant that is a constant pointer pointing to `NULL` for a
    /// specified type.
    ///
    /// # Details
    ///
    /// Creates a constant null pointer value of the specified pointer type.
    ///
    /// This function wraps the `LLVMConstPointerNull` function from the LLVM core library. It generates a constant
    /// null pointer for the specified pointer type (`ty`). This is typically used to represent a pointer that does not
    /// point to any valid memory location within LLVM IR.
    ///
    /// # Parameters
    ///
    /// - `ty`: A reference to the pointer type (`TypeRef`) for which the null pointer value should be created. This type must be a pointer type.
    ///
    /// # Returns
    ///
    /// Returns an instance of `ValueRef`, which encapsulates the constant null pointer value for the specified type.
    #[must_use]
    pub fn const_pointer_null(ty: &TypeRef) -> Self {
        unsafe { Self(core::LLVMConstPointerNull(ty.get_ref())) }
    }
}
