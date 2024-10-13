use super::ValueRef;
use crate::core::module::{MetadataRef, ModuleRef};
use crate::core::types::TypeRef;
use crate::core::{DLLStorageClass, Linkage, UnnamedAddr, Visibility};
use crate::{CStr, CString, CUint, GetRef};
use llvm_sys::core;
use llvm_sys::prelude::LLVMValueMetadataEntry;

/// Wrapper for `LLVMValueMetadataEntry`
#[derive(Debug)]
pub struct ValueMetadataEntry(LLVMValueMetadataEntry);

impl From<LLVMValueMetadataEntry> for ValueMetadataEntry {
    fn from(value: LLVMValueMetadataEntry) -> Self {
        Self(value)
    }
}

impl GetRef for ValueMetadataEntry {
    type RawRef = LLVMValueMetadataEntry;
    fn get_ref(&self) -> Self::RawRef {
        self.0
    }
}

/// Get the module that contains the global value.
///
/// ## Details
///
/// Retrieves the parent module of a global value.
///
/// This function wraps the `LLVMGetGlobalParent` function from the LLVM core library. It returns the `ModuleRef`
/// representing the parent module in which the global value represented by `ValueRef` is defined. The parent module
/// contains all the global values (such as functions and global variables) that are part of a given LLVM module.
///
/// # Returns
///
/// Returns a `ModuleRef` representing the parent module of the global value.
#[must_use]
pub fn get_global_parent(val: &ValueRef) -> ModuleRef {
    unsafe { ModuleRef::from(core::LLVMGetGlobalParent(val.get_ref())) }
}

/// Determine if the global value is a declaration.
///
/// ## Details
///
/// Checks if the global value is a declaration.
///
/// This function wraps the `LLVMIsDeclaration` function from the LLVM core library. It determines whether
/// the global value represented by `ValueRef` is merely a declaration (i.e., it declares the existence of a symbol
/// but does not define it). Declarations are often used to reference functions or variables that are defined
/// in another module or later in the same module.
///
/// # Returns
///
/// Returns `true` if the global value is a declaration, otherwise returns `false`.
#[must_use]
pub fn is_declaration(val: &ValueRef) -> bool {
    unsafe { core::LLVMIsDeclaration(val.get_ref()) != 0 }
}

/// Get the linkage of the global value.
///
/// ## Details
///
/// Sets the linkage type for a global value.
///
/// This function wraps the `LLVMSetLinkage` function from the LLVM core library. It configures the linkage type
/// for the global value represented by `ValueRef`. The linkage type determines how the symbol is treated during the
/// linking process, particularly in relation to how it can be combined with other symbols and whether it is visible
/// outside of the module.
///
/// # Parameters
///
/// - `linkage`: A `Linkage` enum value that specifies the linkage type for the global value. Common linkage types include:
///   - `ExternalLinkage`: The symbol is visible to other modules and can be linked against.
///   - `InternalLinkage`: The symbol is only visible within the current module.
///   - `PrivateLinkage`: The symbol is local to the file and not exposed to other modules.
///   - `LinkOnceODRLinkage`: Ensures that the symbol is defined only once across all modules, complying with the One Definition Rule (ODR).
#[must_use]
pub fn get_linkage(val: &ValueRef) -> Linkage {
    unsafe { crate::core::Linkage::from(core::LLVMGetLinkage(val.get_ref())) }
}

/// Set the linkage of the global value.
///
/// ## Details
///
///
pub fn set_linkage(val: &ValueRef, linkage: Linkage) {
    unsafe { core::LLVMSetLinkage(val.get_ref(), linkage.into()) }
}

/// Get the section of the global value.
///
/// ## Details
///
/// Retrieves the section name in which a global value is placed.
///
/// This function wraps the `LLVMGetSection` function from the LLVM core library. It returns the name of the section
/// where the global value represented by `ValueRef` is placed. Sections are used to organize global values in the object file,
/// allowing the linker and loader to place related values together or handle them in a specific way.
///
/// # Returns
///
/// Returns an `Option<String>`:
/// - `Some(String)` containing the name of the section if the global value is placed in a specific section.
/// - `None` if the global value is not associated with any section.
#[must_use]
pub fn get_section(val: &ValueRef) -> Option<String> {
    unsafe {
        let section = core::LLVMGetSection(val.get_ref());
        if section.is_null() {
            None
        } else {
            Some(CStr::new(section).to_string())
        }
    }
}

/// Set the section of the global value.
///
/// ## Details
///
/// Sets the section in which a global value should be placed.
///
/// This function wraps the `LLVMSetSection` function from the LLVM core library. It specifies the section name
/// for the global value represented by `ValueRef`. Sections are used to organize global values in the object file, allowing
/// the linker and loader to place related values together or handle them in a specific way.
///
/// # Parameters
///
/// - `section`: A string slice (`&str`) representing the name of the section where the global value should be placed.
pub fn set_section(val: &ValueRef, section: &str) {
    let c_section = CString::from(section);
    unsafe {
        core::LLVMSetSection(val.get_ref(), c_section.as_ptr());
    }
}

/// Get the visibility of the global value.
///
/// ## Details
///
/// Retrieves the visibility attribute of a global value.
///
/// This function wraps the `LLVMGetVisibility` function from the LLVM core library. It returns the visibility
/// attribute of the global value represented by `ValueRef`. The visibility attribute determines how the symbol is
/// treated by the linker and whether it can be seen by other modules or shared libraries.
///
/// # Returns
///
/// Returns a `Visibility` enum value representing the visibility attribute of the global value:
/// - `DefaultVisibility`: The symbol is visible to other modules.
/// - `HiddenVisibility`: The symbol is not visible to other modules or shared libraries.
/// - `ProtectedVisibility`: The symbol is visible to other modules but cannot be overridden.
#[must_use]
pub fn get_visibility(val: &ValueRef) -> Visibility {
    unsafe { crate::core::Visibility::from(core::LLVMGetVisibility(val.get_ref())) }
}

/// Set the visibility of the global value.
///
/// ## Details
///
/// Sets the visibility attribute for a global value.
///
/// This function wraps the `LLVMSetVisibility` function from the LLVM core library. It configures the visibility
/// attribute for the global value represented by `ValueRef`. The visibility attribute determines how the symbol is
/// treated by the linker and whether it can be seen by other modules or shared libraries.
///
/// # Parameters
///
/// - `visibility`: A `Visibility` enum value that specifies the visibility of the global value:
///   - `DefaultVisibility`: The symbol is visible to other modules.
///   - `HiddenVisibility`: The symbol is not visible to other modules or shared libraries.
///   - `ProtectedVisibility`: The symbol is visible to other modules but cannot be overridden.
pub fn set_visibility(val: &ValueRef, visibility: Visibility) {
    unsafe {
        core::LLVMSetVisibility(val.get_ref(), visibility.into());
    }
}

/// Get the DLL storage class of a global value.
///
/// ## Details
///
/// Retrieves the DLL storage class of a global value.
///
/// This function wraps the `LLVMGetDLLStorageClass` function from the LLVM core library. It returns the
/// `DLLStorageClass` of the global value represented by `ValueRef`. The DLL storage class determines how the global
/// value is treated in relation to dynamic link libraries (DLLs) on platforms like Windows.
///
/// # Returns
///
/// Returns a `DLLStorageClass` enum value representing the DLL storage class of the global value:
/// - `DefaultStorageClass`: The symbol is treated as a normal global symbol, not specifically marked for import or export from a DLL.
/// - `DLLImportStorageClass`: The symbol is imported from a DLL.
/// - `DLLExportStorageClass`: The symbol is exported to a DLL.
#[must_use]
pub fn get_dll_storage_class(val: &ValueRef) -> DLLStorageClass {
    unsafe { DLLStorageClass::from(core::LLVMGetDLLStorageClass(val.get_ref())) }
}

/// Set the DLL storage class of a global value.
///
/// ## Details
///
/// Sets the DLL storage class for a global value.
///
/// This function wraps the `LLVMSetDLLStorageClass` function from the LLVM core library. It configures the DLL storage class
/// for the global value represented by `ValueRef`. The `DLLStorageClass` attribute determines how the global value is treated
/// in relation to dynamic link libraries (DLLs) on platforms like Windows.
///
/// # Parameters
///
/// - `class`: A `DLLStorageClass` enum value that specifies the DLL storage class for the global value.
///   - `DefaultStorageClass`: The symbol is treated as a normal global symbol, not specifically marked for import or export from a DLL.
///   - `DLLImportStorageClass`: The symbol is imported from a DLL.
///   - `DLLExportStorageClass`: The symbol is exported to a DLL.
pub fn set_dll_storage_class(val: &ValueRef, class: DLLStorageClass) {
    unsafe {
        core::LLVMSetDLLStorageClass(val.get_ref(), class.into());
    }
}

/// Get the unnamed address of a global value.
///
/// ## Details
///
/// Retrieves the unnamed address attribute of a global value.
///
/// This function wraps the `LLVMGetUnnamedAddress` function from the LLVM core library. It returns the
/// `UnnamedAddr` attribute of the global value represented by `ValueRef`. This attribute specifies whether the address
/// of the global value is significant, which can influence certain optimizations in LLVM.
///
/// # Returns
///
/// Returns an `UnnamedAddr` enum value representing the unnamed address attribute of the global value:
/// - `NoUnnamedAddr`: The address of the global value is significant and must be unique.
/// - `LocalUnnamedAddr`: The address is not significant within the module, allowing certain optimizations.
/// - `GlobalUnnamedAddr`: The address is not significant across the entire program, enabling more aggressive optimizations.
#[must_use]
pub fn get_unnamed_address(val: &ValueRef) -> UnnamedAddr {
    unsafe { UnnamedAddr::from(core::LLVMGetUnnamedAddress(val.get_ref())) }
}

/// Set the unnamed address of a global value.
///
/// ## Details
///
/// Sets the unnamed address attribute for a global value.
///
/// This function wraps the `LLVMSetUnnamedAddress` function from the LLVM core library. It configures the
/// unnamed address attribute for the global value represented by `ValueRef`. The `UnnamedAddr` attribute specifies
/// whether the address of the global value is significant, which can influence certain optimizations in LLVM.
///
/// # Parameters
///
/// - `unnamed_addr`: An `UnnamedAddr` enum value that specifies the unnamed address attribute for the global value.
///   - `NoUnnamedAddr`: The address of the global value is significant and must be unique.
///   - `LocalUnnamedAddr`: The address is not significant within the module, allowing certain optimizations.
///   - `GlobalUnnamedAddr`: The address is not significant across the entire program, enabling more aggressive optimizations.
pub fn set_unnamed_address(val: &ValueRef, unnamed_addr: UnnamedAddr) {
    unsafe {
        core::LLVMSetUnnamedAddress(val.get_ref(), unnamed_addr.into());
    }
}

/// Returns the "value type" of a global value. This differs from the formal
/// type of a global value, which is always a pointer type.
///
/// ## Details
///
/// Retrieves the type of the global value.
///
/// This function wraps the `LLVMGlobalGetValueType` function from the LLVM core library. It returns the
/// `TypeRef` representing the type of the global value associated with `ValueRef`. Knowing the type of the global value
/// is essential for understanding what kind of data it holds or operates on, as well as for performing type-specific
/// operations or optimizations.
///
/// # Returns
///
/// Returns a `TypeRef` representing the type of the global value.
#[must_use]
pub fn get_value_type(val: &ValueRef) -> TypeRef {
    unsafe { TypeRef::from(core::LLVMGlobalGetValueType(val.get_ref())) }
}

/// Obtain the preferred alignment of the value.
///
/// ## Details
///
/// Retrieves the alignment of a global value in bytes.
///
/// This function wraps the `LLVMGetAlignment` function from the LLVM core library. It returns the alignment
/// requirement of the global value represented by `ValueRef`, in terms of the number of bytes. Knowing the alignment
/// can be useful for understanding how the global value is laid out in memory and ensuring that it meets the
/// requirements of the target architecture.
///
/// # Returns
///
/// Returns a `u32` representing the alignment of the global value in bytes.
#[must_use]
pub fn get_alignment(val: &ValueRef) -> u32 {
    unsafe { core::LLVMGetAlignment(val.get_ref()) as u32 }
}

/// Set the preferred alignment of the value.
///
/// ## Details
///
/// Sets the alignment for a global value in bytes.
///
/// This function wraps the `LLVMSetAlignment` function from the LLVM core library. It specifies the alignment
/// requirement for the global value represented by `ValueRef`, in terms of the number of bytes. Proper alignment can be
/// important for performance, particularly in low-level systems programming, where misaligned accesses can cause
/// performance penalties or even hardware exceptions.
///
/// # Parameters
///
/// - `bytes`: A `u32` value representing the desired alignment in bytes. This value must be a power of two.
pub fn set_alignment(val: &ValueRef, bytes: u32) {
    unsafe {
        core::LLVMSetAlignment(val.get_ref(), *CUint::from(bytes));
    }
}

/// Sets a metadata attachment, erasing the existing metadata attachment if
/// it already exists for the given kind.
///
/// ## Details
///
/// Sets metadata of a specific kind for a global value.
///
/// This function wraps the `LLVMGlobalSetMetadata` function from the LLVM core library. It attaches metadata of the
/// specified kind to the global value represented by `ValueRef`. If metadata of this kind already exists, it will be replaced
/// with the new metadata provided. Metadata in LLVM is used to attach additional information to global values, such as
/// functions or variables, which can be useful for debugging, optimization, or other purposes.
///
/// # Parameters
///
/// - `kind`: A `u32` representing the kind of metadata to be set. The kind ID specifies the category or type of the metadata.
/// - `md`: A `MetadataRef` representing the metadata to be attached to the global value.
pub fn global_set_metadata(val: &ValueRef, kind: u32, md: &MetadataRef) {
    unsafe {
        core::LLVMGlobalSetMetadata(val.get_ref(), kind, md.get_ref());
    }
}

/// Erases a metadata attachment of the given kind if it exists.
///
/// ## Details
///
/// Erases metadata of a specific kind from a global value.
///
/// This function wraps the `LLVMGlobalEraseMetadata` function from the LLVM core library. It removes the metadata
/// entry of the specified kind associated with the global value represented by `ValueRef`. If the global value has multiple
/// metadata entries, only the entry matching the specified kind will be erased, leaving other metadata intact.
///
/// # Parameters
///
/// - `kind`: A `u32` representing the kind of metadata to be erased. The kind ID specifies the category or type of the metadata.
pub fn global_erase_metadata(val: &ValueRef, kind: u32) {
    unsafe {
        core::LLVMGlobalEraseMetadata(val.get_ref(), *CUint::from(kind));
    }
}

/// Removes all metadata attachments from this value.
///
/// ## Details
///
/// Clears all metadata attached to a global value.
///
/// This function wraps the `LLVMGlobalClearMetadata` function from the LLVM core library. It removes all metadata
/// entries associated with the global value represented by `ValueRef`. This operation effectively detaches any metadata
/// from the global value, which might be useful in scenarios where the metadata is no longer needed or should be reset.
pub fn global_clear_metadata(val: &ValueRef) {
    unsafe {
        core::LLVMGlobalClearMetadata(val.get_ref());
    }
}

/// Destroys value metadata entries.
///
/// ## Panics
/// This function is purely informative and panics with a message about the call
/// being unavailable. Since there are no cases in which it can be called in
/// safe code. For raw access, if there is such a need, must be called
/// `LLVMDisposeValueMetadataEntries` directly.
pub fn dispose_value_metadata_entries(_entries: &[ValueMetadataEntry]) {
    unreachable!("LLVMDisposeValueMetadataEntries is unsafe adn restricted to operated to operate directly for safe code");
}

/// Retrieves an array of metadata entries representing the metadata attached to  this value.
///
/// ## Details
///
/// Copies all metadata attached to a global value and returns it as a vector of `ValueMetadataEntry`.
///
/// This function wraps the `LLVMGlobalCopyAllMetadata` function from the LLVM core library. It retrieves all metadata
/// entries associated with the global value represented by `ValueRef` and returns them as a vector of `ValueMetadataEntry`.
/// Metadata in LLVM is used to attach additional information to various constructs, such as functions or global variables,
/// which can be useful for debugging, optimization, or other purposes.
///
/// After copying the metadata entries, the function ensures that any allocated memory for the metadata entries is correctly
/// freed by calling ``LLVMDisposeValueMetadataEntries``.
///
/// # Returns
///
/// Returns a `Vec<ValueMetadataEntry>` containing all metadata entries attached to the global value. If no metadata is
/// attached, an empty vector is returned.
#[must_use]
pub fn global_copy_all_metadata(val: &ValueRef) -> Vec<ValueMetadataEntry> {
    let mut num_entries: usize = 0;
    let entries_ptr = unsafe { core::LLVMGlobalCopyAllMetadata(val.get_ref(), &mut num_entries) };

    if entries_ptr.is_null() {
        return Vec::new();
    }
    let entries_slice = unsafe { std::slice::from_raw_parts(entries_ptr, num_entries) };

    let entries = entries_slice
        .iter()
        .map(|&entry| ValueMetadataEntry::from(entry))
        .collect::<Vec<_>>();

    // Free allocated memory
    unsafe {
        core::LLVMDisposeValueMetadataEntries(entries_ptr);
    }

    entries
}

/// Returns the kind of a value metadata entry at a specific index.
///
/// ## Details
///
/// Retrieves the metadata kind ID for a specific entry in a list of value metadata entries.
///
/// This function wraps the `LLVMValueMetadataEntriesGetKind` function from the LLVM core library. It retrieves
/// the kind ID of the metadata entry at the specified index within the provided vector of `ValueMetadataEntry`.
/// Metadata kinds in LLVM are used to categorize the type of metadata, allowing different kinds of information
/// to be attached to values.
///
/// # Parameters
///
/// - `value_metadata_entries`: A vector of `ValueMetadataEntry` from which the metadata kind ID will be retrieved.
/// - `index`: The index of the metadata entry within the vector for which the kind ID is requested.
///
/// # Returns
///
/// Returns a `u32` representing the metadata kind ID for the specified entry.
///
/// # Panics
///
/// The function may panic if the provided index is out of bounds for the vector, depending on how the underlying
/// LLVM function handles invalid indices.
#[must_use]
pub fn value_metadata_entries_get_kind(
    value_metadata_entries: &[ValueMetadataEntry],
    index: u32,
) -> u32 {
    let entries_ptr = crate::to_mut_ptr!(value_metadata_entries);
    unsafe { core::LLVMValueMetadataEntriesGetKind(entries_ptr, *CUint::from(index)) }
}

/// Returns the underlying metadata node of a value metadata entry at a specific index.
///
/// ## Details
///
/// Retrieves the metadata reference for a specific entry in a list of value metadata entries.
///
/// This function wraps the `LLVMValueMetadataEntriesGetMetadata` function from the LLVM core library. It retrieves
/// the `MetadataRef` associated with the metadata entry at the specified index within the provided vector of `ValueMetadataEntry`.
/// This allows you to access the metadata attached to a global value or other LLVM constructs.
///
/// # Parameters
///
/// - `value_metadata_entries`: A vector of `ValueMetadataEntry` from which the metadata reference will be retrieved.
/// - `index`: The index of the metadata entry within the vector for which the metadata reference is requested.
///
/// # Returns
///
/// Returns a `MetadataRef` representing the metadata associated with the specified entry.
///
/// # Panics
///
/// The function may panic if the provided index is out of bounds for the vector, depending on how the underlying
/// LLVM function handles invalid indices.
#[must_use]
pub fn value_metadata_entries_get_metadata(
    value_metadata_entries: &[ValueMetadataEntry],
    index: u32,
) -> MetadataRef {
    let entries_ptr = crate::to_mut_ptr!(value_metadata_entries);
    unsafe {
        MetadataRef::from(core::LLVMValueMetadataEntriesGetMetadata(
            entries_ptr,
            *CUint::from(index),
        ))
    }
}
