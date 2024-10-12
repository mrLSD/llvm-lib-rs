//! Defines functions that allow to inspect the uses
//! of a `ValueRef`.

use crate::core::values::ValueRef;
use crate::CUint;
use llvm_sys::core;
use llvm_sys::prelude::LLVMUseRef;

/// LLVM `UseRef` wrapper
#[derive(Debug)]
pub struct UseRef(LLVMUseRef);

/// Obtain the first use of a value.
///
/// Uses are obtained in an iterator fashion. First, call this function
/// to obtain a reference to the first use. Then, call `get_next_use`
/// on that instance and all subsequently obtained instances until
/// `get_next_use` returns `NULL`.
///
/// # Details
///
/// Obtains the first use of a value in the LLVM IR.
///
/// This function wraps the `LLVMGetFirstUse` function from the LLVM core library. It retrieves the first use
/// of the value represented by `ValueRef`. In LLVM IR, a "use" represents an instance where a value is used by an
/// instruction or another value. The use can be iterated over to find all instances where this value is used.
///
/// After obtaining the first use with this function, you can call `get_next_use` on the resulting `UseRef` to
/// iterate over all uses of the value. Continue calling `get_next_use` on each subsequent `UseRef` until it returns `None`.
///
/// # Returns
///
/// Returns an `Option<UseRef>`:
/// - `Some(UseRef)` if there is a use associated with the value.
/// - `None` if there are no uses associated with the value.
#[must_use]
pub fn get_first_use(val: &ValueRef) -> Option<UseRef> {
    let first_use = unsafe { core::LLVMGetFirstUse(val.0) };
    if first_use.is_null() {
        None
    } else {
        Some(UseRef(first_use))
    }
}

/// Obtain the next use of a value.
///
/// This effectively advances the iterator. It returns `NULL` if you are on
/// the final use and no more are available.
///
/// # Details
///
/// Obtains the next use of a value in the LLVM IR.
///
/// This function wraps the `LLVMGetNextUse` function from the LLVM core library. It advances the iterator
/// of uses for a value, returning the next use after the provided `UseRef`. If there are no more uses,
/// it returns `None`. This function is used in conjunction with `get_first_use` to iterate over all uses
/// of a value in LLVM IR.
///
/// # Parameters
///
/// - `u`: A reference to the current `UseRef` from which to obtain the next use.
///
/// # Returns
///
/// Returns an `Option<UseRef>`:
/// - `Some(UseRef)` if there is a subsequent use associated with the value.
/// - `None` if there are no more uses associated with the value.
#[must_use]
pub fn get_next_use(u: &UseRef) -> Option<UseRef> {
    let next_use = unsafe { core::LLVMGetNextUse(u.0) };
    if next_use.is_null() {
        None
    } else {
        Some(UseRef(next_use))
    }
}

/// Obtain the user value for a user.
///
/// The returned value corresponds to a `UserRef` type.
///
/// # Details
///
/// Obtains the value that is using another value in LLVM IR.
///
/// This function wraps the `LLVMGetUser` function from the LLVM core library. It retrieves the user value
/// associated with the provided `UseRef`. In LLVM IR, a "user" is an entity (typically an instruction or another
/// value) that makes use of a particular value. This function returns the value that corresponds to the user.
///
/// # Parameters
///
/// - `u`: A reference to the `UseRef` for which to obtain the user value.
///
/// # Returns
///
/// Returns an instance of `ValueRef`, which represents the user value associated with the provided `UseRef`.
#[must_use]
pub fn get_user(u: &UseRef) -> ValueRef {
    unsafe { ValueRef::from(core::LLVMGetUser(u.0)) }
}

/// Obtain the value this use corresponds to.
///
/// # Details
///
/// Obtains the value that is being used by a specific use in LLVM IR.
///
/// This function wraps the `LLVMGetUsedValue` function from the LLVM core library. It retrieves the value
/// associated with a specific `UseRef`, which represents the value that is being used. This is useful for
/// understanding which value is being utilized in a particular operation or instruction within LLVM IR.
///
/// # Parameters
///
/// - `u`: A reference to the `UseRef` for which to obtain the used value.
///
/// # Returns
///
/// Returns an instance of `ValueRef`, which represents the value that is being used by the provided `UseRef`.
#[must_use]
pub fn get_used_value(u: &UseRef) -> ValueRef {
    unsafe { ValueRef::from(core::LLVMGetUsedValue(u.0)) }
}

/// Obtain an operand at a specific index in a `LLVM User` value.
///
/// # Details
///
/// Retrieves an operand at a specified index from a value in LLVM IR.
///
/// This function wraps the `LLVMGetOperand` function from the LLVM core library. It returns the operand
/// at the specified index (`index`) from the value represented by `ValueRef`. Operands are the inputs to instructions
/// or other values in LLVM IR. If the index is out of bounds or the operand cannot be retrieved, the function
/// returns `None`.
///
/// # Parameters
///
/// - `index`: The index of the operand to retrieve. This index should be within the bounds of the number of operands the value has.
///
/// # Returns
///
/// Returns an `Option<ValueRef>`:
/// - `Some(ValueRef)` containing the retrieved operand if the index is valid and the operand is found.
/// - `None` if the index is out of bounds or the operand cannot be retrieved.
#[must_use]
pub fn get_operand(val: &ValueRef, index: u32) -> Option<ValueRef> {
    let operand = unsafe { core::LLVMGetOperand(val.0, *CUint::from(index)) };
    if operand.is_null() {
        None
    } else {
        Some(ValueRef::from(operand))
    }
}

/// Obtain the use of an operand at a specific index in a `LLVM User` value.
///
/// # Details
///
/// Retrieves the use of an operand at a specified index from a value in LLVM IR.
///
/// This function wraps the `LLVMGetOperandUse` function from the LLVM core library. It returns the `UseRef`
/// associated with the operand at the specified index (`index`) from the value represented by `ValueRef`. In LLVM IR,
/// a "use" refers to an instance where an operand is used by an instruction or another value. If the index is out of
/// bounds or the operand use cannot be retrieved, the function returns `None`.
///
/// # Parameters
///
/// - `index`: The index of the operand use to retrieve. This index should be within the bounds of the number of operands the value has.
///
/// # Returns
///
/// Returns an `Option<UseRef>`:
/// - `Some(UseRef)` containing the retrieved operand use if the index is valid and the operand use is found.
/// - `None` if the index is out of bounds or the operand use cannot be retrieved.
#[must_use]
pub fn get_operand_use(val: &ValueRef, index: u32) -> Option<UseRef> {
    let operand_use = unsafe { core::LLVMGetOperandUse(val.0, *CUint::from(index)) };
    if operand_use.is_null() {
        None
    } else {
        Some(UseRef(operand_use))
    }
}

/// Set an operand at a specific index in a `LLVM User` value.
///
/// # Details
///
/// Sets the value of an operand at a specified index for a value in LLVM IR.
///
/// This function wraps the `LLVMSetOperand` function from the LLVM core library. It assigns a new value
/// (`val`) to the operand at the specified index (`index`) for the value represented by `ValueRef`. This allows
/// modification of the operands of an instruction or another value within LLVM IR.
///
/// # Parameters
///
/// - `index`: The index of the operand to set. This index should be within the bounds of the number of operands the value has.
/// - `val`: A reference to the new value (`ValueRef`) that will be assigned to the operand at the specified index.
pub fn set_operand(val1: &mut ValueRef, index: u32, val2: &ValueRef) {
    unsafe { core::LLVMSetOperand(val1.0, index, val2.0) }
}

/// Obtain the number of operands in a `LLVM User` value.
///
/// # Details
///
/// Retrieves the number of operands associated with a value in LLVM IR.
///
/// This function wraps the `LLVMGetNumOperands` function from the LLVM core library. It returns the number
/// of operands that the value represented by `ValueRef` has. This is useful for determining how many inputs or
/// arguments a particular instruction or value takes within LLVM IR.
///
/// # Returns
///
/// Returns an `i32` representing the number of operands associated with the value.
#[must_use]
pub fn get_num_operands(val: &ValueRef) -> i32 {
    unsafe { core::LLVMGetNumOperands(val.0) }
}
