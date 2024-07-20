use crate::types::{DiagnosticSeverity, TypeRef};
use crate::{CInt, CStr, CString, CUint, GetRef, SizeT, UnsafeMutVoidPtr};
use llvm_sys::core;
use llvm_sys::prelude::{LLVMAttributeRef, LLVMContextRef, LLVMDiagnosticInfoRef};
use llvm_sys::{LLVMDiagnosticHandler, LLVMYieldCallback};
use std::ops::Deref;

/// LLVM Context wrapper
pub struct ContextRef(LLVMContextRef);

impl From<LLVMContextRef> for ContextRef {
    fn from(value: LLVMContextRef) -> Self {
        Self(value)
    }
}

impl ContextRef {
    /// Create new LLVM Context
    #[must_use]
    pub fn new() -> Self {
        Self::create()
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
    pub fn create() -> Self {
        unsafe { Self(core::LLVMContextCreate()) }
    }

    /// Retrieves the global context instance.
    ///
    /// The global context is particularly convenient instance managed by LLVM
    /// itself. It is the default context provided for any operations that
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
        unsafe { Self(core::LLVMGetGlobalContext()) }
    }

    /// Set debug diagnostic handler for this context.
    ///
    /// ## Safety
    /// To provide safe operations wi with diagnostic context should be set:
    /// - `handler` - LLVM diagnostic function (handler)
    /// - `diagnostic_context` - raw pointer for diagnostic
    /// NOTE: it's much safer to use raw pointer in that case than `std::ptr::NonNull` structs.
    pub fn set_diagnostic_handler(
        &self,
        handler: LLVMDiagnosticHandler,
        diagnostic_context: UnsafeMutVoidPtr,
    ) {
        unsafe {
            core::LLVMContextSetDiagnosticHandler(self.0, handler, *diagnostic_context);
        }
    }

    /// Get the diagnostic handler of this context.
    #[must_use]
    pub fn get_diagnostic_handler(&self) -> LLVMDiagnosticHandler {
        unsafe { core::LLVMContextGetDiagnosticHandler(self.0) }
    }

    /// Get the diagnostic context of this context.
    #[must_use]
    pub fn get_diagnostic_context(&self) -> UnsafeMutVoidPtr {
        unsafe {
            let raw_ptr = core::LLVMContextGetDiagnosticContext(self.0);
            UnsafeMutVoidPtr(raw_ptr)
        }
    }

    /// Set the yield callback function for this context.
    pub fn set_yield_callback(&self, callback: LLVMYieldCallback, opaque_handle: UnsafeMutVoidPtr) {
        unsafe { core::LLVMContextSetYieldCallback(self.0, callback, *opaque_handle) }
    }

    /// Retrieve whether the given context is set to discard all value names.
    #[must_use]
    pub fn should_discard_value_names(&self) -> bool {
        unsafe { core::LLVMContextShouldDiscardValueNames(self.0) != 0 }
    }

    /// Set whether the given context discards all value names.
    ///
    /// If true, only the names of `GlobalValue` objects will be available in the IR.
    /// This can be used to save memory and runtime, especially in release mode.
    pub fn set_discard_value_names(&self, discard: bool) {
        unsafe {
            core::LLVMContextSetDiscardValueNames(self.get_ref(), *CInt::from(discard));
        }
    }

    /// Deinitialize this value and dispose of its resources.
    ///
    /// Destroy a context instance.
    /// This should be called for every call to `self::context_create` (`LLVMContextCreate()`) or memory
    /// will be leaked.
    pub fn dispose(&self) {
        unsafe { core::LLVMContextDispose(self.get_ref()) }
    }

    /// Get  Metadata `KindId` by name in current Context.
    /// Useful for working with Metadata.
    #[must_use]
    pub fn get_md_kind_id_in_context(&self, name: &str) -> MetadataKindId {
        MetadataKindId::get_md_kind_id_in_context(self, name)
    }

    /// Create an enum attribute.
    #[must_use]
    pub fn create_enum_attribute(&self, kind_id: u32, val: u64) -> AttributeRef {
        AttributeRef::create_enum_attribute(self, kind_id, val)
    }

    /// Create a type attribute in context
    #[must_use]
    pub fn create_type_attribute(&self, kind_id: u32, type_ref: &TypeRef) -> AttributeRef {
        AttributeRef::create_type_attribute(self, kind_id, type_ref)
    }

    /// Create a string attribute in context
    #[must_use]
    pub fn create_string_attribute(&self, key: &str, value: &str) -> AttributeRef {
        AttributeRef::create_string_attribute(self, key, value)
    }

    /// Obtain a Type from a context by its registered name.
    #[must_use]
    pub fn get_type_by_name2(&self, name: &str) -> Option<TypeRef> {
        let c_name = CString::from(name);
        let type_ref = unsafe { core::LLVMGetTypeByName2(self.get_ref(), c_name.as_ptr()) };
        if type_ref.is_null() {
            None
        } else {
            Some(TypeRef::from(type_ref))
        }
    }
}

impl Drop for ContextRef {
    /// Dispose  context
    fn drop(&mut self) {
        self.dispose();
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

/// Wrapper for `MetadataKindId`
pub struct MetadataKindId(pub u32);

impl MetadataKindId {
    /// Get `MetadataKindId` by name in current `Context`.
    /// Useful for working with Metadata.
    #[must_use]
    pub fn get_md_kind_id_in_context(context: &ContextRef, name: &str) -> Self {
        let c_name = CString::from(name);
        let id = unsafe {
            core::LLVMGetMDKindIDInContext(
                context.get_ref(),
                c_name.as_ptr(),
                *CUint::from(c_name.to_bytes().len()),
            )
        };
        Self(id)
    }

    /// Get  Metadata `KindId` by name.
    /// Useful for working with Metadata.
    #[must_use]
    pub fn get_md_kind_id(name: &str) -> Self {
        let c_name = CString::from(name);
        let id = unsafe {
            core::LLVMGetMDKindID(c_name.as_ptr(), *CUint::from(c_name.to_bytes().len()))
        };
        Self(id)
    }
}

/// LLVM Attributes structure wrapper
pub struct AttributeRef(LLVMAttributeRef);

impl From<LLVMAttributeRef> for AttributeRef {
    fn from(value: LLVMAttributeRef) -> Self {
        Self(value)
    }
}

impl GetRef for AttributeRef {
    type RawRef = LLVMAttributeRef;
    fn get_ref(&self) -> Self::RawRef {
        self.0
    }
}

impl AttributeRef {
    /// Return the unique id given the name of the enum attribute,
    /// or 0 if no attribute by that name exists.
    ///
    /// See <http://llvm.org/docs/LangRef.html#parameter-attributes>
    /// and <http://llvm.org/docs/LangRef.html#function-attributes>
    /// for the list of available attributes.
    ///
    /// NB: Attribute names and/or id are subject to change without
    /// going through the C API deprecation cycle.
    #[must_use]
    pub fn get_enum_attribute_kind_for_name(name: &str) -> u32 {
        let c_name = CString::from(name);
        unsafe {
            core::LLVMGetEnumAttributeKindForName(c_name.as_ptr(), *SizeT(c_name.to_bytes().len()))
        }
    }

    /// Get last enum attribute
    #[must_use]
    pub fn get_last_enum_attribute_kind() -> u32 {
        unsafe { core::LLVMGetLastEnumAttributeKind() }
    }

    /// Create an enum attribute.
    #[must_use]
    pub fn create_enum_attribute(context: &ContextRef, kind_id: u32, val: u64) -> Self {
        let attr =
            unsafe { core::LLVMCreateEnumAttribute(context.get_ref(), *CUint::from(kind_id), val) };
        Self(attr)
    }

    /// Get the unique id corresponding to the enum attribute passed as argument.
    #[must_use]
    pub fn get_enum_attribute_kind(&self) -> u32 {
        unsafe { core::LLVMGetEnumAttributeKind(self.0) }
    }

    /// Get the enum attribute's value. 0 is returned if none exists.
    #[must_use]
    pub fn get_enum_attribute_value(&self) -> u64 {
        unsafe { core::LLVMGetEnumAttributeValue(self.0) }
    }

    /// Create a type attribute
    #[must_use]
    pub fn create_type_attribute(context: &ContextRef, kind_id: u32, type_ref: &TypeRef) -> Self {
        let attr = unsafe {
            core::LLVMCreateTypeAttribute(context.get_ref(), kind_id, type_ref.get_ref())
        };
        Self(attr)
    }

    /// Get the type attribute's value.
    #[must_use]
    pub fn get_type_attribute_value(&self) -> TypeRef {
        let type_ref = unsafe { core::LLVMGetTypeAttributeValue(self.0) };
        type_ref.into()
    }

    /// Create a string attribute.
    #[must_use]
    pub fn create_string_attribute(context: &ContextRef, key: &str, value: &str) -> Self {
        let c_key = CString::from(key);
        let c_value = CString::from(value);
        let attr = unsafe {
            core::LLVMCreateStringAttribute(
                context.get_ref(),
                c_key.as_ptr(),
                *CUint::from(c_key.to_bytes().len()),
                c_value.as_ptr(),
                *CUint::from(c_value.to_bytes().len()),
            )
        };
        Self(attr)
    }

    /// Get the string attribute's kind.
    #[must_use]
    pub fn get_string_attribute_kind(&self) -> Option<String> {
        let mut length = *CUint::from(0_usize);
        unsafe {
            let c_str = core::LLVMGetStringAttributeKind(self.0, &mut length);
            if c_str.is_null() {
                return None;
            }
            Some(CStr::new(c_str).to_string())
        }
    }

    /// Get the string attribute's value.
    #[must_use]
    pub fn get_string_attribute_value(&self) -> Option<String> {
        let mut length = *CUint::from(0_usize);
        unsafe {
            let c_str = core::LLVMGetStringAttributeValue(self.get_ref(), &mut length);
            if c_str.is_null() {
                return None;
            }
            Some(CStr::new(c_str).to_string())
        }
    }

    /// Check for the  types of attributes.
    #[must_use]
    pub fn is_enum(&self) -> bool {
        unsafe { core::LLVMIsEnumAttribute(self.get_ref()) != 0 }
    }

    /// Check for the  types of attributes.
    #[must_use]
    pub fn is_string(&self) -> bool {
        unsafe { core::LLVMIsStringAttribute(self.get_ref()) != 0 }
    }

    /// Check for the  types of attributes.
    #[must_use]
    pub fn is_type(&self) -> bool {
        unsafe { core::LLVMIsTypeAttribute(self.get_ref()) != 0 }
    }
}

/// LLVM Diagnostic Info structure wrapper
pub struct DiagnosticInfoRef(LLVMDiagnosticInfoRef);

impl From<LLVMDiagnosticInfoRef> for DiagnosticInfoRef {
    fn from(value: LLVMDiagnosticInfoRef) -> Self {
        Self(value)
    }
}

impl GetRef for DiagnosticInfoRef {
    type RawRef = LLVMDiagnosticInfoRef;
    fn get_ref(&self) -> Self::RawRef {
        self.0
    }
}

impl DiagnosticInfoRef {
    /// Return a string representation of the `DiagnosticInfo`. Use
    /// [`crate::core::dispose_message`] (`LLVMDisposeMessage`) to free the string.
    #[must_use]
    pub fn get_description(&self) -> Option<String> {
        unsafe {
            let c_str = core::LLVMGetDiagInfoDescription(self.get_ref());
            if c_str.is_null() {
                return None;
            }
            let value = CStr::new(c_str).to_string();
            // Dispose message
            crate::core::dispose_message(c_str);
            Some(value)
        }
    }

    /// Return an enum `DiagnosticSeverity` type
    #[must_use]
    pub fn get_severity(&self) -> DiagnosticSeverity {
        unsafe {
            let severity = core::LLVMGetDiagInfoSeverity(self.get_ref());
            DiagnosticSeverity::from(severity)
        }
    }
}
