use llvm_sys::core::{
    LLVMContextCreate, LLVMContextDispose, LLVMContextGetDiagnosticContext,
    LLVMContextGetDiagnosticHandler, LLVMContextSetDiagnosticHandler, LLVMGetGlobalContext,
};
use llvm_sys::prelude::LLVMContextRef;
use llvm_sys::LLVMDiagnosticHandler;
use std::ffi;
use std::ops::Deref;

/// LLVM Context wrapper
pub struct ContextRef(LLVMContextRef);

impl ContextRef {
    /// Create new LLVM Context
    #[must_use]
    pub fn new() -> Self {
        Self::context_create()
    }

    /// Get LLVM raw context reference
    #[must_use]
    pub const fn get(&self) -> LLVMContextRef {
        self.0
    }

    #[must_use]
    pub fn context_create() -> Self {
        unsafe { Self(LLVMContextCreate()) }
    }

    /// When it possible use `context_create`
    #[must_use]
    pub fn get_global_context() -> Self {
        unsafe { Self(LLVMGetGlobalContext()) }
    }

    /// Set debug diagnostic handler
    ///
    /// ## Safety
    /// To provide safe operations wi with diagnostic context should be set:
    /// - `handler` - LLVM diagnostic function (handler)
    /// - `diagnostic_context` - raw pointer for diagnostic
    /// NOTE: it's much safer to use raw pointer in that case than `std::ptr::NonNull` structs.
    pub unsafe fn context_set_diagnostic_handler(
        &self,
        handler: LLVMDiagnosticHandler,
        diagnostic_context: *mut ffi::c_void,
    ) {
        LLVMContextSetDiagnosticHandler(self.0, handler, diagnostic_context);
    }

    #[must_use]
    pub fn context_get_diagnostic_handler(&self) -> LLVMDiagnosticHandler {
        unsafe { LLVMContextGetDiagnosticHandler(self.0) }
    }

    #[must_use]
    pub fn context_get_diagnostic_context(&self) -> *mut ffi::c_void {
        unsafe { LLVMContextGetDiagnosticContext(self.0) }
    }
}

impl Drop for ContextRef {
    /// Dispose  context
    fn drop(&mut self) {
        unsafe {
            LLVMContextDispose(self.0);
        }
    }
}

impl Deref for ContextRef {
    type Target = LLVMContextRef;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
