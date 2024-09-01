use crate::CUint;
use llvm_sys::{core, LLVMIntPredicate, LLVMOpcode, LLVMRealPredicate};
use std::fmt::Display;
use std::ops::Deref;

pub mod context;
pub mod module;
pub mod types;
pub mod values;

/// Represents an LLVM address space.
///
/// The `AddressSpace` struct encapsulates a numeric value that indicates a specific address space
/// in LLVM. Address spaces are used in LLVM to distinguish between different regions of memory, such as
/// global memory, local memory, and private memory, especially in contexts like GPUs or other specialized
/// hardware where different memory regions have different characteristics.
///
/// # Attributes
///
/// - Wrapped address value - the underlying numeric value representing the address space.
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
///
/// ## Panics
/// This function is purely informative and panics with a message about the call
/// being unavailable. Since there are no cases in which it can be called in
/// safe code. For raw access, if there is such a need, must be called
/// `LLVMDisposeMessage` directly.
pub fn dispose_message(_message: libc::c_char) {
    unreachable!(
        "LLVMDisposeMessage is unsafe adn restricted to operated to operate directly for safe code"
    );
}

/// LLVM version representation
///
/// The `Version` struct encapsulates the major, minor, and patch components of the LLVM version.
/// This struct provides methods to initialize and retrieve the version information.
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Version {
    /// Init and return current LLVM version
    ///
    /// # Details
    ///
    /// Initializes and returns the current LLVM version.
    ///
    /// This method queries the LLVM library for its version information and returns a `Version` instance
    /// containing the major, minor, and patch components of the LLVM version.
    ///
    /// # Returns
    ///
    /// A `Version` instance with the current LLVM version.
    ///
    /// # Example
    ///
    /// ```rust
    /// let llvm_version = Version::new();
    /// ```
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
    ///
    /// # Details
    ///
    ///  Returns the LLVM version as a tuple `(major, minor, patch)`.
    ///
    /// This method provides access to the individual components of the LLVM version stored in this `Version` instance.
    ///
    /// # Returns
    ///
    /// A tuple `(u32, u32, u32)` representing the major, minor, and patch components of the LLVM version.
    ///
    /// # Example
    ///
    /// ```rust
    /// let llvm_version = Version::new();
    /// let (major, minor, patch) = llvm_version.get();
    /// ```
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

/// Represents the various integer comparison predicates in LLVM IR.
///
/// The `IntPredicate` enum defines the possible predicates that can be used for integer comparisons
/// in LLVM IR. These predicates specify the condition under which an integer comparison is considered true.
/// The predicates cover both signed and unsigned comparisons, as well as equality checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntPredicate {
    /// Represents an equality comparison (`==`). This predicate is true if the two integers are equal.
    IntEQ,
    /// Represents an inequality comparison (`!=`). This predicate is true if the two integers are not equal.
    IntNE,
    /// Represents an unsigned greater than comparison (`>`). This predicate is true if the first integer is greater than the second, treating both as unsigned values.
    IntUGT,
    /// Represents an unsigned greater than or equal comparison (`>=`). This predicate is true if the first integer is greater than or equal to the second, treating both as unsigned values.
    IntUGE,
    /// Represents an unsigned less than comparison (`<`). This predicate is true if the first integer is less than the second, treating both as unsigned values.
    IntULT,
    /// Represents an unsigned less than or equal comparison (`<=`). This predicate is true if the first integer is less than or equal to the second, treating both as unsigned values.
    IntULE,
    /// Represents a signed greater than comparison (`>`). This predicate is true if the first integer is greater than the second, treating both as signed values.
    IntSGT,
    /// Represents a signed greater than or equal comparison (`>=`). This predicate is true if the first integer is greater than or equal to the second, treating both as signed values.
    IntSGE,
    /// Represents a signed less than comparison (`<`). This predicate is true if the first integer is less than the second, treating both as signed values.
    IntSLT,
    /// Represents a signed less than or equal comparison (`<=`). This predicate is true if the first integer is less than or equal to the second, treating both as signed values.
    IntSLE,
}

impl From<LLVMIntPredicate> for IntPredicate {
    fn from(predicate: LLVMIntPredicate) -> Self {
        match predicate {
            LLVMIntPredicate::LLVMIntEQ => Self::IntEQ,
            LLVMIntPredicate::LLVMIntNE => Self::IntNE,
            LLVMIntPredicate::LLVMIntUGT => Self::IntUGT,
            LLVMIntPredicate::LLVMIntUGE => Self::IntUGE,
            LLVMIntPredicate::LLVMIntULT => Self::IntULT,
            LLVMIntPredicate::LLVMIntULE => Self::IntULE,
            LLVMIntPredicate::LLVMIntSGT => Self::IntSGT,
            LLVMIntPredicate::LLVMIntSGE => Self::IntSGE,
            LLVMIntPredicate::LLVMIntSLT => Self::IntSLT,
            LLVMIntPredicate::LLVMIntSLE => Self::IntSLE,
        }
    }
}

impl From<IntPredicate> for LLVMIntPredicate {
    fn from(predicate: IntPredicate) -> Self {
        match predicate {
            IntPredicate::IntEQ => Self::LLVMIntEQ,
            IntPredicate::IntNE => Self::LLVMIntNE,
            IntPredicate::IntUGT => Self::LLVMIntUGT,
            IntPredicate::IntUGE => Self::LLVMIntUGE,
            IntPredicate::IntULT => Self::LLVMIntULT,
            IntPredicate::IntULE => Self::LLVMIntULE,
            IntPredicate::IntSGT => Self::LLVMIntSGT,
            IntPredicate::IntSGE => Self::LLVMIntSGE,
            IntPredicate::IntSLT => Self::LLVMIntSLT,
            IntPredicate::IntSLE => Self::LLVMIntSLE,
        }
    }
}

/// Represents the various floating-point comparison predicates in LLVM IR.
///
/// The `RealPredicate` enum defines the possible predicates that can be used for floating-point comparisons
/// in LLVM IR. These predicates specify the conditions under which a floating-point comparison is considered true.
/// The predicates include ordered and unordered comparisons, as well as equality and inequality checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RealPredicate {
    /// Represents a predicate that always returns false. No comparison is true under this predicate.
    RealPredicateFalse = 0,
    /// Represents an ordered equality comparison (`==`). This predicate is true if the two floating-point numbers are equal and neither is NaN.
    RealOEQ,
    /// Represents an ordered greater than comparison (`>`). This predicate is true if the first floating-point number is greater than the second and neither is NaN.
    RealOGT,
    /// Represents an ordered greater than or equal comparison (`>=`). This predicate is true if the first floating-point number is greater than or equal to the second and neither is NaN.
    RealOGE,
    /// Represents an ordered less than comparison (`<`). This predicate is true if the first floating-point number is less than the second and neither is NaN.
    RealOLT,
    /// Represents an ordered less than or equal comparison (`<=`). This predicate is true if the first floating-point number is less than or equal to the second and neither is NaN.
    RealOLE,
    /// Represents an ordered inequality comparison (`!=`). This predicate is true if the two floating-point numbers are not equal and neither is NaN.
    RealONE,
    /// Represents an ordered comparison. This predicate is true if neither of the floating-point numbers is NaN.
    RealORD,
    /// Represents an unordered comparison. This predicate is true if either of the floating-point numbers is NaN.
    RealUNO,
    /// Represents an unordered equality comparison. This predicate is true if the two floating-point numbers are equal or either is NaN.
    RealUEQ,
    /// Represents an unordered greater than comparison. This predicate is true if the first floating-point number is greater than the second or either is NaN.
    RealUGT,
    /// Represents an unordered greater than or equal comparison. This predicate is true if the first floating-point number is greater than or equal to the second or either is NaN.
    RealUGE,
    /// Represents an unordered less than comparison. This predicate is true if the first floating-point number is less than the second or either is NaN.
    RealULT,
    /// Represents an unordered less than or equal comparison. This predicate is true if the first floating-point number is less than or equal to the second or either is NaN.
    RealULE,
    /// Represents an unordered inequality comparison. This predicate is true if the two floating-point numbers are not equal or either is NaN.
    RealUNE,
    /// Represents a predicate that always returns true. All comparisons are true under this predicate.
    RealPredicateTrue,
}

impl From<LLVMRealPredicate> for RealPredicate {
    fn from(predicate: LLVMRealPredicate) -> Self {
        match predicate {
            LLVMRealPredicate::LLVMRealPredicateFalse => Self::RealPredicateFalse,
            LLVMRealPredicate::LLVMRealOEQ => Self::RealOEQ,
            LLVMRealPredicate::LLVMRealOGT => Self::RealOGT,
            LLVMRealPredicate::LLVMRealOGE => Self::RealOGE,
            LLVMRealPredicate::LLVMRealOLT => Self::RealOLT,
            LLVMRealPredicate::LLVMRealOLE => Self::RealOLE,
            LLVMRealPredicate::LLVMRealONE => Self::RealONE,
            LLVMRealPredicate::LLVMRealORD => Self::RealORD,
            LLVMRealPredicate::LLVMRealUNO => Self::RealUNO,
            LLVMRealPredicate::LLVMRealUEQ => Self::RealUEQ,
            LLVMRealPredicate::LLVMRealUGT => Self::RealUGT,
            LLVMRealPredicate::LLVMRealUGE => Self::RealUGE,
            LLVMRealPredicate::LLVMRealULT => Self::RealULT,
            LLVMRealPredicate::LLVMRealULE => Self::RealULE,
            LLVMRealPredicate::LLVMRealUNE => Self::RealUNE,
            LLVMRealPredicate::LLVMRealPredicateTrue => Self::RealPredicateTrue,
        }
    }
}

impl From<RealPredicate> for LLVMRealPredicate {
    fn from(predicate: RealPredicate) -> Self {
        match predicate {
            RealPredicate::RealPredicateFalse => Self::LLVMRealPredicateFalse,
            RealPredicate::RealOEQ => Self::LLVMRealOEQ,
            RealPredicate::RealOGT => Self::LLVMRealOGT,
            RealPredicate::RealOGE => Self::LLVMRealOGE,
            RealPredicate::RealOLT => Self::LLVMRealOLT,
            RealPredicate::RealOLE => Self::LLVMRealOLE,
            RealPredicate::RealONE => Self::LLVMRealONE,
            RealPredicate::RealORD => Self::LLVMRealORD,
            RealPredicate::RealUNO => Self::LLVMRealUNO,
            RealPredicate::RealUEQ => Self::LLVMRealUEQ,
            RealPredicate::RealUGT => Self::LLVMRealUGT,
            RealPredicate::RealUGE => Self::LLVMRealUGE,
            RealPredicate::RealULT => Self::LLVMRealULT,
            RealPredicate::RealULE => Self::LLVMRealULE,
            RealPredicate::RealUNE => Self::LLVMRealUNE,
            RealPredicate::RealPredicateTrue => Self::LLVMRealPredicateTrue,
        }
    }
}
