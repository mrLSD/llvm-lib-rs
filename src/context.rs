use crate::types::{AttributeRef, TypeRef};
use crate::{CInt, CStr, CString, CUint, UnsafeMutVoidPtr};
use llvm_sys::core::{
    LLVMContextCreate, LLVMContextDispose, LLVMContextGetDiagnosticContext,
    LLVMContextGetDiagnosticHandler, LLVMContextSetDiagnosticHandler,
    LLVMContextSetDiscardValueNames, LLVMContextSetYieldCallback,
    LLVMContextShouldDiscardValueNames, LLVMGetGlobalContext, LLVMGetStringAttributeValue,
    LLVMGetTypeByName2, LLVMIsEnumAttribute, LLVMIsStringAttribute, LLVMIsTypeAttribute,
};
use llvm_sys::prelude::LLVMContextRef;
use llvm_sys::{LLVMDiagnosticHandler, LLVMYieldCallback};
use std::ops::Deref;

/// LLVM Context wrapper
pub struct ContextRef(LLVMContextRef);

pub trait GetRef {
    /// Raw LLVM reference type
    type RawRef;
    /// Get LLVM raw reference
    #[must_use]
    fn get_ref(&self) -> Self::RawRef;
}

impl ContextRef {
    /// Create new LLVM Context
    #[must_use]
    pub fn new() -> Self {
        Self::context_create()
    }
}

/// LLVM specific implementations
impl ContextRef {
    /// Create a new context.
    ///
    /// ## Safety
    /// Every call to this function should be paired with a call to
    /// `Self::context_dispose` or the context will leak memory.
    #[must_use]
    pub fn context_create() -> Self {
        unsafe { Self(LLVMContextCreate()) }
    }

    /// Retrieves the global context instance.
    ///
    /// The global context is an particularly convenient instance managed by LLVM
    /// itself.  It is the default context provided for any operations that
    /// require it.
    ///
    /// ### Safety
    /// Failure to specify the correct context in concurrent
    /// environments can lead to data corruption.  In general, it is always
    /// recommended that each thread of execution attempting to access the LLVM
    /// API have its own `Context` instance, rather than rely on this global
    /// context.
    #[must_use]
    pub fn get_global_context() -> Self {
        unsafe { Self(LLVMGetGlobalContext()) }
    }

    /// Set debug diagnostic handler for this context.
    ///
    /// ## Safety
    /// To provide safe operations wi with diagnostic context should be set:
    /// - `handler` - LLVM diagnostic function (handler)
    /// - `diagnostic_context` - raw pointer for diagnostic
    /// NOTE: it's much safer to use raw pointer in that case than `std::ptr::NonNull` structs.
    pub fn context_set_diagnostic_handler(
        &self,
        handler: LLVMDiagnosticHandler,
        diagnostic_context: UnsafeMutVoidPtr,
    ) {
        unsafe {
            LLVMContextSetDiagnosticHandler(self.0, handler, *diagnostic_context);
        }
    }

    /// Get the diagnostic handler of this context.
    #[must_use]
    pub fn context_get_diagnostic_handler(&self) -> LLVMDiagnosticHandler {
        unsafe { LLVMContextGetDiagnosticHandler(self.0) }
    }

    /// Get the diagnostic context of this context.
    #[must_use]
    pub fn context_get_diagnostic_context(&self) -> UnsafeMutVoidPtr {
        unsafe {
            let raw_ptr = LLVMContextGetDiagnosticContext(self.0);
            UnsafeMutVoidPtr(raw_ptr)
        }
    }

    /// Set the yield callback function for this context.
    pub fn context_set_yield_callback(
        &self,
        callback: LLVMYieldCallback,
        opaque_handle: UnsafeMutVoidPtr,
    ) {
        unsafe { LLVMContextSetYieldCallback(self.0, callback, *opaque_handle) }
    }

    /// Retrieve whether the given context is set to discard all value names.
    #[must_use]
    pub fn context_should_discard_value_names(&self) -> bool {
        unsafe { LLVMContextShouldDiscardValueNames(self.0) != 0 }
    }

    /// Set whether the given context discards all value names.
    ///
    /// If true, only the names of `GlobalValue` objects will be available in the IR.
    /// This can be used to save memory and runtime, especially in release mode.
    pub fn context_set_discard_value_names(&self, discard: bool) {
        unsafe {
            LLVMContextSetDiscardValueNames(self.get_ref(), *CInt::from(discard));
        }
    }

    /// Deinitialize this value and dispose of its resources.
    ///
    /// Destroy a context instance.
    /// This should be called for every call to `self::context_create` (`LLVMContextCreate()`) or memory
    /// will be leaked.
    pub fn context_dispose(&self) {
        unsafe { LLVMContextDispose(self.get_ref()) }
    }

    /*
             pub fn get_diag_info_description(di: NonNull<c_void>) -> String {
    }
             pub fn get_diag_info_severity(di: NonNull<c_void>) -> LLVMDiagnosticSeverity {}

             pub fn get_md_kind_id_in_context(context: NonNull<c_void>, name: &str) -> u32 {}

             pub fn get_md_kind_id(name: &str) -> u32 {}

             pub fn get_enum_attribute_kind_for_name(name: &str) -> u32 {}

             pub fn get_last_enum_attribute_kind() -> u32 {}

             pub fn create_enum_attribute() -> Option<NonNull<c_void>> {}

             pub fn get_enum_attribute_kind(attr: NonNull<c_void>) -> u32 {}

             pub fn get_enum_attribute_value(attr: NonNull<c_void>) -> u64 {}

             pub fn create_type_attribute() -> Option<NonNull<c_void>> {}

             pub fn get_type_attribute_value(attr: NonNull<c_void>) -> Option<NonNull<c_void>> {}

             pub fn create_string_attribute) -> Option<NonNull<c_void>> {}

             pub fn get_string_attribute_kind(attr: AttributeRef,length:u32) -> String {}
        */

    /// Get the string attribute's value.
    #[must_use]
    pub fn get_string_attribute_value(attr: &AttributeRef) -> Option<String> {
        attr.get_string_attribute_value()
    }

    /// Check for the  types of attributes.
    #[must_use]
    pub fn is_enum_attribute(attr: &AttributeRef) -> bool {
        attr.is_enum()
    }

    /// Check for the  types of attributes.
    #[must_use]
    pub fn is_string_attribute(attr: &AttributeRef) -> bool {
        attr.is_string()
    }

    /// Check for the  types of attributes.
    #[must_use]
    pub fn is_type_attribute(attr: &AttributeRef) -> bool {
        attr.is_type()
    }

    /// Obtain a Type from a context by its registered name.
    #[must_use]
    pub fn get_type_by_name2(&self, name: &str) -> Option<TypeRef> {
        let c_name = CString::from(name);
        let type_ref = unsafe { LLVMGetTypeByName2(self.get_ref(), c_name.as_ptr()) };
        if type_ref.is_null() {
            None
        } else {
            Some(TypeRef::from(type_ref))
        }
    }
}

impl AttributeRef {
    /// Get the string attribute's value.
    #[must_use]
    pub fn get_string_attribute_value(&self) -> Option<String> {
        let mut length = *CUint::from(0_usize);
        unsafe {
            let raw_c_str = LLVMGetStringAttributeValue(self.get_ref(), &mut length);
            if raw_c_str.is_null() {
                return None;
            }
            Some(CStr::new(raw_c_str).to_string())
        }
    }

    /// Check for the  types of attributes.
    #[must_use]
    pub fn is_enum(&self) -> bool {
        unsafe { LLVMIsEnumAttribute(self.get_ref()) != 0 }
    }

    /// Check for the  types of attributes.
    #[must_use]
    pub fn is_string(&self) -> bool {
        unsafe { LLVMIsStringAttribute(self.get_ref()) != 0 }
    }

    /// Check for the  types of attributes.
    #[must_use]
    pub fn is_type(&self) -> bool {
        unsafe { LLVMIsTypeAttribute(self.get_ref()) != 0 }
    }
}

impl Drop for ContextRef {
    /// Dispose  context
    fn drop(&mut self) {
        self.context_dispose();
    }
}

impl Deref for ContextRef {
    type Target = LLVMContextRef;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl GetRef for ContextRef {
    type RawRef = LLVMContextRef;
    fn get_ref(&self) -> Self::RawRef {
        self.0
    }
}
