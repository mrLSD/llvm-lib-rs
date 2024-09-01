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
    ///
    /// # Details
    ///
    /// Dumps a textual representation of the LLVM value to standard output.
    ///
    /// This function wraps the `LLVMDumpValue` function from the LLVM core library. It prints a human-readable
    /// representation of the value represented by `self` to standard output. This is useful for debugging or
    /// inspecting the contents of a value during development.
    pub fn dump_value(&self) {
        unsafe { core::LLVMDumpValue(self.0) }
    }

    /// Return a string representation of the value. Use
    /// `dispose_message` to free the string.
    ///
    /// # Details
    ///
    /// Converts the LLVM value to a human-readable string representation.
    ///
    /// This function wraps the `LLVMPrintValueToString` function from the LLVM core library. It returns a
    /// string containing a human-readable representation of the value represented by `self`. This is useful
    /// for debugging or inspecting the contents of a value programmatically.
    ///
    /// The function returns `None` if the conversion fails or if the value cannot be represented as a string.
    ///
    /// # Returns
    ///
    /// Returns an `Option<String>`:
    /// - `Some(String)` containing the string representation of the value if successful.
    /// - `None` if the conversion fails or the value cannot be represented as a string.
    #[must_use]
    pub fn print_value_to_string(&self) -> Option<String> {
        unsafe {
            let c_str = core::LLVMPrintValueToString(self.0);
            if c_str.is_null() {
                return None;
            }
            let result = CStr::new(c_str).to_string();
            core::LLVMDisposeMessage(c_str);
            Some(result)
        }
    }

    /// Replace all uses of a value with another one.
    ///
    /// # Details
    ///
    /// Replaces all uses of this value with another value in the LLVM IR.
    ///
    /// This function wraps the `LLVMReplaceAllUsesWith` function from the LLVM core library. It replaces
    /// every use of the value represented by `self` with the value represented by `new_val`. This is useful
    /// for modifying LLVM IR when you need to substitute one value with another throughout the IR.
    ///
    /// # Parameters
    ///
    /// - `new_val`: A reference to the value that will replace all uses of `ValueRef`.
    pub fn replace_all_uses_with(&self, new_val: &Self) {
        unsafe { core::LLVMReplaceAllUsesWith(self.0, new_val.0) }
    }

    /// Determines whether the specified value instance is constant.
    ///
    /// # Details
    ///
    /// Checks if the value is a constant in LLVM IR.
    ///
    /// This function wraps the `LLVMIsConstant` function from the LLVM core library. It determines whether
    /// the value represented by `self` is a constant. In LLVM IR, constants are values that are known at compile time,
    /// such as integer literals, floating-point literals, or constant expressions.
    ///
    /// # Returns
    ///
    /// Returns `true` if the value is a constant, otherwise returns `false`.
    #[must_use]
    pub fn is_constant(&self) -> bool {
        unsafe { core::LLVMIsConstant(self.0) != 0 }
    }

    /// Determine whether a value instance is undefined.
    ///
    /// # Details
    ///
    /// Checks if the value is an 'undefined' value in LLVM IR.
    ///
    /// This function wraps the `LLVMIsUndef` function from the LLVM core library. It determines whether
    /// the value represented by `self` is an 'undefined' value. In LLVM IR, an undefined value is a placeholder
    /// that can take any value of the specified type during program execution, often used in optimization phases.
    ///
    /// # Returns
    ///
    /// Returns `true` if the value is an undefined value, otherwise returns `false`.
    #[must_use]
    pub fn is_undef(&self) -> bool {
        unsafe { core::LLVMIsUndef(self.0) != 0 }
    }

    /// Determine whether a value instance is poisonous.
    ///
    /// # Details
    ///
    /// Checks if the value is a 'poison' value in LLVM IR.
    ///
    /// This function wraps the `LLVMIsPoison` function from the LLVM core library. It determines whether
    /// the value represented by `self` is a 'poison' value. In LLVM IR, a poison value results from an operation
    /// with undefined behavior and can propagate through further operations, potentially leading to incorrect results.
    ///
    /// # Returns
    ///
    /// Returns `true` if the value is a poison value, otherwise returns `false`.
    #[must_use]
    pub fn is_poison(&self) -> bool {
        unsafe { core::LLVMIsPoison(self.0) != 0 }
    }

    /// Determines whether the specified value instance is an `AMD` node.
    ///
    /// # Details
    ///
    /// Checks if the value is an AMD node in LLVM IR and returns the corresponding value.
    ///
    /// This function wraps the `LLVMIsAMDNode` function from the LLVM core library. It determines whether
    /// the value represented by `self` is an AMD node and returns the corresponding value if it is. AMD nodes
    /// are specific to AMD's extensions in LLVM, and this function is used to identify and work with those nodes.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` that represents the value if it is an AMD node.
    #[must_use]
    pub fn is_amd_node(&self) -> Self {
        unsafe { Self(core::LLVMIsAMDNode(self.0)) }
    }

    /// Determines whether the specified value instance is a value as metadata.
    ///
    /// # Details
    ///
    /// Checks if the value can be treated as metadata in LLVM IR and returns the corresponding value.
    ///
    /// This function wraps the `LLVMIsAValueAsMetadata` function from the LLVM core library. It determines whether
    /// the value represented by `self` can be treated as metadata and returns the corresponding value if it can. In LLVM IR,
    /// some values can also be used as metadata, which is often used for attaching additional information to instructions
    /// or other IR elements.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` that represents the value if it can be treated as metadata.
    #[must_use]
    pub fn is_value_as_metadata(&self) -> Self {
        unsafe { Self(core::LLVMIsAValueAsMetadata(self.0)) }
    }

    /// Determines whether the specified value instance is an `AMD` string.
    ///
    /// # Details
    ///
    /// Checks if the value is an AMD string in LLVM IR and returns the corresponding value.
    ///
    /// This function wraps the `LLVMIsAMDString` function from the LLVM core library. It determines whether
    /// the value represented by `self` is an AMD string and returns the corresponding value if it is. AMD strings
    /// are specific to AMD's extensions in LLVM, and this function is used to identify and work with those strings.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` that represents the value if it is an AMD string.
    #[must_use]
    pub fn is_amd_string(&self) -> Self {
        unsafe { Self(core::LLVMIsAMDString(self.0)) }
    }
}
