use super::ValueRef;
use crate::core::module::{MetadataRef, ModuleRef};
use crate::core::types::TypeRef;
use crate::{CStr, CString, CUint, GetRef};
use llvm_sys::core;
use llvm_sys::prelude::LLVMValueMetadataEntry;

/// Wrapper for `LLVMValueMetadataEntry`
#[derive(Debug)]
pub struct ValueMetadataEntry(LLVMValueMetadataEntry);

impl From<LLVMValueMetadataEntry> for ValueMetadataEntry {
    fn from(value: LLVMValueMetadataEntry) -> Self {
        ValueMetadataEntry(value)
    }
}

impl GetRef for ValueMetadataEntry {
    type RawRef = LLVMValueMetadataEntry;
    fn get_ref(&self) -> Self::RawRef {
        self.0
    }
}

impl ValueRef {
    /// Get the module that contains the global value.
    ///
    /// @see llvm::GlobalValue::getParent()
    pub fn get_global_parent(&self) -> ModuleRef {
        unsafe { ModuleRef::from(core::LLVMGetGlobalParent(self.0)) }
    }

    /// Determine if the global value is a declaration.
    ///
    /// @see llvm::GlobalValue::isDeclaration()
    pub fn is_declaration(&self) -> bool {
        unsafe { core::LLVMIsDeclaration(self.0) != 0 }
    }

    /// Get the linkage of the global value.
    ///
    /// @see llvm::GlobalValue::getLinkage()
    pub fn get_linkage(&self) -> LLVMLinkage {
        unsafe { core::LLVMGetLinkage(self.0) }
    }

    /// Set the linkage of the global value.
    ///
    /// @see llvm::GlobalValue::setLinkage()
    pub fn set_linkage(&self, linkage: LLVMLinkage) {
        unsafe { core::LLVMSetLinkage(self.0, linkage) }
    }

    /// Get the section of the global value.
    ///
    /// @see llvm::GlobalValue::getSection()
    pub fn get_section(&self) -> Option<String> {
        unsafe {
            let section = core::LLVMGetSection(self.0);
            if section.is_null() {
                None
            } else {
                Some(CStr::new(section).to_string())
            }
        }
    }

    /// Set the section of the global value.
    ///
    /// @see llvm::GlobalValue::setSection()
    pub fn set_section(&self, section: &str) {
        let c_section = CString::from(section);
        unsafe {
            core::LLVMSetSection(self.0, c_section.as_ptr());
        }
    }

    /// Get the visibility of the global value.
    ///
    /// @see llvm::GlobalValue::getVisibility()
    pub fn get_visibility(&self) -> Visibility {
        unsafe { core::LLVMGetVisibility(self.0) }
    }

    /// Set the visibility of the global value.
    ///
    /// @see llvm::GlobalValue::setVisibility()
    pub fn set_visibility(&self, visibility: Visibility) {
        unsafe {
            core::LLVMSetVisibility(self.0, visibility);
        }
    }

    /// Get the DLL storage class of a global value.
    ///
    /// @see llvm::GlobalValue::getDLLStorageClass()
    pub fn get_dll_storage_class(&self) -> DLLStorageClass {
        unsafe { core::LLVMGetDLLStorageClass(self.0) }
    }

    /// Set the DLL storage class of a global value.
    ///
    /// @see llvm::GlobalValue::setDLLStorageClass()
    pub fn set_dll_storage_class(&self, class: DLLStorageClass) {
        unsafe {
            core::LLVMSetDLLStorageClass(self.0, class);
        }
    }

    /// Get the unnamed address of a global value.
    ///
    /// @see llvm::GlobalValue::getUnnamedAddr()
    pub fn get_unnamed_address(&self) -> UnnamedAddr {
        unsafe { core::LLVMGetUnnamedAddress(self.0) }
    }

    /// Set the unnamed address of a global value.
    ///
    /// @see llvm::GlobalValue::setUnnamedAddr()
    pub fn set_unnamed_address(&self, unnamed_addr: UnnamedAddr) {
        unsafe {
            core::LLVMSetUnnamedAddress(self.0, unnamed_addr);
        }
    }

    /// Returns the "value type" of a global value. This differs from the formal
    /// type of a global value, which is always a pointer type.
    ///
    /// @see llvm::GlobalValue::getValueType()
    pub fn get_value_type(&self) -> TypeRef {
        unsafe { TypeRef::from(core::LLVMGlobalGetValueType(self.0)) }
    }

    /// Obtain the preferred alignment of the value.
    ///
    /// @see llvm::AllocaInst::getAlignment()
    /// @see llvm::LoadInst::getAlignment()
    /// @see llvm::StoreInst::getAlignment()
    /// @see llvm::AtomicRMWInst::setAlignment()
    /// @see llvm::AtomicCmpXchgInst::setAlignment()
    /// @see llvm::GlobalValue::getAlignment()
    pub fn get_alignment(&self) -> u32 {
        unsafe { core::LLVMGetAlignment(self.0) as u32 }
    }

    /// Set the preferred alignment of the value.
    ///
    /// @see llvm::AllocaInst::setAlignment()
    /// @see llvm::LoadInst::setAlignment()
    /// @see llvm::StoreInst::setAlignment()
    /// @see llvm::AtomicRMWInst::setAlignment()
    /// @see llvm::AtomicCmpXchgInst::setAlignment()
    /// @see llvm::GlobalValue::setAlignment()
    pub fn set_alignment(&self, bytes: u32) {
        unsafe {
            core::LLVMSetAlignment(self.0, bytes as c_uint);
        }
    }

    /// Sets a metadata attachment, erasing the existing metadata attachment if
    /// it already exists for the given kind.
    ///
    /// @see llvm::GlobalObject::setMetadata()
    pub fn global_set_metadata(&self, kind: u32, md: LLVMMetadataRef) {
        unsafe {
            core::LLVMGlobalSetMetadata(self.0, kind, md);
        }
    }

    /// Erases a metadata attachment of the given kind if it exists.
    ///
    /// @see llvm::GlobalObject::eraseMetadata()
    pub fn global_erase_metadata(&self, kind: u32) {
        unsafe {
            core::LLVMGlobalEraseMetadata(self.0, kind);
        }
    }

    /// Removes all metadata attachments from this value.
    ///
    /// @see llvm::GlobalObject::clearMetadata()
    pub fn global_clear_metadata(&self) {
        unsafe {
            core::LLVMGlobalClearMetadata(self.0);
        }
    }

    /// Destroys value metadata entries.
    ///
    /// ## Panics
    /// This function is purely informative and panics with a message about the call
    /// being unavailable. Since there are no cases in which it can be called in
    /// safe code. For raw access, if there is such a need, must be called
    /// `LLVMDisposeValueMetadataEntries` directly.
    pub fn dispose_value_metadata_entries(_entries: Vec<ValueMetadataEntry>) {
        unreachable!("LLVMDisposeValueMetadataEntries is unsafe adn restricted to operated to operate directly for safe code");
    }

    /// Retrieves an array of metadata entries representing the metadata attached to
    /// this value. The caller is responsible for freeing this array by calling
    /// `LLVMDisposeValueMetadataEntries`.
    ///
    /// @see llvm::GlobalObject::getAllMetadata()
    pub fn global_copy_all_metadata(&self) -> Vec<ValueMetadataEntry> {
        let mut num_entries: usize = 0;
        let entries_ptr = unsafe { core::LLVMGlobalCopyAllMetadata(self.0, &mut num_entries) };

        if entries_ptr.is_null() {
            return Vec::new();
        }
        let entries_slice = unsafe { std::slice::from_raw_parts(entries_ptr, num_entries) };

        let entries = entries_slice
            .iter()
            .map(|&entry| ValueMetadataEntry::from(entry))
            .collect::<Vec<_>>();

        // Free the memory allocated by `LLVMGlobalCopyAllMetadata`
        unsafe {
            core::LLVMDisposeValueMetadataEntries(entries_ptr);
        }

        entries
    }

    /// Returns the kind of a value metadata entry at a specific index.
    pub fn get_kind(&self, index: u32) -> u32 {
        unsafe { core::LLVMValueMetadataEntriesGetKind(self.0, index as c_uint) as u32 }
    }

    /// Returns the underlying metadata node of a value metadata entry at a specific index.
    pub fn get_metadata(&self, index: u32) -> MetadataRef {
        unsafe {
            MetadataRef::from(core::LLVMValueMetadataEntriesGetMetadata(
                self.0,
                *CUint::from(index),
            ))
        }
    }
}
