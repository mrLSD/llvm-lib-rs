use crate::core::module::InlineAsmDialect;
use crate::core::types::TypeRef;
use crate::{CStr, CUint, GetRef, SizeT};
use llvm_sys::core;
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMValueKind;
use std::ops::Deref;
use std::rc::Rc;

pub mod constants;
pub mod general;
pub mod uses;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValueKind {
    Argument,
    BasicBlock,
    MemoryUse,
    MemoryDef,
    MemoryPhi,
    Function,
    GlobalAlias,
    GlobalIFunc,
    GlobalVariable,
    BlockAddress,
    ConstantExpr,
    ConstantArray,
    ConstantStruct,
    ConstantVector,
    Undef,
    ConstantAggregateZero,
    ConstantDataArray,
    ConstantDataVector,
    ConstantInt,
    ConstantFP,
    ConstantPointerNull,
    ConstantTokenNone,
    MetadataAsValue,
    InlineAsm,
    Instruction,
    Poison,
    ConstantTargetNone,
}

impl From<LLVMValueKind> for ValueKind {
    fn from(value: LLVMValueKind) -> Self {
        match value {
            LLVMValueKind::LLVMArgumentValueKind => Self::Argument,
            LLVMValueKind::LLVMBasicBlockValueKind => Self::BasicBlock,
            LLVMValueKind::LLVMMemoryUseValueKind => Self::MemoryUse,
            LLVMValueKind::LLVMMemoryDefValueKind => Self::MemoryDef,
            LLVMValueKind::LLVMMemoryPhiValueKind => Self::MemoryPhi,
            LLVMValueKind::LLVMFunctionValueKind => Self::Function,
            LLVMValueKind::LLVMGlobalAliasValueKind => Self::GlobalAlias,
            LLVMValueKind::LLVMGlobalIFuncValueKind => Self::GlobalIFunc,
            LLVMValueKind::LLVMGlobalVariableValueKind => Self::GlobalVariable,
            LLVMValueKind::LLVMBlockAddressValueKind => Self::BlockAddress,
            LLVMValueKind::LLVMConstantExprValueKind => Self::ConstantExpr,
            LLVMValueKind::LLVMConstantArrayValueKind => Self::ConstantArray,
            LLVMValueKind::LLVMConstantStructValueKind => Self::ConstantStruct,
            LLVMValueKind::LLVMConstantVectorValueKind => Self::ConstantVector,
            LLVMValueKind::LLVMUndefValueValueKind => Self::Undef,
            LLVMValueKind::LLVMConstantAggregateZeroValueKind => Self::ConstantAggregateZero,
            LLVMValueKind::LLVMConstantDataArrayValueKind => Self::ConstantDataArray,
            LLVMValueKind::LLVMConstantDataVectorValueKind => Self::ConstantDataVector,
            LLVMValueKind::LLVMConstantIntValueKind => Self::ConstantInt,
            LLVMValueKind::LLVMConstantFPValueKind => Self::ConstantFP,
            LLVMValueKind::LLVMConstantPointerNullValueKind => Self::ConstantPointerNull,
            LLVMValueKind::LLVMConstantTokenNoneValueKind => Self::ConstantTokenNone,
            LLVMValueKind::LLVMMetadataAsValueValueKind => Self::MetadataAsValue,
            LLVMValueKind::LLVMInlineAsmValueKind => Self::InlineAsm,
            LLVMValueKind::LLVMInstructionValueKind => Self::Instruction,
            LLVMValueKind::LLVMPoisonValueKind => Self::Poison,
            LLVMValueKind::LLVMConstantTargetNoneValueKind => Self::ConstantTargetNone,
        }
    }
}

/// LLVM Value wrapper
#[derive(Debug)]
pub struct ValueRef(LLVMValueRef);

impl Deref for ValueRef {
    type Target = LLVMValueRef;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl GetRef for ValueRef {
    type RawRef = LLVMValueRef;
    fn get_ref(&self) -> Self::RawRef {
        self.0
    }
}

impl From<LLVMValueRef> for ValueRef {
    fn from(value_ref: LLVMValueRef) -> Self {
        Self(value_ref)
    }
}

impl ValueRef {
    /// Get function parameter by index
    // TODO: move to Function Values
    #[must_use]
    pub fn get_func_param(func_value: &Rc<Self>, index: usize) -> Self {
        unsafe { Self(core::LLVMGetParam(***func_value, *CUint::from(index))) }
    }

    /// Get the template string used for an inline assembly snippet.
    #[must_use]
    pub fn get_inline_asm_asm_string(&self) -> Option<String> {
        unsafe {
            let mut length = SizeT::from(0_usize);
            let c_str = core::LLVMGetInlineAsmAsmString(self.0, &mut *length);
            if c_str.is_null() {
                return None;
            }
            Some(CStr::new(c_str).to_string())
        }
    }

    /// Get the raw constraint string for an inline assembly snippet.
    #[must_use]
    pub fn get_inline_asm_constraint_string(&self) -> Option<String> {
        unsafe {
            let mut length = SizeT::from(0_usize);
            let c_str = core::LLVMGetInlineAsmConstraintString(self.0, &mut *length);
            if c_str.is_null() {
                return None;
            }
            Some(CStr::new(c_str).to_string())
        }
    }

    /// Get the dialect used by the inline asm snippet.
    #[must_use]
    pub fn get_inline_asm_dialect(&self) -> InlineAsmDialect {
        let inline_asm_dialect = unsafe { core::LLVMGetInlineAsmDialect(self.0) };
        inline_asm_dialect.into()
    }

    /// Get the function type of the inline assembly snippet.
    ///
    /// This is the same type that was passed into `LLVMGetInlineAsm` originally.
    #[must_use]
    pub fn get_inline_asm_function_type(&self) -> TypeRef {
        TypeRef::from(unsafe { core::LLVMGetInlineAsmFunctionType(self.0) })
    }

    /// Get if the inline asm snippet has side effects
    #[must_use]
    pub fn get_inline_asm_has_side_effects(&self) -> bool {
        unsafe { core::LLVMGetInlineAsmHasSideEffects(self.0) != 0 }
    }

    /// Get if the inline asm snippet needs an aligned stack
    #[must_use]
    pub fn get_inline_asm_needs_aligned_stack(&self) -> bool {
        unsafe { core::LLVMGetInlineAsmNeedsAlignedStack(self.0) != 0 }
    }

    /// Get if the inline asm snippet may unwind the stack
    #[must_use]
    pub fn get_inline_asm_can_unwind(&self) -> bool {
        unsafe { core::LLVMGetInlineAsmCanUnwind(self.0) != 0 }
    }

    /// Return the directory of the debug location for this value, which must be
    /// an LLVM `Instruction`, `GlobalVariable`, or `Function`.
    #[must_use]
    pub fn get_debug_loc_directory(&self) -> Option<String> {
        unsafe {
            let mut length = CUint::from(0_usize);
            let c_str = core::LLVMGetDebugLocDirectory(self.0, &mut *length);
            if c_str.is_null() {
                return None;
            }
            Some(CStr::new(c_str).to_string())
        }
    }

    /// Return the filename of the debug location for this value, which must be
    /// an LLVM `Instruction`, `lGlobalVariable`, or `Function`.
    #[must_use]
    pub fn get_debug_loc_filename(&self) -> Option<String> {
        unsafe {
            let mut length = CUint::from(0_usize);
            let c_str = core::LLVMGetDebugLocFilename(self.0, &mut *length);
            if c_str.is_null() {
                return None;
            }
            Some(CStr::new(c_str).to_string())
        }
    }

    /// Return the line number of the debug location for this value, which must be
    /// an LLVM `Instruction`, `GlobalVariable`, or `Function`.
    #[must_use]
    pub fn get_debug_loc_line(&self) -> u32 {
        unsafe { core::LLVMGetDebugLocLine(self.0) }
    }

    /// Return the column number of the debug location for this value, which must be
    /// an LLVM `Instruction`.
    #[must_use]
    pub fn get_debug_loc_column(&self) -> u32 {
        unsafe { core::LLVMGetDebugLocColumn(self.0) }
    }

    /// Advance a `Function` iterator to the next Function.
    ///
    /// Returns `None` if the iterator was already at the end and there are no more functions.
    #[must_use]
    pub fn get_next_function(&self) -> Option<Self> {
        unsafe {
            let next_func = core::LLVMGetNextFunction(self.0);
            if next_func.is_null() {
                None
            } else {
                Some(Self(next_func))
            }
        }
    }

    /// Decrement a `Function` iterator to the previous Function.
    ///
    /// Returns `None` if the iterator was already at the beginning and there are no previous functions.
    #[must_use]
    pub fn get_previous_function(&self) -> Option<Self> {
        unsafe {
            let prev_func = core::LLVMGetPreviousFunction(self.0);
            if prev_func.is_null() {
                None
            } else {
                Some(Self(prev_func))
            }
        }
    }
}
