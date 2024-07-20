use std::ops::Deref;

use llvm_sys::core::{
    LLVMAddFunction, LLVMAddModuleFlag, LLVMAddNamedMetadataOperand, LLVMAppendModuleInlineAsm,
    LLVMCloneModule, LLVMCopyModuleFlagsMetadata, LLVMDisposeModule,
    LLVMDisposeModuleFlagsMetadata, LLVMDumpModule, LLVMGetDataLayoutStr,
    LLVMGetFirstNamedMetadata, LLVMGetInlineAsm, LLVMGetLastNamedMetadata, LLVMGetModuleContext,
    LLVMGetModuleFlag, LLVMGetModuleIdentifier, LLVMGetModuleInlineAsm, LLVMGetNamedMetadata,
    LLVMGetNamedMetadataName, LLVMGetNamedMetadataNumOperands, LLVMGetNamedMetadataOperands,
    LLVMGetNextNamedMetadata, LLVMGetOrInsertNamedMetadata, LLVMGetPreviousNamedMetadata,
    LLVMGetSourceFileName, LLVMGetTarget, LLVMModuleCreateWithName,
    LLVMModuleCreateWithNameInContext, LLVMModuleFlagEntriesGetFlagBehavior,
    LLVMModuleFlagEntriesGetKey, LLVMModuleFlagEntriesGetMetadata, LLVMPrintModuleToFile,
    LLVMPrintModuleToString, LLVMSetDataLayout, LLVMSetModuleIdentifier, LLVMSetModuleInlineAsm2,
    LLVMSetSourceFileName, LLVMSetTarget,
};
use llvm_sys::prelude::{
    LLVMMetadataRef, LLVMModuleFlagEntry, LLVMModuleRef, LLVMNamedMDNodeRef, LLVMValueRef,
};
use llvm_sys::{LLVMInlineAsmDialect, LLVMModuleFlagBehavior};

use crate::context::ContextRef;
use crate::types::TypeRef;
use crate::value::ValueRef;
use crate::{CInt, CStr, CString, GetRef, SizeT};

/// Inline Asm Dialect
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InlineAsmDialect {
    InlineAsmDialectATT,
    InlineAsmDialectIntel,
}

impl From<InlineAsmDialect> for LLVMInlineAsmDialect {
    fn from(value: InlineAsmDialect) -> Self {
        match value {
            InlineAsmDialect::InlineAsmDialectATT => Self::LLVMInlineAsmDialectATT,
            InlineAsmDialect::InlineAsmDialectIntel => Self::LLVMInlineAsmDialectIntel,
        }
    }
}

impl From<LLVMInlineAsmDialect> for InlineAsmDialect {
    fn from(value: LLVMInlineAsmDialect) -> Self {
        match value {
            LLVMInlineAsmDialect::LLVMInlineAsmDialectATT => Self::InlineAsmDialectATT,
            LLVMInlineAsmDialect::LLVMInlineAsmDialectIntel => Self::InlineAsmDialectIntel,
        }
    }
}

/// Named Metadata Node.
/// Used to associate metadata with a module in a way that is identifiable by a name. These nodes
/// can be used for various purposes, such as attaching additional information to a module that can
/// be used by the compiler or other tools processing the LLVM IR.
#[derive(Debug)]
pub struct NamedMetadataNodeRef(LLVMNamedMDNodeRef);

impl From<LLVMNamedMDNodeRef> for NamedMetadataNodeRef {
    fn from(value: LLVMNamedMDNodeRef) -> Self {
        Self(value)
    }
}

impl NamedMetadataNodeRef {
    /// Advance a `NamedMetaDataNode` iterator to the next `NamedMetaDataNode`.
    ///
    /// Returns NULL if the iterator was already at the end and there are no more
    /// named metadata nodes.
    #[must_use]
    pub fn get_next(&self) -> Option<Self> {
        let next_md = unsafe { LLVMGetNextNamedMetadata(self.0) };
        if next_md.is_null() {
            None
        } else {
            Some(next_md.into())
        }
    }

    /// Decrement a `NamedMetaDataNode` iterator to the previous `NamedMetaDataNode`.
    ///
    /// Returns NULL if the iterator was already at the beginning and there are
    /// no previously named metadata nodes.
    #[must_use]
    pub fn get_previous(&self) -> Option<Self> {
        let prev_md = unsafe { LLVMGetPreviousNamedMetadata(self.0) };
        if prev_md.is_null() {
            None
        } else {
            Some(prev_md.into())
        }
    }

    /// Retrieve the name of a `NamedMetadataNode`.
    #[must_use]
    pub fn get_name(&self) -> Option<String> {
        let mut length = SizeT::from(0_usize);
        unsafe {
            let c_str = LLVMGetNamedMetadataName(self.0, &mut *length);
            if c_str.is_null() {
                None
            } else {
                Some(CStr::new(c_str).to_string())
            }
        }
    }
}

#[derive(Debug)]
pub struct MetadataRef(LLVMMetadataRef);

/// Represents flags that describe information about the module for use by
/// an external entity e.g. the dynamic linker.
#[allow(dead_code)]
#[derive(Debug)]
pub struct ModuleFlagEntry(*mut LLVMModuleFlagEntry, SizeT);

impl ModuleFlagEntry {
    /// Get the number of module flag metadata entries.
    #[must_use]
    pub fn get_count(&self) -> usize {
        *self.1
    }

    /// Destroys module flags metadata entries.
    pub fn dispose_module_flags_metadata(&self) {
        unsafe {
            LLVMDisposeModuleFlagsMetadata(self.0);
        }
    }

    /// Returns the flag behavior for a module flag entry at a specific index.
    #[must_use]
    pub fn get_flag_behavior(&self, index: u32) -> ModuleFlagBehavior {
        let behavior = unsafe { LLVMModuleFlagEntriesGetFlagBehavior(self.0, index) };
        behavior.into()
    }

    /// Returns the key for a module flag entry at a specific index.
    #[must_use]
    pub fn get_key(&self, index: u32) -> Option<String> {
        unsafe {
            let mut length: usize = 0;
            let c_str = LLVMModuleFlagEntriesGetKey(self.0, index, &mut length);
            if c_str.is_null() {
                None
            } else {
                Some(CStr::new(c_str).to_string())
            }
        }
    }

    /// Returns the metadata for a module flag entry at a specific index.
    #[must_use]
    pub fn get_metadata(&self, index: u32) -> MetadataRef {
        let metadata = unsafe { LLVMModuleFlagEntriesGetMetadata(self.0, index) };
        MetadataRef(metadata)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModuleFlagBehavior {
    /// Emits an error if two values disagree, otherwise the resulting value is that of the operands.
    ModuleFlagBehaviorError,
    /// Emits a warning if two values disagree. The result value will be the operand for the flag
    /// // from the first module being linked.
    ModuleFlagBehaviorWarning,
    /// Adds a requirement that another module flag be present and have a specified value after
    /// // linking is performed. The value must be a metadata pair, where the first element of
    /// the pair is the ID of the module flag to be restricted, and the second element of the pair
    /// is the value the module flag should be restricted to. This behavior can be used to restrict
    /// the allowable results (via triggering of an error) of linking IDs with the **Override** behavior.
    ModuleFlagBehaviorRequire,
    /// Uses the specified value, regardless of the behavior or value of the other module. If both
    /// modules specify **Override**, but the values differ, an error will be emitted.
    ModuleFlagBehaviorOverride,
    /// Appends the two values, which are required to be metadata nodes.
    ModuleFlagBehaviorAppend,
    /// Appends the two values, which are required to be metadata nodes. However, duplicate entries
    /// in the second list are dropped during the append operation.
    ModuleFlagBehaviorAppendUnique,
}

impl From<LLVMModuleFlagBehavior> for ModuleFlagBehavior {
    fn from(value: LLVMModuleFlagBehavior) -> Self {
        match value {
            LLVMModuleFlagBehavior::LLVMModuleFlagBehaviorError => Self::ModuleFlagBehaviorError,
            LLVMModuleFlagBehavior::LLVMModuleFlagBehaviorWarning => {
                Self::ModuleFlagBehaviorWarning
            }
            LLVMModuleFlagBehavior::LLVMModuleFlagBehaviorRequire => {
                Self::ModuleFlagBehaviorRequire
            }
            LLVMModuleFlagBehavior::LLVMModuleFlagBehaviorOverride => {
                Self::ModuleFlagBehaviorOverride
            }
            LLVMModuleFlagBehavior::LLVMModuleFlagBehaviorAppend => Self::ModuleFlagBehaviorAppend,
            LLVMModuleFlagBehavior::LLVMModuleFlagBehaviorAppendUnique => {
                Self::ModuleFlagBehaviorAppendUnique
            }
        }
    }
}

impl From<ModuleFlagBehavior> for LLVMModuleFlagBehavior {
    fn from(value: ModuleFlagBehavior) -> Self {
        match value {
            ModuleFlagBehavior::ModuleFlagBehaviorError => Self::LLVMModuleFlagBehaviorError,
            ModuleFlagBehavior::ModuleFlagBehaviorWarning => Self::LLVMModuleFlagBehaviorWarning,
            ModuleFlagBehavior::ModuleFlagBehaviorRequire => Self::LLVMModuleFlagBehaviorRequire,
            ModuleFlagBehavior::ModuleFlagBehaviorOverride => Self::LLVMModuleFlagBehaviorOverride,
            ModuleFlagBehavior::ModuleFlagBehaviorAppend => Self::LLVMModuleFlagBehaviorAppend,
            ModuleFlagBehavior::ModuleFlagBehaviorAppendUnique => {
                Self::LLVMModuleFlagBehaviorAppendUnique
            }
        }
    }
}

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

    /// Add a module-level flag to the module-level flags metadata if it doesn't already exist.
    #[must_use]
    pub fn get_module_flag(&self, key: &str) -> MetadataRef {
        let c_key = CString::from(key);
        let metadata =
            unsafe { LLVMGetModuleFlag(self.0, c_key.as_ptr(), *SizeT(c_key.to_bytes().len())) };
        MetadataRef(metadata)
    }

    pub fn add_module_flag(&self, behavior: &ModuleFlagBehavior, key: &str, val: &MetadataRef) {
        let c_key = CString::from(key);
        unsafe {
            LLVMAddModuleFlag(
                self.0,
                (*behavior).into(),
                c_key.as_ptr(),
                c_key.to_bytes().len(),
                val.0,
            );
        }
    }

    /// Dump module to stdout
    pub fn dump_module(&self) {
        unsafe {
            LLVMDumpModule(self.0);
        }
    }

    /// Print a representation of a module to a file. The `ErrorMessage` needs to be
    ///  disposed with `core::dispose_message`. Returns 0 on success, 1 otherwise.
    ///
    /// # Errors
    /// Return error as `String` if print module fails
    pub fn print_module_to_file(&self, filename: &str) -> Result<(), String> {
        let c_filename = CString::from(filename);
        let mut error_message: *mut std::ffi::c_char = std::ptr::null_mut();
        let result =
            unsafe { LLVMPrintModuleToFile(self.0, c_filename.as_ptr(), &mut error_message) };
        if result == 0 {
            Ok(())
        } else {
            unsafe {
                let error = CStr::new(error_message).to_string();
                crate::core::dispose_message(error_message);
                Err(error)
            }
        }
    }

    /// Return a string representation of the module.
    #[must_use]
    pub fn print_module_to_string(&self) -> Option<String> {
        unsafe {
            let c_str = LLVMPrintModuleToString(self.0);
            if c_str.is_null() {
                None
            } else {
                let result = CStr::new(c_str).to_string();
                crate::core::dispose_message(c_str);
                Some(result)
            }
        }
    }

    /// Get inline assembly for a module.
    #[must_use]
    pub fn get_module_inline_asm(&self) -> Option<String> {
        unsafe {
            let mut len = SizeT::from(0_usize);
            let c_str = LLVMGetModuleInlineAsm(self.0, &mut *len);
            if c_str.is_null() {
                None
            } else {
                Some(CStr::new(c_str).to_string())
            }
        }
    }

    /// Set inline assembly for a module.
    pub fn set_module_inline_asm(&self, asm: &str) {
        let c_asm = CString::from(asm);
        unsafe {
            LLVMSetModuleInlineAsm2(self.0, c_asm.as_ptr(), *SizeT(c_asm.to_bytes().len()));
        }
    }

    /// Append inline assembly to a module.
    pub fn append_module_inline_asm(&self, asm: &str) {
        let c_asm = CString::from(asm);
        unsafe {
            LLVMAppendModuleInlineAsm(self.0, c_asm.as_ptr(), *SizeT(c_asm.to_bytes().len()));
        }
    }

    /// Set add function value based on Function type
    #[must_use]
    pub fn add_function(&self, fn_name: &str, fn_type: &TypeRef) -> ValueRef {
        unsafe {
            let c_name = CString::from(fn_name);
            ValueRef::from(LLVMAddFunction(self.0, c_name.as_ptr(), **fn_type))
        }
    }

    /// Obtain the context to which this module is associated.
    #[must_use]
    pub fn get_module_context(&self) -> ContextRef {
        ContextRef::from(unsafe { LLVMGetModuleContext(self.0) })
    }

    /// Obtain an iterator to the first `NamedMDNode` in a `Module`.
    #[must_use]
    pub fn get_first_named_metadata(&self) -> Option<NamedMetadataNodeRef> {
        let md = unsafe { LLVMGetFirstNamedMetadata(self.0) };
        if md.is_null() {
            None
        } else {
            Some(md.into())
        }
    }

    /// Obtain an iterator to the last `NamedMDNode` in a Module.
    #[must_use]
    pub fn get_last_named_metadata(&self) -> Option<NamedMetadataNodeRef> {
        let md = unsafe { LLVMGetLastNamedMetadata(self.0) };
        if md.is_null() {
            None
        } else {
            Some(md.into())
        }
    }

    ///  Retrieve a `NamedMetadataNode` with the given name, returning `None` if no such node exists.
    #[must_use]
    pub fn get_named_metadata(&self, name: &str) -> Option<NamedMetadataNodeRef> {
        let c_name = CString::from(name);
        let md = unsafe {
            LLVMGetNamedMetadata(self.0, c_name.as_ptr(), *SizeT(c_name.as_bytes().len()))
        };
        if md.is_null() {
            None
        } else {
            Some(md.into())
        }
    }

    /// Retrieve a `NamedMetadataNode` with the given name, creating a new node if no such node exists.
    #[must_use]
    pub fn get_or_insert_named_metadata(&self, name: &str) -> NamedMetadataNodeRef {
        let c_name = CString::from(name);
        let md = unsafe {
            LLVMGetOrInsertNamedMetadata(self.0, c_name.as_ptr(), *SizeT(c_name.as_bytes().len()))
        };
        md.into()
    }

    /// Obtain the number of operands for named metadata in a module.
    #[must_use]
    pub fn get_named_metadata_num_operands(&self, name: &str) -> u32 {
        let c_name = CString::from(name);
        unsafe { LLVMGetNamedMetadataNumOperands(self.0, c_name.as_ptr()) }
    }

    /// Obtain the named metadata operands for a module.
    ///
    /// The passed `ValueRef` pointer should refer to an array of
    /// `ValueRef` at least `get_names_metadata_operands` long. This
    /// array will be populated with the `ValueRef` instances. Each
    /// instance corresponds to a Metadata Node.
    #[must_use]
    pub fn get_named_metadata_operands(&self, name: &str) -> Vec<ValueRef> {
        let c_name = CString::from(name);
        let num_operands = self.get_named_metadata_num_operands(name);
        let mut raw_operands: Vec<LLVMValueRef> = Vec::with_capacity(num_operands as usize);
        unsafe {
            LLVMGetNamedMetadataOperands(self.0, c_name.as_ptr(), raw_operands.as_mut_ptr());
            raw_operands.set_len(num_operands as usize);
        }
        raw_operands.into_iter().map(ValueRef::from).collect()
    }

    /// Add an operand to named metadata.
    pub fn add_named_metadata_operand(&self, name: &str, val: &ValueRef) {
        let c_name = CString::from(name);
        unsafe { LLVMAddNamedMetadataOperand(self.0, c_name.as_ptr(), val.get_ref()) };
    }
}

/// Get the template string used for an inline assembly snippet.
#[must_use]
pub fn get_inline_asm_asm_string(inline_asm_val: &ValueRef) -> Option<String> {
    inline_asm_val.get_inline_asm_asm_string()
}

/// Create the specified unique inline asm string.
#[must_use]
pub fn get_inline_asm(
    ty: &TypeRef,
    asm_string: &str,
    constraints: &str,
    has_side_effects: bool,
    is_align_stack: bool,
    dialect: InlineAsmDialect,
    can_throw: bool,
) -> ValueRef {
    let c_asm_string = CString::from(asm_string);
    let c_constraints = CString::from(constraints);
    let value_ref = unsafe {
        LLVMGetInlineAsm(
            ty.get_ref(),
            c_asm_string.as_ptr(),
            *SizeT(c_asm_string.to_bytes().len()),
            c_constraints.as_ptr(),
            *SizeT(c_constraints.to_bytes().len()),
            *CInt::from(has_side_effects),
            *CInt::from(is_align_stack),
            dialect.into(),
            *CInt::from(can_throw),
        )
    };
    ValueRef::from(value_ref)
}

/// Get the raw constraint string for an inline assembly snippet.
#[must_use]
pub fn get_inline_asm_constraint_string(inline_asm_val: &ValueRef) -> Option<String> {
    inline_asm_val.get_inline_asm_constraint_string()
}

/// Get the dialect used by the inline asm snippet.
#[must_use]
pub fn get_inline_asm_dialect(inline_asm_val: &ValueRef) -> InlineAsmDialect {
    inline_asm_val.get_inline_asm_dialect()
}

/// Get the function type of the inline assembly snippet.
///
/// This is the same type that was passed into `LLVMGetInlineAsm` originally.
#[must_use]
pub fn get_inline_asm_function_type(inline_asm_val: &ValueRef) -> TypeRef {
    inline_asm_val.get_inline_asm_function_type()
}

/// Get if the inline asm snippet has side effects
#[must_use]
pub fn get_inline_asm_has_side_effects(inline_asm_val: &ValueRef) -> bool {
    inline_asm_val.get_inline_asm_has_side_effects()
}

/// Get if the inline asm snippet needs an aligned stack
#[must_use]
pub fn get_inline_asm_needs_aligned_stack(inline_asm_val: &ValueRef) -> bool {
    inline_asm_val.get_inline_asm_needs_aligned_stack()
}

/// Get if the inline asm snippet may unwind the stack
#[must_use]
pub fn get_inline_asm_can_unwind(inline_asm_val: &ValueRef) -> bool {
    inline_asm_val.get_inline_asm_can_unwind()
}

/// Return the directory of the debug location for this value, which must be
/// an LLVM `Instruction`, `GlobalVariable`, or `Function`.
#[must_use]
pub fn get_debug_loc_directory(val: &ValueRef) -> Option<String> {
    val.get_debug_loc_directory()
}

/// Return the filename of the debug location for this value, which must be
/// an LLVM `Instruction`, `lGlobalVariable`, or `Function`.
#[must_use]
pub fn get_debug_loc_filename(val: &ValueRef) -> Option<String> {
    val.get_debug_loc_filename()
}

/// Return the line number of the debug location for this value, which must be
/// an LLVM `Instruction`, `GlobalVariable`, or `Function`.
#[must_use]
pub fn get_debug_loc_line(val: &ValueRef) -> u32 {
    val.get_debug_loc_line()
}

/// Return the column number of the debug location for this value, which must be
/// an LLVM `Instruction`.
#[must_use]
pub fn get_debug_loc_column(val: &ValueRef) -> u32 {
    val.get_debug_loc_column()
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
