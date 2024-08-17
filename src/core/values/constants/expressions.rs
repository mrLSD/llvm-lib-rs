//! Functions in this group operate on constants expressions.
//!
//! This module provides a set of functions and methods for operating on constants expressions
//! within the LLVM framework.
//!
//! The functions in this module allow for various operations and manipulations of constant
//! values, such as obtaining the alignment or size of a type, performing arithmetic
//! operations, and creating constant expressions. These operations are essential for
//! low-level programming tasks that require precise control over memory layout and
//! arithmetic behavior.
//!
//! The module includes:
//!
//! - **Opcode Retrieval**: Functions to get the opcode of a constant expression.
//! - **Type Information**: Functions to obtain the alignment and size of a type.
//! - **Arithmetic Operations**: Functions to perform negation, addition, subtraction, multiplication, and other arithmetic operations on constant values, with support for `NSW` (No Signed Wrap) and `NUW` (No Unsigned Wrap) flags.
//! - **Logical Operations**: Functions to perform logical NOT and XOR operations on constant values.
//! - **Comparison Operations**: Functions to perform integer and floating-point comparisons on constant values.
//! - **Bitwise Operations**: Functions to perform bitwise operations such as left shift and bit-cast on constant values.
//! - **Pointer Operations**: Functions to convert between pointers and integers, and to perform address space casts.
//! - **Vector Operations**: Functions to extract and insert elements in vector constants, and to create shuffle vector operations.
//! - **Block Addressing**: Functions to obtain the address of a basic block in a function.
//!
//! These functions wrap the corresponding LLVM core library functions, providing a safe and idiomatic Rust interface for interacting with LLVM constants.

use super::ValueRef;
use crate::basic_block::BasicBlockRef;
use crate::core::types::TypeRef;
use crate::core::{IntPredicate, Opcode, RealPredicate};
use crate::{CUint, GetRef};
use llvm_sys::core;

impl ValueRef {
    /// Get the opcode for a constant value.
    ///
    /// # Details
    ///
    /// Retrieves the opcode of a constant expression.
    ///
    /// This function wraps the `LLVMGetConstOpcode` function from the LLVM core library, which returns
    /// the opcode (operation code) for a constant expression. The opcode indicates the specific
    /// operation that the constant expression represents, such as addition, multiplication, etc.
    ///
    /// # Returns
    ///
    /// Returns an `Opcode` enum value that represents the opcode of the constant expression. The
    /// `Opcode` enum provides a Rust-friendly abstraction over the raw opcode value returned by
    /// LLVM.
    #[must_use]
    pub fn get_const_opcode(&self) -> Opcode {
        unsafe { Opcode::from(core::LLVMGetConstOpcode(self.0)) }
    }

    /// Obtain the alignment of the specified type.
    ///
    /// # Details
    ///
    /// Creates a new constant integer value representing the alignment, in bytes, of a given type.
    ///
    /// This function wraps the `LLVMAlignOf` function from the LLVM core library, which returns the alignment
    /// of the provided type in bytes as a constant integer value. Alignment is the byte boundary
    /// that the type must adhere to in memory, and understanding it is important for certain
    /// low-level operations.
    ///
    /// # Arguments
    ///
    /// * `ty` - A reference to the `TypeRef` representing the type whose alignment is being queried.
    ///
    /// # Returns
    ///
    /// Returns a new constant integer value representing the alignment of the specified type in bytes.
    #[must_use]
    pub fn align_of(ty: &TypeRef) -> Self {
        unsafe { Self(core::LLVMAlignOf(ty.get_ref())) }
    }

    /// Obtain the size of the specified type.
    ///
    /// # Details
    ///
    /// Creates a new constant integer value representing the size, in bytes, of a given type.
    ///
    /// This function wraps the `LLVMSizeOf` function from the LLVM core library, which returns the size
    /// of the provided type in bytes as a constant integer value. This can be useful for operations
    /// that require knowledge of the memory footprint of a particular type.
    ///
    /// # Arguments
    ///
    /// * `ty` - A reference to the `TypeRef` representing the type whose size is being queried.
    ///
    /// # Returns
    ///
    /// Returns a new constant integer value representing the size of the specified type in bytes.
    #[must_use]
    pub fn size_of(ty: &TypeRef) -> Self {
        unsafe { Self(core::LLVMSizeOf(ty.get_ref())) }
    }

    /// Create a negation operation on a constant value.
    ///
    /// # Details
    ///
    /// Creates a new constant integer value representing the arithmetic negation
    /// of the original value.
    ///
    /// This function wraps the `LLVMConstNeg` function from the LLVM core library, which
    /// computes the negation of the given constant value (`-self`).
    ///
    /// # Returns
    ///
    /// Returns a new constant integer value representing the result of the negation
    /// operation (`-self`).
    #[must_use]
    pub fn const_neg(&self) -> Self {
        unsafe { Self(core::LLVMConstNeg(self.0)) }
    }

    /// Create a `NSW` negation operation on a constant value.
    ///
    /// # Details
    ///
    /// Creates a new constant integer value representing the arithmetic negation
    /// of the original value with the `nsw` (No Signed Wrap) flag set.
    ///
    /// The `nsw` flag indicates that signed overflow is not allowed, and if it occurs,
    /// the program's behavior will be undefined. This allows LLVM to optimize the code
    /// under the assumption that overflow does not happen.
    ///
    /// # Returns
    ///
    /// Returns a new constant integer value representing the result of the negation
    /// operation (`-self`) with the `nsw` flag set.
    #[must_use]
    pub fn const_nsw_neg(&self) -> Self {
        unsafe { Self(core::LLVMConstNSWNeg(self.0)) }
    }

    /// Create a `NUW` negation operation on a constant value.
    ///
    /// # Details
    ///
    /// Creates a new constant integer value representing the arithmetic negation
    /// of the original value, with the `nuw` (No Unsigned Wrap) flag set.
    ///
    /// The `nuw` flag indicates that unsigned overflow is not allowed, and if it occurs,
    /// the program's behavior will be undefined. This allows LLVM to optimize the code
    /// under the assumption that overflow does not happen during the negation operation.
    ///
    /// # Returns
    ///
    /// Returns a new constant integer value representing the result of the negation
    /// operation (`-self`) with the `nuw` flag set.
    #[must_use]
    pub fn const_nuw_neg(&self) -> Self {
        unsafe { Self(core::LLVMConstNUWNeg(self.0)) }
    }

    /// Create a logical NOT operation on a constant value.
    ///
    /// # Details
    ///
    /// Creates a new constant integer value representing the bitwise negation (NOT) of the original value.
    ///
    /// This function wraps the `LLVMConstNot` function from the LLVM core library, which computes the bitwise
    /// complement of the given constant integer value (`~self`). The result is a new constant where each
    /// bit of the original value is inverted (i.e., `0` becomes `1` and `1` becomes `0`).
    ///
    /// # Returns
    ///
    /// Returns a new constant integer value representing the result of the bitwise NOT operation (`~self`).
    #[must_use]
    pub fn const_not(&self) -> Self {
        unsafe { Self(core::LLVMConstNot(self.0)) }
    }

    /// Create an addition operation on two constant values.
    ///
    /// # Details
    ///
    /// Creates a new constant integer value representing the addition of two constant integer values.
    ///
    /// This function wraps the `LLVMConstAdd` function from the LLVM core library, which performs the addition
    /// of two constant integer values and returns the result as a new constant value.
    ///
    /// # Arguments
    ///
    /// * `lhs` - A reference to the left-hand side (LHS) constant integer value.
    /// * `rhs` - A reference to the right-hand side (RHS) constant integer value.
    ///
    /// # Returns
    ///
    /// Returns a new constant integer value representing the sum of `lhs` and `rhs`.
    #[must_use]
    pub fn const_add(lhs: &Self, rhs: &Self) -> Self {
        unsafe { Self(core::LLVMConstAdd(lhs.0, rhs.0)) }
    }

    /// Create a NSW (No Signed Wrap) addition operation on two constant values.
    ///
    /// # Details
    ///
    /// Creates a new constant integer value representing the addition of two constant integer values,
    /// with the `nsw` (No Signed Wrap) flag set.
    ///
    /// This function wraps the `LLVMConstNSWAdd` function from the LLVM core library, which performs the addition
    /// of two constant integer values and returns the result as a new constant value. The `nsw` flag
    /// indicates that signed overflow is not allowed, and if it occurs, the program's behavior will be undefined.
    /// This allows LLVM to optimize the code under the assumption that overflow does not happen.
    ///
    /// # Arguments
    ///
    /// * `lhs` - A reference to the left-hand side (LHS) constant integer value.
    /// * `rhs` - A reference to the right-hand side (RHS) constant integer value.
    ///
    /// # Returns
    ///
    /// Returns a new constant integer value representing the sum of `lhs` and `rhs` with the `nsw` flag set.
    #[must_use]
    pub fn const_nsw_add(lhs: &Self, rhs: &Self) -> Self {
        unsafe { Self(core::LLVMConstNSWAdd(lhs.0, rhs.0)) }
    }

    /// Create a NUW (No Unsigned Wrap) addition operation on two constant values.
    ///
    /// # Details
    ///
    /// Creates a new constant integer value representing the addition of two constant integer values,
    /// with the `nuw` (No Unsigned Wrap) flag set.
    ///
    /// This function wraps the `LLVMConstNUWAdd` function from the LLVM core library, which performs the addition
    /// of two constant integer values and returns the result as a new constant value. The `nuw` flag
    /// indicates that unsigned overflow is not allowed, and if it occurs, the program's behavior will be undefined.
    /// This allows LLVM to optimize the code under the assumption that overflow does not happen.
    ///
    /// # Arguments
    ///
    /// * `lhs` - A reference to the left-hand side (LHS) constant integer value.
    /// * `rhs` - A reference to the right-hand side (RHS) constant integer value.
    ///
    /// # Returns
    ///
    /// Returns a new constant integer value representing the sum of `lhs` and `rhs` with the `nuw` flag set.
    #[must_use]
    pub fn const_nuw_add(lhs: &Self, rhs: &Self) -> Self {
        unsafe { Self(core::LLVMConstNUWAdd(lhs.0, rhs.0)) }
    }

    /// Create a subtraction operation on two constant values.
    ///
    /// # Details
    ///
    /// Creates a new constant integer value representing the subtraction of two constant integer values.
    ///
    /// This function wraps the `LLVMConstSub` function from the LLVM core library, which performs the subtraction
    /// of the right-hand side (RHS) constant integer value from the left-hand side (LHS) constant integer value
    /// and returns the result as a new constant value.
    ///
    /// # Arguments
    ///
    /// * `lhs` - A reference to the left-hand side (LHS) constant integer value.
    /// * `rhs` - A reference to the right-hand side (RHS) constant integer value.
    ///
    /// # Returns
    ///
    /// Returns a new constant integer value representing the result of subtracting `rhs` from `lhs`.
    #[must_use]
    pub fn const_sub(lhs: &Self, rhs: &Self) -> Self {
        unsafe { Self(core::LLVMConstSub(lhs.0, rhs.0)) }
    }

    /// Create a NSW (No Signed Wrap) subtraction operation on two constant values.
    ///
    /// # Details
    ///
    /// Creates a new constant integer value representing the subtraction of two constant integer values,
    /// with the `nsw` (No Signed Wrap) flag set.
    ///
    /// This function wraps the `LLVMConstNSWSub` function from the LLVM core library, which performs the subtraction
    /// of the right-hand side (RHS) constant integer value from the left-hand side (LHS) constant integer value
    /// and returns the result as a new constant value. The `nsw` flag indicates that signed overflow is not allowed,
    /// and if it occurs, the program's behavior will be undefined. This allows LLVM to optimize the code under the
    /// assumption that overflow does not happen during the subtraction.
    ///
    /// # Arguments
    ///
    /// * `lhs` - A reference to the left-hand side (LHS) constant integer value.
    /// * `rhs` - A reference to the right-hand side (RHS) constant integer value.
    ///
    /// # Returns
    ///
    /// Returns a new constant integer value representing the result of subtracting `rhs` from `lhs` with the `nsw` flag set.
    #[must_use]
    pub fn const_nsw_sub(lhs: &Self, rhs: &Self) -> Self {
        unsafe { Self(core::LLVMConstNSWSub(lhs.0, rhs.0)) }
    }

    /// Create a NUW (No Unsigned Wrap) subtraction operation on two constant values.
    ///
    /// # Details
    ///
    /// Creates a new constant integer value representing the subtraction of two constant integer values,
    /// with the `nuw` (No Unsigned Wrap) flag set.
    ///
    /// This function wraps the `LLVMConstNUWSub` function from the LLVM core library, which performs the subtraction
    /// of the right-hand side (RHS) constant integer value from the left-hand side (LHS) constant integer value
    /// and returns the result as a new constant value. The `nuw` flag indicates that unsigned overflow is not allowed,
    /// and if it occurs, the program's behavior will be undefined. This allows LLVM to optimize the code under the
    /// assumption that overflow does not happen during the subtraction.
    ///
    /// # Arguments
    ///
    /// * `lhs` - A reference to the left-hand side (LHS) constant integer value.
    /// * `rhs` - A reference to the right-hand side (RHS) constant integer value.
    ///
    /// # Returns
    ///
    /// Returns a new constant integer value representing the result of subtracting `rhs` from `lhs` with the `nuw` flag set.
    #[must_use]
    pub fn const_nuw_sub(lhs: &Self, rhs: &Self) -> Self {
        unsafe { Self(core::LLVMConstNUWSub(lhs.0, rhs.0)) }
    }

    /// Create a multiplication operation on two constant values.
    ///
    /// # Details
    ///
    /// Creates a new constant integer value representing the multiplication of two constant integer values.
    ///
    /// This function wraps the `LLVMConstMul` function from the LLVM core library, which performs the multiplication
    /// of two constant integer values and returns the result as a new constant value.
    ///
    /// # Arguments
    ///
    /// * `lhs` - A reference to the left-hand side (LHS) constant integer value.
    /// * `rhs` - A reference to the right-hand side (RHS) constant integer value.
    ///
    /// # Returns
    ///
    /// Returns a new constant integer value representing the product of `lhs` and `rhs`.
    #[must_use]
    pub fn const_mul(lhs: &Self, rhs: &Self) -> Self {
        unsafe { Self(core::LLVMConstMul(lhs.0, rhs.0)) }
    }

    /// Create a NSW (No Signed Wrap) multiplication operation on two constant values.
    ///
    /// # Details
    ///
    /// Creates a new constant integer value representing the multiplication of two constant integer values,
    /// with the `nsw` (No Signed Wrap) flag set.
    ///
    /// This function wraps the `LLVMConstNSWMul` function from the LLVM core library, which performs the multiplication
    /// of two constant integer values and returns the result as a new constant value. The `nsw` flag indicates that
    /// signed overflow is not allowed, and if it occurs, the program's behavior will be undefined. This allows LLVM
    /// to optimize the code under the assumption that overflow does not happen during the multiplication.
    ///
    /// # Arguments
    ///
    /// * `lhs` - A reference to the left-hand side (LHS) constant integer value.
    /// * `rhs` - A reference to the right-hand side (RHS) constant integer value.
    ///
    /// # Returns
    ///
    /// Returns a new constant integer value representing the product of `lhs` and `rhs` with the `nsw` flag set.
    #[must_use]
    pub fn const_nsw_mul(lhs: &Self, rhs: &Self) -> Self {
        unsafe { Self(core::LLVMConstNSWMul(lhs.0, rhs.0)) }
    }

    /// Create a NUW (No Unsigned Wrap) multiplication operation on two constant values.
    ///
    /// # Details
    ///
    /// Creates a new constant integer value representing the multiplication of two constant integer values,
    /// with the `nuw` (No Unsigned Wrap) flag set.
    ///
    /// This function wraps the `LLVMConstNUWMul` function from the LLVM core library, which performs the multiplication
    /// of two constant integer values and returns the result as a new constant value. The `nuw` flag indicates that
    /// unsigned overflow is not allowed, and if it occurs, the program's behavior will be undefined. This allows LLVM
    /// to optimize the code under the assumption that overflow does not happen during the multiplication.
    ///
    /// # Arguments
    ///
    /// * `lhs` - A reference to the left-hand side (LHS) constant integer value.
    /// * `rhs` - A reference to the right-hand side (RHS) constant integer value.
    ///
    /// # Returns
    ///
    /// Returns a new constant integer value representing the product of `lhs` and `rhs` with the `nuw` flag set.
    #[must_use]
    pub fn const_nuw_mul(lhs: &Self, rhs: &Self) -> Self {
        unsafe { Self(core::LLVMConstNUWMul(lhs.0, rhs.0)) }
    }

    /// Create a logical XOR operation on two constant values.
    ///
    /// # Details
    ///
    /// Creates a new constant integer value representing the bitwise XOR (exclusive OR) of two constant integer values.
    ///
    /// This function wraps the `LLVMConstXor` function from the LLVM core library, which performs the bitwise XOR operation
    /// between two constant integer values and returns the result as a new constant value. The XOR operation compares
    /// each corresponding bit of the two values, setting the resulting bit to `1` if the bits differ, and to `0` if
    /// they are the same.
    ///
    /// # Arguments
    ///
    /// * `lhs` - A reference to the left-hand side (LHS) constant integer value.
    /// * `rhs` - A reference to the right-hand side (RHS) constant integer value.
    ///
    /// # Returns
    ///
    /// Returns a new constant integer value representing the result of the bitwise XOR operation between `lhs` and `rhs`.
    #[must_use]
    pub fn const_xor(lhs: &Self, rhs: &Self) -> Self {
        unsafe { Self(core::LLVMConstXor(lhs.0, rhs.0)) }
    }

    /// Create an integer comparison operation on two constant values.
    ///
    /// # Details
    ///
    /// Performs a constant integer comparison between two values using a specified comparison predicate.
    ///
    /// This function is a wrapper around the `LLVMConstICmp` function from the LLVM core library.
    /// It allows you to perform a comparison between two integer values (`lhs` and `rhs`) at compile time,
    /// returning a new value that represents the result of the comparison. The comparison is specified
    /// by the `predicate`, which determines the type of comparison to be made (e.g., equality, less than, greater than).
    ///
    /// # Parameters
    ///
    /// - `predicate`: An instance of [`IntPredicate`] that specifies the kind of comparison to perform.
    ///   This could be one of several variants like `IntEQ` (equal), `IntNE` (not equal), `IntUGT` (unsigned greater than),
    ///   and others, depending on the type of integer comparison desired.
    /// - `lhs`: A reference to the left-hand side value (`lhs`) of the comparison. This is the first integer value to compare.
    /// - `rhs`: A reference to the right-hand side value (`rhs`) of the comparison. This is the second integer value to compare.
    ///
    /// # Returns
    ///
    /// Returns an instance of `ValueRef`, which encapsulates the result of the comparison. The result is a constant
    /// value determined at compile time based on the specified `predicate`.
    #[must_use]
    pub fn const_icmp(predicate: IntPredicate, lhs: &Self, rhs: &Self) -> Self {
        unsafe { Self(core::LLVMConstICmp(predicate.into(), lhs.0, rhs.0)) }
    }

    /// Create a floating-point comparison operation on two constant values.
    ///
    /// # Details
    ///
    /// Performs a constant floating-point comparison between two values using a specified comparison predicate.
    ///
    /// This function is a wrapper around the `LLVMConstFCmp` function from the LLVM core library.
    /// It allows you to perform a comparison between two floating-point values (`lhs` and `rhs`) at compile time,
    /// returning a new value that represents the result of the comparison. The comparison is specified
    /// by the `predicate`, which determines the type of comparison to be made (e.g., equal to, less than, greater than).
    ///
    /// # Parameters
    ///
    /// - `predicate`: An instance of [`RealPredicate`] that specifies the kind of comparison to perform.
    ///   This could be one of several variants like `RealPredicate::RealOEQ` (ordered and equal),
    ///   `RealPredicate::RealOLT` (ordered and less than), `RealPredicate::RealUGT` (unordered and greater than), etc.
    ///   The exact variant determines the nature of the floating-point comparison.
    /// - `lhs`: A reference to the left-hand side value (`lhs`) of the comparison. This is the first floating-point value to compare.
    /// - `rhs`: A reference to the right-hand side value (`rhs`) of the comparison. This is the second floating-point value to compare.
    ///
    /// # Returns
    ///
    /// Returns an instance of `ValueRef`, which encapsulates the result of the comparison. The result is a constant
    /// value determined at compile time based on the specified `predicate`.
    #[must_use]
    pub fn const_fcmp(predicate: RealPredicate, lhs: &Self, rhs: &Self) -> Self {
        unsafe { Self(core::LLVMConstFCmp(predicate.into(), lhs.0, rhs.0)) }
    }

    /// Create a left shift operation on two constant values.
    ///
    /// # Details
    ///
    /// Performs a constant bitwise left shift operation between two integer values.
    ///
    /// This function is a wrapper around the `LLVMConstShl` function from the LLVM core library.
    /// It allows you to perform a left shift operation on two integer values (`lhs` and `rhs`) at compile time,
    /// returning a new value that represents the result of the shift. The left-hand side value (`lhs`) is shifted
    /// left by the number of bits specified by the right-hand side value (`rhs`).
    ///
    /// # Parameters
    ///
    /// - `lhs`: A reference to the left-hand side value (`lhs`) of the shift operation. This is the integer value that will be shifted.
    /// - `rhs`: A reference to the right-hand side value (`rhs`) of the shift operation. This is the integer value that specifies the number of bits to shift.
    ///
    /// # Returns
    ///
    /// Returns an instance of `ValueRef`, which encapsulates the result of the left shift operation. The result is a constant
    /// value determined at compile time based on shifting `lhs` to the left by `rhs` bits.
    #[must_use]
    pub fn const_shl(lhs: &Self, rhs: &Self) -> Self {
        unsafe { Self(core::LLVMConstShl(lhs.0, rhs.0)) }
    }

    /// Create a GEP (`GetElementPtr`) operation on a constant value.
    ///
    /// # Details
    ///
    /// Creates a constant `GetElementPtr` (GEP) instruction with an explicit type.
    ///
    /// This function wraps the `LLVMConstGEP2` function from the LLVM core library. It generates a constant
    /// `GEP` instruction, which calculates the address of a sub-element of an aggregate data structure (such as
    /// arrays or structs) at compile time. The `GEP` is calculated using the base pointer `constant_val` and the
    /// specified `constant_indices`.
    ///
    /// # Parameters
    ///
    /// - `ty`: A reference to the type of the base pointer (`constant_val`). This specifies the type of the data structure from which the `GEP` is calculated.
    /// - `constant_val`: A reference to the base value from which the GEP is calculated. This is typically a pointer to an aggregate data structure.
    /// - `constant_indices`: A slice of references to constant values that represent the indices used in the GEP calculation.
    ///
    /// # Returns
    ///
    /// Returns an instance of `ValueRef`, which encapsulates the result of the `GEP` calculation. The result is a constant
    /// value determined at compile time, representing the address of the sub-element within the aggregate data structure.
    #[must_use]
    pub fn const_gep2(ty: &TypeRef, constant_val: &Self, constant_indices: &[Self]) -> Self {
        let mut constant_indices = constant_indices.iter().map(|v| v.0).collect::<Vec<_>>();
        let constant_indices_ptr = if constant_indices.is_empty() {
            std::ptr::null_mut()
        } else {
            constant_indices.as_mut_ptr()
        };
        unsafe {
            Self(core::LLVMConstGEP2(
                ty.get_ref(),
                constant_val.0,
                constant_indices_ptr,
                *CUint::from(constant_indices.len()),
            ))
        }
    }

    /// Create an in-bounds GEP (`GetElementPtr`) operation on a constant value.
    ///
    /// # Details
    ///
    /// Creates a constant in-bounds `GetElementPtr` (GEP) instruction with an explicit type.
    ///
    /// This function wraps the `LLVMConstInBoundsGEP2` function from the LLVM core library. It generates a constant
    /// in-bounds `GEP` instruction, which calculates the address of a sub-element of an aggregate data structure (such as
    /// arrays or structs) at compile time. The in-bounds `GEP` ensures that the resulting address is within the bounds
    /// of the allocated object, allowing for more aggressive optimizations.
    ///
    /// # Parameters
    ///
    /// - `ty`: A reference to the type of the base pointer (`constant_val`). This specifies the type of the data structure from which the `GEP` is calculated.
    /// - `constant_val`: A reference to the base value from which the `GEP` is calculated. This is typically a pointer to an aggregate data structure.
    /// - `constant_indices`: A slice of references to constant values that represent the indices used in the `GEP` calculation.
    ///
    /// # Returns
    ///
    /// Returns an instance of `ValueRef`, which encapsulates the result of the in-bounds `GEP` calculation. The result is a constant
    /// value determined at compile time, representing the address of the sub-element within the aggregate data structure,
    /// with the guarantee that the address is within the bounds of the object.
    #[must_use]
    pub fn const_in_bounds_gep2(
        ty: &TypeRef,
        constant_val: &Self,
        constant_indices: &[Self],
    ) -> Self {
        let mut constant_indices = constant_indices.iter().map(|v| v.0).collect::<Vec<_>>();
        let constant_indices_ptr = if constant_indices.is_empty() {
            std::ptr::null_mut()
        } else {
            constant_indices.as_mut_ptr()
        };
        unsafe {
            Self(core::LLVMConstInBoundsGEP2(
                ty.get_ref(),
                constant_val.0,
                constant_indices_ptr,
                *CUint::from(constant_indices.len()),
            ))
        }
    }

    /// Truncate a constant value to the specified type.
    ///
    /// # Details
    ///
    /// Truncates a constant integer value to a smaller integer type.
    ///
    /// This function wraps the `LLVMConstTrunc` function from the LLVM core library. It generates a constant
    /// truncation instruction, which reduces the bit width of the integer value represented by `ValueRef` to the bit width
    /// of the target type specified by `to_type`. This is typically used when you need to narrow a constant integer
    /// value to a smaller type at compile time.
    ///
    /// # Parameters
    ///
    /// - `to_type`: A reference to the target type (`TypeRef`) to which the integer value should be truncated. This type must have a smaller bit width than the original integer type.
    ///
    /// # Returns
    ///
    /// Returns an instance of `ValueRef`, which encapsulates the result of the truncation. The result is a constant value
    /// determined at compile time, representing the truncated integer value.
    #[must_use]
    pub fn const_trunc(&self, to_type: &TypeRef) -> Self {
        unsafe { Self(core::LLVMConstTrunc(self.0, to_type.get_ref())) }
    }

    /// Convert a constant pointer to an integer of the specified type.
    ///
    /// # Details
    ///
    /// Converts a constant pointer value to an integer of the specified type.
    ///
    /// This function wraps the `LLVMConstPtrToInt` function from the LLVM core library. It generates a constant
    /// pointer-to-integer conversion, which interprets the pointer value represented by `ValueRef` as an integer of the
    /// type specified by `to_type`. This is commonly used in low-level programming to perform operations where
    /// a pointer needs to be treated as an integer at compile time.
    ///
    /// # Parameters
    ///
    /// - `to_type`: A reference to the target integer type (`TypeRef`) to which the pointer value should be converted. This type specifies the bit width and signedness of the resulting integer.
    ///
    /// # Returns
    ///
    /// Returns an instance of `ValueRef`, which encapsulates the result of the pointer-to-integer conversion. The result
    /// is a constant value determined at compile time, representing the integer interpretation of the pointer value.
    #[must_use]
    pub fn const_ptr_to_int(&self, to_type: &TypeRef) -> Self {
        unsafe { Self(core::LLVMConstPtrToInt(self.0, to_type.get_ref())) }
    }

    /// Convert a constant integer to a pointer of the specified type.
    ///
    /// # Details
    ///
    /// Converts a constant integer value to a pointer of the specified type.
    ///
    /// This function wraps the `LLVMConstIntToPtr` function from the LLVM core library. It generates a constant
    /// integer-to-pointer conversion, which interprets the integer value represented by `ValueRef` as a pointer of the
    /// type specified by `to_type`. This is often used in low-level programming to perform operations where
    /// an integer needs to be treated as a pointer at compile time.
    ///
    /// # Parameters
    ///
    /// - `to_type`: A reference to the target pointer type (`TypeRef`) to which the integer value should be converted. This type specifies the type of the pointer that the integer value will be interpreted as.
    ///
    /// # Returns
    ///
    /// Returns an instance of `ValueRef`, which encapsulates the result of the integer-to-pointer conversion. The result
    /// is a constant value determined at compile time, representing the pointer interpretation of the integer value.
    #[must_use]
    pub fn const_int_to_ptr(&self, to_type: &TypeRef) -> Self {
        unsafe { Self(core::LLVMConstIntToPtr(self.0, to_type.get_ref())) }
    }

    /// Perform a bitcast operation on a constant value to the specified type.
    ///
    /// # Details
    ///
    /// Performs a constant bitcast of a value to another type without changing the bit representation.
    ///
    /// This function wraps the `LLVMConstBitCast` function from the LLVM core library. It generates a constant
    /// bitcast instruction, which reinterprets the value represented by `ValueRef` as another type specified by `to_type`.
    /// The bitcast does not change the underlying bit representation of the value; it merely reinterprets it as a different type.
    /// This is typically used for converting between types of the same size, such as casting between integers and pointers or between different floating-point types.
    ///
    /// # Parameters
    ///
    /// - `to_type`: A reference to the target type (`TypeRef`) to which the value should be cast. This type must have the same bit width as the original type.
    ///
    /// # Returns
    ///
    /// Returns an instance of `ValueRef`, which encapsulates the result of the bitcast. The result is a constant value
    /// determined at compile time, representing the value reinterpreted as the target type.
    #[must_use]
    pub fn const_bit_cast(&self, to_type: &TypeRef) -> Self {
        unsafe { Self(core::LLVMConstBitCast(self.0, to_type.get_ref())) }
    }

    /// Perform an address space cast operation on a constant value to the specified type.
    ///
    /// # Details
    ///
    /// Casts a constant pointer value to a different address space.
    ///
    /// This function wraps the `LLVMConstAddrSpaceCast` function from the LLVM core library. It generates a constant
    /// address space cast, which reinterprets the pointer value represented by `ValueRef` as a pointer in a different
    /// address space specified by `to_type`. This is commonly used in systems with multiple memory address spaces
    /// where pointers may need to be converted between them at compile time.
    ///
    /// # Parameters
    ///
    /// - `to_type`: A reference to the target pointer type (`TypeRef`) that specifies the new address space. The type should have the same bit width as the original pointer type but reside in a different address space.
    ///
    /// # Returns
    ///
    /// Returns an instance of `ValueRef`, which encapsulates the result of the address space cast. The result is a constant
    /// value determined at compile time, representing the pointer value in the new address space.
    #[must_use]
    pub fn const_addr_space_cast(&self, to_type: &TypeRef) -> Self {
        unsafe { Self(core::LLVMConstAddrSpaceCast(self.0, to_type.get_ref())) }
    }

    /// Perform either a truncation or bitcast operation on a constant value to the specified type.
    ///
    /// # Details
    ///
    /// Performs a constant truncation or bitcast of a value to a specified type, depending on the target type's bit width.
    ///
    /// This function wraps the `LLVMConstTruncOrBitCast` function from the LLVM core library. It either truncates the value
    /// represented by `ValueRef` to a smaller integer type or performs a bitcast if the target type has the same bit width.
    /// The operation performed depends on the relationship between the original type and the target type's bit width.
    ///
    /// - If the target type has a smaller bit width than the original type, a truncation is performed.
    /// - If the target type has the same bit width, a bitcast is performed, reinterpreting the value as the target type without changing its bit representation.
    ///
    /// # Parameters
    ///
    /// - `to_type`: A reference to the target type (`TypeRef`) to which the value should be truncated or bitcast. The nature of the operation depends on the bit width of this type relative to the original type.
    ///
    /// # Returns
    ///
    /// Returns an instance of `ValueRef`, which encapsulates the result of the truncation or bitcast. The result is a constant
    /// value determined at compile time, representing the value either truncated to a smaller type or reinterpreted as the target type.
    #[must_use]
    pub fn const_trunc_or_bit_cast(&self, to_type: &TypeRef) -> Self {
        unsafe { Self(core::LLVMConstTruncOrBitCast(self.0, to_type.get_ref())) }
    }

    /// Perform a pointer cast operation on a constant value to the specified type.
    ///
    /// # Details
    ///
    /// Casts a constant pointer value to a different pointer type without changing the address or bit representation.
    ///
    /// This function wraps the `LLVMConstPointerCast` function from the LLVM core library. It generates a constant
    /// pointer cast, which reinterprets the pointer value represented by `ValueRef` as a different pointer type specified
    /// by `to_type`. The cast does not alter the underlying address or bit representation of the pointer; it simply changes
    /// the type of the pointer. This is typically used when you need to change the type of a pointer while preserving its
    /// address in memory.
    ///
    /// # Parameters
    ///
    /// - `to_type`: A reference to the target pointer type (`TypeRef`) to which the pointer value should be cast. The target type must be a pointer type, but it may point to a different type than the original pointer.
    ///
    /// # Returns
    ///
    /// Returns an instance of `ValueRef`, which encapsulates the result of the pointer cast. The result is a constant value
    /// determined at compile time, representing the pointer value reinterpreted as the new type.
    #[must_use]
    pub fn const_pointer_cast(&self, to_type: &TypeRef) -> Self {
        unsafe { Self(core::LLVMConstPointerCast(self.0, to_type.get_ref())) }
    }

    /// Extract an element from a vector constant at the specified index.
    ///
    /// # Details
    ///
    /// Extracts a single element from a constant vector at a specified index.
    ///
    /// This function wraps the `LLVMConstExtractElement` function from the LLVM core library. It generates a constant
    /// extract element instruction, which retrieves a specific element from the vector value represented by `ValueRef`
    /// at the position specified by `index`. This is commonly used when working with constant vectors, allowing you to
    /// extract a single element at compile time.
    ///
    /// # Parameters
    ///
    /// - `index`: A reference to a constant value that specifies the index of the element to extract. The index should be an integer value and within the bounds of the vector.
    ///
    /// # Returns
    ///
    /// Returns an instance of `ValueRef`, which encapsulates the extracted element as a constant value determined at compile time.
    #[must_use]
    pub fn const_extract_element(&self, index: &Self) -> Self {
        unsafe { Self(core::LLVMConstExtractElement(self.0, index.0)) }
    }

    /// Insert an element into a vector constant at the specified index.
    ///
    /// # Details
    ///
    /// Inserts a constant element into a constant vector at a specified index.
    ///
    /// This function wraps the `LLVMConstInsertElement` function from the LLVM core library. It generates a constant
    /// insert element instruction, which inserts the value represented by `element_value` into the vector value
    /// represented by `ValueRef` at the position specified by `index`. This is typically used to create or modify constant
    /// vectors by inserting elements at specific positions at compile time.
    ///
    /// # Parameters
    ///
    /// - `element_value`: A reference to the constant value that should be inserted into the vector. This value must be of the same type as the elements of the vector.
    /// - `index`: A reference to a constant value that specifies the index at which the element should be inserted. The index should be an integer value and within the bounds of the vector.
    ///
    /// # Returns
    ///
    /// Returns an instance of `ValueRef`, which encapsulates the resulting vector after the insertion, as a constant value determined at compile time.
    #[must_use]
    pub fn const_insert_element(&self, element_value: &Self, index: &Self) -> Self {
        unsafe {
            Self(core::LLVMConstInsertElement(
                self.0,
                element_value.0,
                index.0,
            ))
        }
    }

    /// Create a shuffle vector operation on two vector constants.
    ///
    /// # Details
    ///
    /// Creates a constant shuffling of elements from two vectors according to a specified mask.
    ///
    /// This function wraps the `LLVMConstShuffleVector` function from the LLVM core library. It generates a constant
    /// shuffle vector instruction, which produces a new vector by selecting elements from two input vectors, `vector_a`
    /// and `vector_b`, based on the indices specified by `mask`. The resulting vector is determined at compile time and
    /// is a permutation of elements from the original vectors according to the mask.
    ///
    /// # Parameters
    ///
    /// - `vector_a`: A reference to the first input vector from which elements may be selected.
    /// - `vector_b`: A reference to the second input vector from which elements may be selected.
    /// - `mask`: A reference to a constant vector that specifies the indices of elements to select from `vector_a` and `vector_b`. The mask values determine which elements from the input vectors are placed in the resulting vector.
    ///
    /// # Returns
    ///
    /// Returns an instance of `ValueRef`, which encapsulates the resulting shuffled vector as a constant value determined at compile time.
    #[must_use]
    pub fn const_shuffle_vector(vector_a: &Self, vector_b: &Self, mask: &Self) -> Self {
        unsafe { Self(core::LLVMConstShuffleVector(vector_a.0, vector_b.0, mask.0)) }
    }

    /// Obtain the address of a basic block in a function.
    ///
    /// # Details
    ///
    /// Retrieves the address of a basic block within a specified function.
    ///
    /// This function wraps the `LLVMBlockAddress` function from the LLVM core library. It generates a constant
    /// representing the address of a specific basic block within a given function. This is typically used for low-level
    /// operations such as creating labels or handling jumps within a function at compile time.
    ///
    /// # Parameters
    ///
    /// - `function`: A reference to the function (`ValueRef`) that contains the basic block whose address is being retrieved.
    /// - `basic_block`: A reference to the basic block (`BasicBlockRef`) within the function whose address is to be retrieved.
    ///
    /// # Returns
    ///
    /// Returns an instance of `ValueRef`, which encapsulates the address of the specified basic block as a constant value determined at compile time.
    #[must_use]
    pub fn block_address(function: &Self, basic_block: &BasicBlockRef) -> Self {
        unsafe { Self(core::LLVMBlockAddress(function.0, basic_block.get_ref())) }
    }
}
