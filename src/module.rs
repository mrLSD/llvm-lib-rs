use crate::context::ContextRef;
use crate::{CStr, CString, GetRef, SizeT};
use llvm_sys::core::{
    LLVMCloneModule, LLVMCopyModuleFlagsMetadata, LLVMDisposeModule,
    LLVMDisposeModuleFlagsMetadata, LLVMDumpModule, LLVMGetDataLayoutStr, LLVMGetModuleIdentifier,
    LLVMGetSourceFileName, LLVMGetTarget, LLVMModuleCreateWithName,
    LLVMModuleCreateWithNameInContext, LLVMSetDataLayout, LLVMSetModuleIdentifier,
    LLVMSetSourceFileName, LLVMSetTarget,
};
use llvm_sys::prelude::{LLVMModuleFlagEntry, LLVMModuleRef};
use std::ops::Deref;

/// Wrapper for `ModuleFlagEntry`
// TODO: manage lengtn read
#[allow(dead_code)]
pub struct ModuleFlagEntry(*mut LLVMModuleFlagEntry, SizeT);

/// LLVM Module wrapper
pub struct ModuleRef(LLVMModuleRef);

impl ModuleRef {
    /// Create LLVM module with name
    ///
    /// ## Panics
    /// It panics if module creation is null
    #[must_use]
    pub fn new(module_name: &str) -> Self {
        Self::create_module_with_name(module_name)
    }

    /// ## Panics
    /// It panics if module creation is null
    #[must_use]
    pub fn create_module_with_name(module_name: &str) -> Self {
        let c_name = CString::from(module_name);
        let module_ref = unsafe { LLVMModuleCreateWithName(c_name.as_ptr()) };
        // Force panic as it's unexpected situation
        assert!(!module_ref.is_null(), "Failed to create LLVM module");
        Self(module_ref)
    }

    /// ## Panics
    /// It panics if module creation is null
    #[must_use]
    pub fn create_module_with_name_in_context(module_name: &str, context: &ContextRef) -> Self {
        let c_name = CString::from(module_name);
        let module_ref =
            unsafe { LLVMModuleCreateWithNameInContext(c_name.as_ptr(), context.get_ref()) };
        // Force panic as it's unexpected situation
        assert!(!module_ref.is_null(), "Failed to create LLVM module");
        Self(module_ref)
    }

    /// Return an exact copy of the current module.
    #[must_use]
    pub fn clone_module(&self) -> Self {
        let module_ref = unsafe { LLVMCloneModule(self.0) };
        Self(module_ref)
    }

    /// Obtain the identifier of a module.
    #[must_use]
    pub fn get_module_identifier(&self) -> Option<String> {
        let mut length = *SizeT::from(0_usize);
        unsafe {
            let c_str = LLVMGetModuleIdentifier(self.0, &mut length);
            if c_str.is_null() {
                None
            } else {
                Some(CStr::new(c_str).to_string())
            }
        }
    }

    /// Set the identifier of a module to a string Ident with length Len.
    pub fn set_module_identifier(&self, ident: &str) {
        let c_ident = CString::from(ident);
        unsafe {
            LLVMSetModuleIdentifier(
                self.0,
                c_ident.as_ptr(),
                *SizeT::from(c_ident.to_bytes().len()),
            );
        }
    }

    /// Obtain the module's original source file name.
    #[must_use]
    pub fn get_source_file_name(&self) -> Option<String> {
        let mut length = *SizeT::from(0_usize);
        unsafe {
            let c_str = LLVMGetSourceFileName(self.0, &mut length);
            if c_str.is_null() {
                None
            } else {
                Some(CStr::new(c_str).to_string())
            }
        }
    }

    /// Set the original source file name of a module to a string Name with length Len.
    pub fn set_source_file_name(&self, name: &str) {
        let c_name = CString::from(name);
        unsafe {
            LLVMSetSourceFileName(
                self.0,
                c_name.as_ptr(),
                *SizeT::from(c_name.to_bytes().len()),
            );
        }
    }

    /// Get module data layout
    #[must_use]
    pub fn get_data_layout_str(&self) -> Option<String> {
        unsafe {
            let c_str = LLVMGetDataLayoutStr(self.0);
            if c_str.is_null() {
                None
            } else {
                Some(CStr::new(c_str).to_string())
            }
        }
    }

    /// Set the data layout for a module.
    pub fn set_data_layout(&self, data_layout_str: &str) {
        let c_data_layout_str = CString::from(data_layout_str);
        unsafe {
            LLVMSetDataLayout(self.0, c_data_layout_str.as_ptr());
        }
    }

    /// Obtain the target triple for a module.
    #[must_use]
    pub fn get_target(&self) -> Option<String> {
        unsafe {
            let c_str = LLVMGetTarget(self.0);
            if c_str.is_null() {
                None
            } else {
                Some(CStr::new(c_str).to_string())
            }
        }
    }

    /// Set the target triple for a module.
    pub fn set_target(&self, triple: &str) {
        let c_triple = CString::from(triple);
        unsafe {
            LLVMSetTarget(self.0, c_triple.as_ptr());
        }
    }

    /// Returns the module flags as an array of flag-key-value triples.  The caller
    /// is responsible for freeing this array by calling  `dispose_module_flags_metadata`.
    #[must_use]
    pub fn copy_module_flags_metadata(&self) -> Option<ModuleFlagEntry> {
        unsafe {
            let mut length = SizeT(0_usize);
            let entries = LLVMCopyModuleFlagsMetadata(self.0, &mut *length);
            if entries.is_null() {
                None
            } else {
                Some(ModuleFlagEntry(entries, length))
            }
        }
    }

    /// Destroys module flags metadata entries.
    pub fn dispose_module_flags_metadata(&self, entries: &ModuleFlagEntry) {
        unsafe {
            LLVMDisposeModuleFlagsMetadata(entries.0);
        }
    }
    /*
       pub fn get_flag_behavior(
           &self,
           entries: *mut LLVMModuleFlagEntry,
           index: u32,
       ) -> LLVMModuleFlagBehavior {
           unsafe { LLVMModuleFlagEntriesGetFlagBehavior(entries, index) }
       }

       pub fn get_key(&self, entries: *mut LLVMModuleFlagEntry, index: u32) -> Option<String> {
           unsafe {
               let mut len: usize = 0;
               let c_str = LLVMModuleFlagEntriesGetKey(entries, index, &mut len);
               if c_str.is_null() {
                   None
               } else {
                   Some(CStr::from_ptr(c_str).to_string_lossy().into_owned())
               }
           }
       }

       pub fn get_metadata(&self, entries: *mut LLVMModuleFlagEntry, index: u32) -> LLVMMetadataRef {
           unsafe { LLVMModuleFlagEntriesGetMetadata(entries, index) }
       }

       pub fn get_module_flag(&self, key: &str) -> LLVMMetadataRef {
           let c_key = CString::new(key).expect("CString::new failed");
           unsafe { LLVMGetModuleFlag(self.0, c_key.as_ptr(), c_key.to_bytes().len()) }
       }

       pub fn add_module_flag(
           &self,
           behavior: LLVMModuleFlagBehavior,
           key: &str,
           val: LLVMMetadataRef,
       ) {
           let c_key = CString::new(key).expect("CString::new failed");
           unsafe {
               LLVMAddModuleFlag(
                   self.0,
                   behavior,
                   c_key.as_ptr(),
                   c_key.to_bytes().len(),
                   val,
               );
           }
       }

       pub fn dump_module(&self) {
           unsafe {
               LLVMDumpModule(self.0);
           }
       }

       pub fn print_module_to_file(&self, filename: &str) -> Result<(), String> {
           let c_filename = CString::new(filename).expect("CString::new failed");
           let mut error_message: *mut c_char = ptr::null_mut();
           let result =
               unsafe { LLVMPrintModuleToFile(self.0, c_filename.as_ptr(), &mut error_message) };
           if result == 0 {
               Ok(())
           } else {
               let error = unsafe { CStr::from_ptr(error_message).to_string_lossy().into_owned() };
               unsafe { llvm::core::LLVMDisposeMessage(error_message) };
               Err(error)
           }
       }

       pub fn print_module_to_string(&self) -> Option<String> {
           unsafe {
               let c_str = LLVMPrintModuleToString(self.0);
               if c_str.is_null() {
                   None
               } else {
                   let result = CStr::from_ptr(c_str).to_string_lossy().into_owned();
                   llvm::core::LLVMDisposeMessage(c_str);
                   Some(result)
               }
           }
       }
    */

    /// Dump module to stdout
    pub fn dump_module(&self) {
        unsafe {
            LLVMDumpModule(self.0);
        }
    }
}

impl Deref for ModuleRef {
    type Target = LLVMModuleRef;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for ModuleRef {
    /// Dispose module
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeModule(self.0);
        }
    }
}

impl GetRef for ModuleRef {
    type RawRef = LLVMModuleRef;
    fn get_ref(&self) -> Self::RawRef {
        self.0
    }
}
