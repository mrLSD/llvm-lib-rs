use crate::CUint;
use llvm_sys::core;
use std::fmt::Display;

pub mod context;
pub mod module;
pub mod types;
pub mod value;

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
