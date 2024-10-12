use crate::core::module::InlineAsmDialect;
use crate::core::types::TypeRef;
use crate::{CStr, CUint, GetRef, SizeT};
use llvm_sys::core;
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMValueKind;
use std::ops::Deref;

pub mod constants;
pub mod general;
pub mod uses;

/// Represents the different kinds of values in LLVM IR.
///
/// The `ValueKind` enum categorizes the various types of values that can exist within LLVM IR. Each variant
/// of this enum corresponds to a specific kind of value or entity in the LLVM IR, such as a function, global variable,
/// instruction, or constant. This enum is useful for identifying the type of a value when working with LLVM IR structures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValueKind {
    /// Represents a function argument. Each argument passed to a function is an instance of this kind.
    Argument,
    /// Represents a basic block within a function. Basic blocks are the building blocks of a function's control flow.
    BasicBlock,
    /// Represents a memory use in the `MemorySSA`. This kind tracks the use of memory locations within SSA form.
    MemoryUse,
    /// Represents a memory definition in the `MemorySSA`. This kind tracks the definition of memory locations within SSA form.
    MemoryDef,
    /// Represents a memory PHI node in the `MemorySSA`. This kind handles merging memory states from different control flow paths.
    MemoryPhi,
    /// Represents a function. Functions are the primary callable entities in LLVM IR.
    Function,
    /// Represents a global alias. Global aliases are alternative names for global variables or functions.
    GlobalAlias,
    /// Represents an indirect function. This is a function pointer that can be resolved at runtime to a specific implementation.
    GlobalIFunc,
    /// Represents a global variable. Global variables are variables that are globally accessible across the entire module.
    GlobalVariable,
    /// Represents a block address. This is used to refer to the address of a basic block within a function.
    BlockAddress,
    /// Represents a constant expression. Constant expressions are constant values that are computed at compile time from other constants.
    ConstantExpr,
    /// Represents a constant array. Constant arrays are arrays whose elements are all constant values.
    ConstantArray,
    /// Represents a constant struct. Constant structs are structures whose fields are all constant values.
    ConstantStruct,
    /// Represents a constant vector. Constant vectors are vectors whose elements are all constant values.
    ConstantVector,
    /// Represents an undefined value. Undefined values are placeholders that can take any value of their type during execution.
    Undef,
    /// Represents a constant aggregate with all elements set to zero. This includes arrays, structs, and vectors with all elements set to zero.
    ConstantAggregateZero,
    /// Represents a constant data array. These are arrays of simple data types like integers or floating-point numbers, stored as constants.
    ConstantDataArray,
    /// Represents a constant data vector. These are vectors of simple data types like integers or floating-point numbers, stored as constants.
    ConstantDataVector,
    /// Represents a constant integer. Constant integers are fixed integer values known at compile time.
    ConstantInt,
    /// Represents a constant floating-point value. Constant floating-point values are fixed floating-point numbers known at compile time.
    ConstantFP,
    /// Represents a constant null pointer. This is a pointer that is explicitly set to null.
    ConstantPointerNull,
    /// Represents a constant token with no value. Used in certain intrinsic functions that deal with tokens.
    ConstantTokenNone,
    /// Represents metadata used as a value. Metadata can be used to store extra information for optimizations, debugging, or analysis.
    MetadataAsValue,
    /// Represents inline assembly code. Inline assembly allows embedding low-level assembly code within LLVM IR.
    InlineAsm,
    /// Represents an instruction. Instructions are the individual operations that make up the body of functions.
    Instruction,
    /// Represents a poison value. Poison values result from operations with undefined behavior and can propagate to cause further undefined behavior.
    Poison,
    /// Represents a target-specific constant value that has no direct representation in the source code.
    ConstantTargetNone,
    /// Represents a constant pointer with pointer authentication. This is a pointer with additional authentication information.
    ConstantPtrAuthValueKind,
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
            LLVMValueKind::LLVMConstantPtrAuthValueKind => Self::ConstantPtrAuthValueKind,
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

/// That implementations related to LLVM Modules `MemoryDef`.
impl ValueRef {
    /// Get the template string used for an inline assembly snippet.
    ///
    /// # Details
    ///
    /// Retrieves the assembly code string from the inline assembly block in LLVM IR.
    ///
    /// This function wraps the `LLVMGetInlineAsmAsmString` function from the LLVM core library. It returns the
    /// assembly code string used by the inline assembly block associated with `self`. This string contains the
    /// actual assembly instructions that will be executed as part of the inline assembly.
    ///
    /// If the assembly string cannot be retrieved, the function returns `None`.
    ///
    /// # Returns
    ///
    /// Returns an `Option<String>`:
    /// - `Some(String)` containing the assembly code string if successful.
    /// - `None` if the assembly string cannot be retrieved.
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
    ///
    /// # Details
    ///
    /// Retrieves the constraint string associated with the inline assembly block in LLVM IR.
    ///
    /// This function wraps the `LLVMGetInlineAsmConstraintString` function from the LLVM core library. It returns the
    /// constraint string used by the inline assembly block associated with `self`. The constraint string specifies the
    /// constraints on the operands used in the inline assembly, such as register classes or memory addressing modes.
    ///
    /// If the constraint string cannot be retrieved, the function returns `None`.
    ///
    /// # Returns
    ///
    /// Returns an `Option<String>`:
    /// - `Some(String)` containing the constraint string if successful.
    /// - `None` if the constraint string cannot be retrieved.
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
    ///
    /// # Details
    ///
    /// Retrieves the dialect of the inline assembly block in LLVM IR.
    ///
    /// This function wraps the `LLVMGetInlineAsmDialect` function from the LLVM core library. It returns the
    /// `InlineAsmDialect` representing the dialect used by the inline assembly block associated with `self`.
    /// The dialect determines the syntax and conventions used in the inline assembly, which may vary between
    /// different assemblers (e.g., AT&T vs. Intel syntax).
    ///
    /// # Returns
    ///
    /// Returns an `InlineAsmDialect` that represents the dialect of the inline assembly block.
    #[must_use]
    pub fn get_inline_asm_dialect(&self) -> InlineAsmDialect {
        let inline_asm_dialect = unsafe { core::LLVMGetInlineAsmDialect(self.0) };
        inline_asm_dialect.into()
    }

    /// Get the function type of the inline assembly snippet.
    ///
    /// This is the same type that was passed into `LLVMGetInlineAsm` originally.
    ///
    /// # Returns
    ///
    /// Retrieves the function type of the inline assembly block in LLVM IR.
    ///
    /// This function wraps the `LLVMGetInlineAsmFunctionType` function from the LLVM core library. It returns the
    /// `TypeRef` representing the function type of the inline assembly block associated with `self`. The function type
    /// defines the signature of the inline assembly, including the types of its arguments and return value.
    ///
    /// # Returns
    ///
    /// Returns a `TypeRef` that represents the function type of the inline assembly block.
    #[must_use]
    pub fn get_inline_asm_function_type(&self) -> TypeRef {
        TypeRef::from(unsafe { core::LLVMGetInlineAsmFunctionType(self.0) })
    }

    /// Get if the inline asm snippet has side effects
    ///
    /// # Details
    ///
    /// Checks whether an inline assembly block has side effects in LLVM IR.
    ///
    /// This function wraps the `LLVMGetInlineAsmHasSideEffects` function from the LLVM core library. It determines
    /// whether the inline assembly represented by `self` has side effects, meaning that it may alter state or interact
    /// with external systems in ways that are not visible within the LLVM IR. This flag is important for optimizations,
    /// as it indicates that the inline assembly cannot be removed or reordered without potentially affecting program behavior.
    ///
    /// # Returns
    ///
    /// Returns `true` if the inline assembly block has side effects, otherwise returns `false`.
    #[must_use]
    pub fn get_inline_asm_has_side_effects(&self) -> bool {
        unsafe { core::LLVMGetInlineAsmHasSideEffects(self.0) != 0 }
    }

    /// Get if the inline asm snippet needs an aligned stack
    ///
    /// # Details
    ///
    /// Checks whether an inline assembly block requires an aligned stack in LLVM IR.
    ///
    /// This function wraps the `LLVMGetInlineAsmNeedsAlignedStack` function from the LLVM core library. It determines
    /// whether the inline assembly represented by `self` requires the stack to be aligned. Proper stack alignment
    /// may be necessary for certain instructions or calling conventions, and this flag indicates whether such alignment
    /// is needed.
    ///
    /// # Returns
    ///
    /// Returns `true` if the inline assembly block requires an aligned stack, otherwise returns `false`.
    #[must_use]
    pub fn get_inline_asm_needs_aligned_stack(&self) -> bool {
        unsafe { core::LLVMGetInlineAsmNeedsAlignedStack(self.0) != 0 }
    }

    /// Get if the inline asm snippet may unwind the stack
    ///
    /// # Details
    ///
    /// Checks whether an inline assembly block can unwind in LLVM IR.
    ///
    /// This function wraps the `LLVMGetInlineAsmCanUnwind` function from the LLVM core library. It determines whether
    /// the inline assembly represented by `self` is capable of unwinding, which can affect how exceptions and
    /// other control flows are handled during execution.
    ///
    /// # Returns
    ///
    /// Returns `true` if the inline assembly block can unwind, otherwise returns `false`.
    #[must_use]
    pub fn get_inline_asm_can_unwind(&self) -> bool {
        unsafe { core::LLVMGetInlineAsmCanUnwind(self.0) != 0 }
    }

    /// Return the directory of the debug location for this value, which must be
    /// an LLVM `Instruction`, `GlobalVariable`, or `Function`.
    ///
    /// # Details
    ///
    /// Retrieves the directory from the debug location associated with this value in LLVM IR.
    ///
    /// This function wraps the `LLVMGetDebugLocDirectory` function from the LLVM core library. It returns the
    /// directory of the source code location associated with the debug information for the value represented by `self`.
    /// If the directory cannot be retrieved, the function returns `None`.
    ///
    /// # Returns
    ///
    /// Returns an `Option<String>`:
    /// - `Some(String)` containing the directory associated with the value's debug location if successful.
    /// - `None` if the directory cannot be retrieved.
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
    ///
    /// # Details
    ///
    /// Retrieves the filename from the debug location associated with this value in LLVM IR.
    ///
    /// This function wraps the `LLVMGetDebugLocFilename` function from the LLVM core library. It returns the
    /// filename of the source code location associated with the debug information for the value represented by `self`.
    /// If the filename cannot be retrieved, the function returns `None`.
    ///
    /// # Returns
    ///
    /// Returns an `Option<String>`:
    /// - `Some(String)` containing the filename associated with the value's debug location if successful.
    /// - `None` if the filename cannot be retrieved.
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
    ///
    /// # Details
    ///
    /// Retrieves the line number from the debug location associated with this value in LLVM IR.
    ///
    /// This function wraps the `LLVMGetDebugLocLine` function from the LLVM core library. It returns the
    /// line number of the source code location associated with the debug information for the value represented by `self`.
    /// This is useful for debugging and for tools that need to report precise source locations.
    ///
    /// # Returns
    ///
    /// Returns a `u32` representing the line number in the source code associated with this value's debug location.
    #[must_use]
    pub fn get_debug_loc_line(&self) -> u32 {
        unsafe { core::LLVMGetDebugLocLine(self.0) }
    }

    /// Return the column number of the debug location for this value, which must be
    /// an LLVM `Instruction`.
    ///
    /// # Details
    ///
    /// Retrieves the column number from the debug location associated with this value in LLVM IR.
    ///
    /// This function wraps the `LLVMGetDebugLocColumn` function from the LLVM core library. It returns the
    /// column number of the source code location associated with the debug information for the value represented by `self`.
    /// This is useful for debugging and for tools that need to report precise source locations.
    ///
    /// # Returns
    ///
    /// Returns a `u32` representing the column number in the source code associated with this value's debug location.
    #[must_use]
    pub fn get_debug_loc_column(&self) -> u32 {
        unsafe { core::LLVMGetDebugLocColumn(self.0) }
    }

    /// Advance a `Function` iterator to the next Function.
    ///
    /// Returns `None` if the iterator was already at the end and there are no more functions.
    ///
    /// # Details
    ///
    /// Retrieves the next function in the module relative to this function, if it exists.
    ///
    /// This function wraps the `LLVMGetNextFunction` function from the LLVM core library. It returns the
    /// next function in the module relative to the function represented by `self`. If there is no next
    /// function, the function returns `None`. This is useful for iterating over functions within a module in LLVM IR.
    ///
    /// # Returns
    ///
    /// Returns an `Option<ValueRef>`:
    /// - `Some(ValueRef)` containing the next function if it exists.
    /// - `None` if there is no next function in the module.
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
    ///
    /// # Details
    ///
    /// Retrieves the previous function in the module relative to this function, if it exists.
    ///
    /// This function wraps the `LLVMGetPreviousFunction` function from the LLVM core library. It returns the
    /// previous function in the module relative to the function represented by `self`. If there is no previous
    /// function, the function returns `None`. This is useful for iterating over functions within a module in LLVM IR.
    ///
    /// # Returns
    ///
    /// Returns an `Option<ValueRef>`:
    /// - `Some(ValueRef)` containing the previous function if it exists.
    /// - `None` if there is no previous function in the module.
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
