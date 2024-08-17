use crate::CUint;
use llvm_sys::{core, LLVMIntPredicate, LLVMOpcode, LLVMRealPredicate};
use std::fmt::Display;
use std::ops::Deref;

pub mod context;
pub mod module;
pub mod types;
pub mod values;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressSpace(CUint);

impl From<u32> for AddressSpace {
    fn from(value: u32) -> Self {
        Self(CUint::from(value))
    }
}

impl Deref for AddressSpace {
    type Target = CUint;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AddressSpace {
    #[must_use]
    pub const fn new(value: CUint) -> Self {
        Self(value)
    }
}

/// Dispose LLVM message
/// ## Safety
/// Common function to dispose allocated message
pub unsafe fn dispose_message(message: *mut libc::c_char) {
    unsafe { core::LLVMDisposeMessage(message) }
}

/// LLVM version representation
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Version {
    /// Init and return current LLVM version
    #[must_use]
    pub fn new() -> Self {
        let mut major = CUint::from(0_u32);
        let mut minor = CUint::from(0_u32);
        let mut patch = CUint::from(0_u32);
        unsafe {
            core::LLVMGetVersion(&mut *major, &mut *minor, &mut *patch);
        }
        Self {
            major: major.into(),
            minor: minor.into(),
            patch: patch.into(),
        }
    }

    /// Return LLVM version data: (major, minor, patch)
    #[must_use]
    pub const fn get(&self) -> (u32, u32, u32) {
        (self.minor, self.minor, self.patch)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Represents the various opcodes in LLVM IR.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opcode {
    /// Return instruction.
    Ret,
    /// Branch instruction.
    Br,
    /// Switch instruction.
    Switch,
    /// Indirect branch instruction.
    IndirectBr,
    /// Invoke instruction.
    Invoke,
    /// Unreachable instruction.
    Unreachable,
    /// `CallBr` instruction.
    CallBr,
    /// Floating-point negation instruction.
    FNeg,
    /// Integer addition instruction.
    Add,
    /// Floating-point addition instruction.
    FAdd,
    /// Integer subtraction instruction.
    Sub,
    /// Floating-point subtraction instruction.
    FSub,
    /// Integer multiplication instruction
    Mul,
    /// Floating-point multiplication instruction.
    FMul,
    /// Unsigned integer division instruction.
    UDiv,
    /// Signed integer division instruction.
    SDiv,
    /// Floating-point division instruction.
    FDiv,
    /// Unsigned integer remainder instruction.
    URem,
    /// Signed integer remainder instruction.
    SRem,
    /// Floating-point remainder instruction.
    FRem,
    /// Logical shift left instruction.
    Shl,
    /// Logical shift right instruction.
    LShr,
    /// Arithmetic shift right instruction.
    AShr,
    /// Bitwise AND instruction.
    And,
    /// Bitwise OR instruction.
    Or,
    /// Bitwise XOR instruction.
    Xor,
    /// Alloca instruction.
    Alloca,
    /// Load instruction.
    Load,
    /// Store instruction.
    Store,
    /// `GetElementPtr` instruction.
    GetElementPtr,
    /// Truncate instruction.
    Trunc,
    /// Zero extend instruction.
    ZExt,
    /// Sign extend instruction.
    SExt,
    /// Floating-point to unsigned integer instruction.
    FPToUI,
    /// Floating-point to signed integer instruction.
    FPToSI,
    /// Unsigned integer to floating-point instruction.
    UIToFP,
    /// Signed integer to floating-point instruction.
    SIToFP,
    /// Floating-point truncate instruction.
    FPTrunc,
    /// Floating-point extend instruction
    FPExt,
    /// Pointer to integer instruction
    PtrToInt,
    /// Integer to pointer instruction
    IntToPtr,
    /// Bit-cast instruction
    BitCast,
    /// Address space cast instruction
    AddrSpaceCast,
    /// Integer comparison instruction
    ICmp,
    /// Floating-point comparison instruction
    FCmp,
    /// PHI node instruction
    PHI,
    /// Call instruction
    Call,
    /// Select instruction
    Select,

    /// User-defined operation 1
    UserOp1,
    /// User-defined operation 2
    UserOp2,
    /// Variable argument instruction
    VAArg,
    /// Extract element from vector instruction
    ExtractElement,
    /// Insert element into vector instruction
    InsertElement,
    /// Shuffle vector instruction
    ShuffleVector,
    /// Extract value from aggregate instruction
    ExtractValue,
    /// Insert value into aggregate instruction
    InsertValue,
    /// Freeze instruction
    Freeze,
    /// Fence instruction
    Fence,
    /// Atomic compare and exchange instruction
    AtomicCmpXchg,
    /// Atomic read-modify-write instruction
    AtomicRMW,
    /// Resume instruction
    Resume,
    /// Landing pad instruction
    LandingPad,
    /// Cleanup return instruction.
    CleanupRet,
    /// Catch return instruction
    CatchRet,
    /// Catch pad instruction
    CatchPad,
    /// Cleanup pad instruction
    CleanupPad,
    /// Catch switch instruction
    CatchSwitch,
}

impl From<LLVMOpcode> for Opcode {
    fn from(opcode: LLVMOpcode) -> Self {
        match opcode {
            LLVMOpcode::LLVMRet => Self::Ret,
            LLVMOpcode::LLVMBr => Self::Br,
            LLVMOpcode::LLVMSwitch => Self::Switch,
            LLVMOpcode::LLVMIndirectBr => Self::IndirectBr,
            LLVMOpcode::LLVMInvoke => Self::Invoke,
            LLVMOpcode::LLVMUnreachable => Self::Unreachable,
            LLVMOpcode::LLVMCallBr => Self::CallBr,
            LLVMOpcode::LLVMFNeg => Self::FNeg,
            LLVMOpcode::LLVMAdd => Self::Add,
            LLVMOpcode::LLVMFAdd => Self::FAdd,
            LLVMOpcode::LLVMSub => Self::Sub,
            LLVMOpcode::LLVMFSub => Self::FSub,
            LLVMOpcode::LLVMMul => Self::Mul,
            LLVMOpcode::LLVMFMul => Self::FMul,
            LLVMOpcode::LLVMUDiv => Self::UDiv,
            LLVMOpcode::LLVMSDiv => Self::SDiv,
            LLVMOpcode::LLVMFDiv => Self::FDiv,
            LLVMOpcode::LLVMURem => Self::URem,
            LLVMOpcode::LLVMSRem => Self::SRem,
            LLVMOpcode::LLVMFRem => Self::FRem,
            LLVMOpcode::LLVMShl => Self::Shl,
            LLVMOpcode::LLVMLShr => Self::LShr,
            LLVMOpcode::LLVMAShr => Self::AShr,
            LLVMOpcode::LLVMAnd => Self::And,
            LLVMOpcode::LLVMOr => Self::Or,
            LLVMOpcode::LLVMXor => Self::Xor,
            LLVMOpcode::LLVMAlloca => Self::Alloca,
            LLVMOpcode::LLVMLoad => Self::Load,
            LLVMOpcode::LLVMStore => Self::Store,
            LLVMOpcode::LLVMGetElementPtr => Self::GetElementPtr,
            LLVMOpcode::LLVMTrunc => Self::Trunc,
            LLVMOpcode::LLVMZExt => Self::ZExt,
            LLVMOpcode::LLVMSExt => Self::SExt,
            LLVMOpcode::LLVMFPToUI => Self::FPToUI,
            LLVMOpcode::LLVMFPToSI => Self::FPToSI,
            LLVMOpcode::LLVMUIToFP => Self::UIToFP,
            LLVMOpcode::LLVMSIToFP => Self::SIToFP,
            LLVMOpcode::LLVMFPTrunc => Self::FPTrunc,
            LLVMOpcode::LLVMFPExt => Self::FPExt,
            LLVMOpcode::LLVMPtrToInt => Self::PtrToInt,
            LLVMOpcode::LLVMIntToPtr => Self::IntToPtr,
            LLVMOpcode::LLVMBitCast => Self::BitCast,
            LLVMOpcode::LLVMAddrSpaceCast => Self::AddrSpaceCast,
            LLVMOpcode::LLVMICmp => Self::ICmp,
            LLVMOpcode::LLVMFCmp => Self::FCmp,
            LLVMOpcode::LLVMPHI => Self::PHI,
            LLVMOpcode::LLVMCall => Self::Call,
            LLVMOpcode::LLVMSelect => Self::Select,
            LLVMOpcode::LLVMUserOp1 => Self::UserOp1,
            LLVMOpcode::LLVMUserOp2 => Self::UserOp2,
            LLVMOpcode::LLVMVAArg => Self::VAArg,
            LLVMOpcode::LLVMExtractElement => Self::ExtractElement,
            LLVMOpcode::LLVMInsertElement => Self::InsertElement,
            LLVMOpcode::LLVMShuffleVector => Self::ShuffleVector,
            LLVMOpcode::LLVMExtractValue => Self::ExtractValue,
            LLVMOpcode::LLVMInsertValue => Self::InsertValue,
            LLVMOpcode::LLVMFreeze => Self::Freeze,
            LLVMOpcode::LLVMFence => Self::Fence,
            LLVMOpcode::LLVMAtomicCmpXchg => Self::AtomicCmpXchg,
            LLVMOpcode::LLVMAtomicRMW => Self::AtomicRMW,
            LLVMOpcode::LLVMResume => Self::Resume,
            LLVMOpcode::LLVMLandingPad => Self::LandingPad,
            LLVMOpcode::LLVMCleanupRet => Self::CleanupRet,
            LLVMOpcode::LLVMCatchRet => Self::CatchRet,
            LLVMOpcode::LLVMCatchPad => Self::CatchPad,
            LLVMOpcode::LLVMCleanupPad => Self::CleanupPad,
            LLVMOpcode::LLVMCatchSwitch => Self::CatchSwitch,
        }
    }
}

impl From<Opcode> for LLVMOpcode {
    fn from(opcode: Opcode) -> Self {
        match opcode {
            Opcode::Ret => Self::LLVMRet,
            Opcode::Br => Self::LLVMBr,
            Opcode::Switch => Self::LLVMSwitch,
            Opcode::IndirectBr => Self::LLVMIndirectBr,
            Opcode::Invoke => Self::LLVMInvoke,
            Opcode::Unreachable => Self::LLVMUnreachable,
            Opcode::CallBr => Self::LLVMCallBr,
            Opcode::FNeg => Self::LLVMFNeg,
            Opcode::Add => Self::LLVMAdd,
            Opcode::FAdd => Self::LLVMFAdd,
            Opcode::Sub => Self::LLVMSub,
            Opcode::FSub => Self::LLVMFSub,
            Opcode::Mul => Self::LLVMMul,
            Opcode::FMul => Self::LLVMFMul,
            Opcode::UDiv => Self::LLVMUDiv,
            Opcode::SDiv => Self::LLVMSDiv,
            Opcode::FDiv => Self::LLVMFDiv,
            Opcode::URem => Self::LLVMURem,
            Opcode::SRem => Self::LLVMSRem,
            Opcode::FRem => Self::LLVMFRem,
            Opcode::Shl => Self::LLVMShl,
            Opcode::LShr => Self::LLVMLShr,
            Opcode::AShr => Self::LLVMAShr,
            Opcode::And => Self::LLVMAnd,
            Opcode::Or => Self::LLVMOr,
            Opcode::Xor => Self::LLVMXor,
            Opcode::Alloca => Self::LLVMAlloca,
            Opcode::Load => Self::LLVMLoad,
            Opcode::Store => Self::LLVMStore,
            Opcode::GetElementPtr => Self::LLVMGetElementPtr,
            Opcode::Trunc => Self::LLVMTrunc,
            Opcode::ZExt => Self::LLVMZExt,
            Opcode::SExt => Self::LLVMSExt,
            Opcode::FPToUI => Self::LLVMFPToUI,
            Opcode::FPToSI => Self::LLVMFPToSI,
            Opcode::UIToFP => Self::LLVMUIToFP,
            Opcode::SIToFP => Self::LLVMSIToFP,
            Opcode::FPTrunc => Self::LLVMFPTrunc,
            Opcode::FPExt => Self::LLVMFPExt,
            Opcode::PtrToInt => Self::LLVMPtrToInt,
            Opcode::IntToPtr => Self::LLVMIntToPtr,
            Opcode::BitCast => Self::LLVMBitCast,
            Opcode::AddrSpaceCast => Self::LLVMAddrSpaceCast,
            Opcode::ICmp => Self::LLVMICmp,
            Opcode::FCmp => Self::LLVMFCmp,
            Opcode::PHI => Self::LLVMPHI,
            Opcode::Call => Self::LLVMCall,
            Opcode::Select => Self::LLVMSelect,
            Opcode::UserOp1 => Self::LLVMUserOp1,
            Opcode::UserOp2 => Self::LLVMUserOp2,
            Opcode::VAArg => Self::LLVMVAArg,
            Opcode::ExtractElement => Self::LLVMExtractElement,
            Opcode::InsertElement => Self::LLVMInsertElement,
            Opcode::ShuffleVector => Self::LLVMShuffleVector,
            Opcode::ExtractValue => Self::LLVMExtractValue,
            Opcode::InsertValue => Self::LLVMInsertValue,
            Opcode::Freeze => Self::LLVMFreeze,
            Opcode::Fence => Self::LLVMFence,
            Opcode::AtomicCmpXchg => Self::LLVMAtomicCmpXchg,
            Opcode::AtomicRMW => Self::LLVMAtomicRMW,
            Opcode::Resume => Self::LLVMResume,
            Opcode::LandingPad => Self::LLVMLandingPad,
            Opcode::CleanupRet => Self::LLVMCleanupRet,
            Opcode::CatchRet => Self::LLVMCatchRet,
            Opcode::CatchPad => Self::LLVMCatchPad,
            Opcode::CleanupPad => Self::LLVMCleanupPad,
            Opcode::CatchSwitch => Self::LLVMCatchSwitch,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntPredicate {
    LLVMIntEQ,
    LLVMIntNE,
    LLVMIntUGT,
    LLVMIntUGE,
    LLVMIntULT,
    LLVMIntULE,
    LLVMIntSGT,
    LLVMIntSGE,
    LLVMIntSLT,
    LLVMIntSLE,
}

impl From<LLVMIntPredicate> for IntPredicate {
    fn from(predicate: LLVMIntPredicate) -> Self {
        match predicate {
            LLVMIntPredicate::LLVMIntEQ => Self::LLVMIntEQ,
            LLVMIntPredicate::LLVMIntNE => Self::LLVMIntNE,
            LLVMIntPredicate::LLVMIntUGT => Self::LLVMIntUGT,
            LLVMIntPredicate::LLVMIntUGE => Self::LLVMIntUGE,
            LLVMIntPredicate::LLVMIntULT => Self::LLVMIntULT,
            LLVMIntPredicate::LLVMIntULE => Self::LLVMIntULE,
            LLVMIntPredicate::LLVMIntSGT => Self::LLVMIntSGT,
            LLVMIntPredicate::LLVMIntSGE => Self::LLVMIntSGE,
            LLVMIntPredicate::LLVMIntSLT => Self::LLVMIntSLT,
            LLVMIntPredicate::LLVMIntSLE => Self::LLVMIntSLE,
        }
    }
}

impl From<IntPredicate> for LLVMIntPredicate {
    fn from(predicate: IntPredicate) -> Self {
        match predicate {
            IntPredicate::LLVMIntEQ => Self::LLVMIntEQ,
            IntPredicate::LLVMIntNE => Self::LLVMIntNE,
            IntPredicate::LLVMIntUGT => Self::LLVMIntUGT,
            IntPredicate::LLVMIntUGE => Self::LLVMIntUGE,
            IntPredicate::LLVMIntULT => Self::LLVMIntULT,
            IntPredicate::LLVMIntULE => Self::LLVMIntULE,
            IntPredicate::LLVMIntSGT => Self::LLVMIntSGT,
            IntPredicate::LLVMIntSGE => Self::LLVMIntSGE,
            IntPredicate::LLVMIntSLT => Self::LLVMIntSLT,
            IntPredicate::LLVMIntSLE => Self::LLVMIntSLE,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RealPredicate {
    LLVMRealPredicateFalse = 0,
    LLVMRealOEQ,
    LLVMRealOGT,
    LLVMRealOGE,
    LLVMRealOLT,
    LLVMRealOLE,
    LLVMRealONE,
    LLVMRealORD,
    LLVMRealUNO,
    LLVMRealUEQ,
    LLVMRealUGT,
    LLVMRealUGE,
    LLVMRealULT,
    LLVMRealULE,
    LLVMRealUNE,
    LLVMRealPredicateTrue,
}

impl From<LLVMRealPredicate> for RealPredicate {
    fn from(predicate: LLVMRealPredicate) -> Self {
        match predicate {
            LLVMRealPredicate::LLVMRealPredicateFalse => Self::LLVMRealPredicateFalse,
            LLVMRealPredicate::LLVMRealOEQ => Self::LLVMRealOEQ,
            LLVMRealPredicate::LLVMRealOGT => Self::LLVMRealOGT,
            LLVMRealPredicate::LLVMRealOGE => Self::LLVMRealOGE,
            LLVMRealPredicate::LLVMRealOLT => Self::LLVMRealOLT,
            LLVMRealPredicate::LLVMRealOLE => Self::LLVMRealOLE,
            LLVMRealPredicate::LLVMRealONE => Self::LLVMRealONE,
            LLVMRealPredicate::LLVMRealORD => Self::LLVMRealORD,
            LLVMRealPredicate::LLVMRealUNO => Self::LLVMRealUNO,
            LLVMRealPredicate::LLVMRealUEQ => Self::LLVMRealUEQ,
            LLVMRealPredicate::LLVMRealUGT => Self::LLVMRealUGT,
            LLVMRealPredicate::LLVMRealUGE => Self::LLVMRealUGE,
            LLVMRealPredicate::LLVMRealULT => Self::LLVMRealULT,
            LLVMRealPredicate::LLVMRealULE => Self::LLVMRealULE,
            LLVMRealPredicate::LLVMRealUNE => Self::LLVMRealUNE,
            LLVMRealPredicate::LLVMRealPredicateTrue => Self::LLVMRealPredicateTrue,
        }
    }
}

impl From<RealPredicate> for LLVMRealPredicate {
    fn from(predicate: RealPredicate) -> Self {
        match predicate {
            RealPredicate::LLVMRealPredicateFalse => Self::LLVMRealPredicateFalse,
            RealPredicate::LLVMRealOEQ => Self::LLVMRealOEQ,
            RealPredicate::LLVMRealOGT => Self::LLVMRealOGT,
            RealPredicate::LLVMRealOGE => Self::LLVMRealOGE,
            RealPredicate::LLVMRealOLT => Self::LLVMRealOLT,
            RealPredicate::LLVMRealOLE => Self::LLVMRealOLE,
            RealPredicate::LLVMRealONE => Self::LLVMRealONE,
            RealPredicate::LLVMRealORD => Self::LLVMRealORD,
            RealPredicate::LLVMRealUNO => Self::LLVMRealUNO,
            RealPredicate::LLVMRealUEQ => Self::LLVMRealUEQ,
            RealPredicate::LLVMRealUGT => Self::LLVMRealUGT,
            RealPredicate::LLVMRealUGE => Self::LLVMRealUGE,
            RealPredicate::LLVMRealULT => Self::LLVMRealULT,
            RealPredicate::LLVMRealULE => Self::LLVMRealULE,
            RealPredicate::LLVMRealUNE => Self::LLVMRealUNE,
            RealPredicate::LLVMRealPredicateTrue => Self::LLVMRealPredicateTrue,
        }
    }
}
