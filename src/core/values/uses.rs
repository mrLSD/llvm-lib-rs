//! Defines functions that allow to inspect the uses
//! of a `ValueRef`.

use crate::core::values::ValueRef;
use crate::CUint;
use llvm_sys::core;
use llvm_sys::prelude::LLVMUseRef;

/// LLVM `UseRef` wrapper
#[derive(Debug)]
pub struct UseRef(LLVMUseRef);

impl ValueRef {
    /// Obtain the first use of a value.
    ///
    /// Uses are obtained in an iterator fashion. First, call this function
    /// to obtain a reference to the first use. Then, call `get_next_use`
    /// on that instance and all subsequently obtained instances until
    /// `get_next_use` returns `NULL`.
    #[must_use]
    pub fn get_first_use(&self) -> Option<UseRef> {
        let first_use = unsafe { core::LLVMGetFirstUse(self.0) };
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
    #[must_use]
    pub fn get_next_use(&self, u: &UseRef) -> Option<UseRef> {
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
    #[must_use]
    pub fn get_user(&self, u: &UseRef) -> Self {
        unsafe { Self::from(core::LLVMGetUser(u.0)) }
    }

    /// Obtain the value this use corresponds to.
    #[must_use]
    pub fn get_used_value(&self, u: &UseRef) -> Self {
        unsafe { Self::from(core::LLVMGetUsedValue(u.0)) }
    }

    /// Obtain an operand at a specific index in a `LLVM User` value.
    #[must_use]
    pub fn get_operand(&self, index: u32) -> Option<Self> {
        let operand = unsafe { core::LLVMGetOperand(self.0, *CUint::from(index)) };
        if operand.is_null() {
            None
        } else {
            Some(Self::from(operand))
        }
    }

    /// Obtain the use of an operand at a specific index in a `LLVM User` value.
    #[must_use]
    pub fn get_operand_use(&self, index: u32) -> Option<UseRef> {
        let operand_use = unsafe { core::LLVMGetOperandUse(self.0, *CUint::from(index)) };
        if operand_use.is_null() {
            None
        } else {
            Some(UseRef(operand_use))
        }
    }

    /// Set an operand at a specific index in a `LLVM User` value.
    pub fn set_operand(&mut self, index: u32, val: &Self) {
        unsafe { core::LLVMSetOperand(self.0, index, val.0) }
    }

    /// Obtain the number of operands in a `LLVM User` value.
    #[must_use]
    pub fn get_num_operands(&self) -> i32 {
        unsafe { core::LLVMGetNumOperands(self.0) }
    }
}
