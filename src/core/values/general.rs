//! Functions in this section work on all `ValueRef` instances,
//! regardless of their sub-type. They correspond to functions available
//! on `C LLVM Value`.

use super::{ValueKind, ValueRef};
use crate::core::types::TypeRef;
use crate::{CStr, CString, SizeT};
use llvm_sys::core;

impl ValueRef {
    /// Obtain the type of the value.
    ///
    /// # Details
    ///
    /// Retrieves the LLVM type of the value.
    ///
    /// This function wraps the `LLVMTypeOf` function from the LLVM core library. It returns the `TypeRef` representing
    /// the LLVM type of the value represented by `self`. This is useful for inspecting the type information of values
    /// within LLVM IR, such as determining whether a value is an integer, floating-point, pointer, or another type.
    ///
    /// # Returns
    ///
    /// Returns a `TypeRef` that represents the LLVM type of the value.
    #[must_use]
    pub fn type_of(&self) -> TypeRef {
        unsafe { TypeRef::from(core::LLVMTypeOf(self.0)) }
    }

    /// Returns the kind of the given LLVM value (Obtain the enumerated type of the Value instance.).
    ///
    /// # Details
    ///
    /// Retrieves the kind of value represented by this LLVM value.
    ///
    /// This function wraps the `LLVMGetValueKind` function from the LLVM core library. It returns a `ValueKind`
    /// enumeration that identifies the specific kind of the value, such as whether it is an instruction, a constant,
    /// a global variable, a function, etc. This is useful for understanding what kind of entity a value represents within
    /// the LLVM IR.
    ///
    /// # Returns
    ///
    /// Returns a `ValueKind` enumeration that represents the kind of the value.
    #[must_use]
    pub fn get_value_kind(&self) -> ValueKind {
        unsafe { ValueKind::from(core::LLVMGetValueKind(self.0)) }
    }

    /// Obtain the string name of a value.
    ///
    /// # Details
    ///
    /// Retrieves the name of the LLVM value, if it has one.
    ///
    /// This function wraps the `LLVMGetValueName2` function from the LLVM core library. It returns the name of the
    /// value represented by `self` as a `String`, if the value has a name. In LLVM IR, named values typically include
    /// functions, global variables, and named instructions. If the value does not have a name, the function returns `None`.
    ///
    /// # Returns
    ///
    /// Returns an `Option<String>`:
    /// - `Some(String)` containing the name of the value if it has one.
    /// - `None` if the value does not have a name.
    #[must_use]
    pub fn get_value_name(&self) -> Option<String> {
        unsafe {
            let mut length = SizeT::from(0);
            let c_str = core::LLVMGetValueName2(self.0, &mut *length);
            if c_str.is_null() {
                return None;
            }
            Some(CStr::new(c_str).to_string())
        }
    }

    /// Set the string name of a value.
    ///
    /// # Details
    ///
    /// Sets the name of the LLVM value.
    ///
    /// This function wraps the `LLVMSetValueName2` function from the LLVM core library. It assigns a new name
    /// to the value represented by `self`. Naming a value in LLVM IR is useful for debugging, readability, and
    /// when generating human-readable IR. Named values typically include functions, global variables, and named
    /// instructions.
    ///
    /// # Parameters
    ///
    /// - `name`: A string slice (`&str`) representing the new name to assign to the value.
    ///
    /// # Example
    ///
    /// ```rust
    /// let my_value: ValueRef; // Assume this is an LLVM value.
    /// my_value.set_value_name("my_value_name");
    /// ```
    ///
    /// After calling this function, the value will be named "`my_value_name`" in the LLVM IR.
    pub fn set_value_name(&self, name: &str) {
        let c_string = CString::from(name);
        unsafe {
            core::LLVMSetValueName2(self.0, c_string.as_ptr(), *SizeT::from(name.len()));
        }
    }

    /// Dump a representation of a value to stderr.
    pub fn dump_value(&self) {
        unsafe { core::LLVMDumpValue(self.0) }
    }

    /// Return a string representation of the value. Use
    /// `dispose_message` to free the string.
    #[must_use]
    pub fn print_value_to_string(&self) -> Option<String> {
        unsafe {
            let c_str = core::LLVMPrintValueToString(self.0);
            if c_str.is_null() {
                return None;
            }
            let result = CStr::new(c_str).to_string();
            crate::core::dispose_message(c_str);
            Some(result)
        }
    }

    /// Replace all uses of a value with another one.
    pub fn replace_all_uses_with(&self, new_val: &Self) {
        unsafe { core::LLVMReplaceAllUsesWith(self.0, new_val.0) }
    }

    /// Determines whether the specified value instance is constant.
    #[must_use]
    pub fn is_constant(&self) -> bool {
        unsafe { core::LLVMIsConstant(self.0) != 0 }
    }

    /// Determine whether a value instance is undefined.
    #[must_use]
    pub fn is_undef(&self) -> bool {
        unsafe { core::LLVMIsUndef(self.0) != 0 }
    }

    /// Determine whether a value instance is poisonous.
    #[must_use]
    pub fn is_poison(&self) -> bool {
        unsafe { core::LLVMIsPoison(self.0) != 0 }
    }

    /// Determines whether the specified value instance is an `AMD` node.
    #[must_use]
    pub fn is_amd_node(&self) -> Self {
        unsafe { Self(core::LLVMIsAMDNode(self.0)) }
    }

    /// Determines whether the specified value instance is a value as metadata.
    #[must_use]
    pub fn is_value_as_metadata(&self) -> Self {
        unsafe { Self(core::LLVMIsAValueAsMetadata(self.0)) }
    }

    /// Determines whether the specified value instance is an `AMD` string.
    #[must_use]
    pub fn is_amd_string(&self) -> Self {
        unsafe { Self(core::LLVMIsAMDString(self.0)) }
    }
}
