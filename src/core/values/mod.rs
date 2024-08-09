use llvm_sys::LLVMValueKind;

pub mod general;

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
