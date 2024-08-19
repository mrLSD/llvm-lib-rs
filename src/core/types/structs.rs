//! Functions in this section operate on structs types.

use crate::core::context::ContextRef;
use crate::core::types::TypeRef;
use crate::{CInt, CStr, CString, CUint, GetRef};
use llvm_sys::core;
use llvm_sys::prelude::LLVMTypeRef;

/// These functions relate to `TypeRef` of `LLVMTypeRef` instances.
#[derive(Debug)]
pub struct StructTypeRef(LLVMTypeRef);

impl From<LLVMTypeRef> for StructTypeRef {
    fn from(value: LLVMTypeRef) -> Self {
        Self(value)
    }
}

impl GetRef for StructTypeRef {
    type RawRef = LLVMTypeRef;
    fn get_ref(&self) -> Self::RawRef {
        self.0
    }
}

impl From<StructTypeRef> for TypeRef {
    fn from(value: StructTypeRef) -> Self {
        Self(value.0)
    }
}

impl StructTypeRef {
    /// Create a new structure type in a context.
    ///
    /// A structure is specified by a list of inner elements/types and
    /// whether these can be packed together.
    ///
    /// # Details
    ///
    /// Creates a structure type in the specified LLVM context.
    ///
    /// This function wraps the `LLVMStructTypeInContext` function from the LLVM core library. It creates and returns
    /// a structure type within the specified LLVM context. The structure type can be composed of multiple elements,
    /// each specified by a `TypeRef`. The structure can be optionally packed, meaning that its elements are laid out
    /// contiguously in memory without any padding.
    ///
    /// # Parameters
    ///
    /// - `context`: A reference to the `ContextRef` in which the structure type will be created.
    /// - `element_types`: A slice of `TypeRef` representing the types of the elements in the structure. Each element in this slice corresponds to a field in the structure.
    /// - `packed`: A boolean indicating whether the structure should be packed (`true`) or unpacked (`false`). A packed structure has its fields tightly packed without padding.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the structure type in the specified context.
    #[must_use]
    pub fn struct_type_in_context(
        context: &ContextRef,
        element_types: &[TypeRef],
        packed: bool,
    ) -> Self {
        let mut element_types = element_types.iter().map(|v| v.0).collect::<Vec<_>>();
        let elements = if element_types.is_empty() {
            std::ptr::null_mut()
        } else {
            element_types.as_mut_ptr()
        };
        unsafe {
            Self(core::LLVMStructTypeInContext(
                context.get_ref(),
                elements,
                *CUint::from(element_types.len()),
                *CInt::from(packed),
            ))
        }
    }

    /// Create a new structure type in the global context.
    ///
    /// # Details
    ///
    /// Creates a structure type in the global LLVM context.
    ///
    /// This function wraps the `LLVMStructType` function from the LLVM core library. It creates and returns
    /// a structure type in the global LLVM context. The structure type can be composed of multiple elements,
    /// each specified by a `TypeRef`. The structure can be optionally packed, meaning that its elements are laid out
    /// contiguously in memory without any padding.
    ///
    /// # Parameters
    ///
    /// - `element_types`: A slice of `TypeRef` representing the types of the elements in the structure. Each element in this slice corresponds to a field in the structure.
    /// - `packed`: A boolean indicating whether the structure should be packed (`true`) or unpacked (`false`). A packed structure has its fields tightly packed without padding.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the structure type in the global context.
    #[must_use]
    pub fn struct_type(element_types: &[TypeRef], packed: bool) -> Self {
        let mut element_types = element_types.iter().map(|v| v.0).collect::<Vec<_>>();
        let elements = if element_types.is_empty() {
            std::ptr::null_mut()
        } else {
            element_types.as_mut_ptr()
        };
        unsafe {
            Self(core::LLVMStructType(
                elements,
                *CUint::from(element_types.len()),
                *CInt::from(packed),
            ))
        }
    }

    /// Create an empty structure in a context having a specified name.
    ///
    /// # Details
    ///
    /// Creates a named structure type in the specified LLVM context.
    ///
    /// This function wraps the `LLVMStructCreateNamed` function from the LLVM core library. It creates and returns
    /// a named structure type within the specified LLVM context. Named structures are useful when you want to refer
    /// to a structure type by name throughout your LLVM IR code, which can improve readability and organization,
    /// especially in complex modules.
    ///
    /// # Parameters
    ///
    /// - `context`: A reference to the `ContextRef` in which the named structure type will be created.
    /// - `name`: A string slice (`&str`) representing the name of the structure type. This name will be used to identify the structure type within the context.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the named structure type in the specified context.
    #[must_use]
    pub fn struct_create_named(context: &ContextRef, name: &str) -> Self {
        let c_name = CString::from(name);
        unsafe {
            Self(core::LLVMStructCreateNamed(
                context.get_ref(),
                c_name.as_ptr(),
            ))
        }
    }

    /// Obtain the name of a structure.
    ///
    /// # Details
    ///
    /// Retrieves the name of a named structure type in LLVM IR, if it has one.
    ///
    /// This function wraps the `LLVMGetStructName` function from the LLVM core library. It returns the name
    /// of the structure type represented by `self` as an `Option<String>`. This is useful for identifying named
    /// structure types in LLVM IR. If the structure is not named, the function returns `None`.
    ///
    /// # Returns
    ///
    /// Returns an `Option<String>`:
    /// - `Some(String)` containing the name of the structure type if it is named.
    /// - `None` if the structure type does not have a name.
    #[must_use]
    pub fn get_struct_name(&self) -> Option<String> {
        unsafe {
            let c_str = core::LLVMGetStructName(self.0);
            if c_str.is_null() {
                return None;
            }
            Some(CStr::new(c_str).to_string())
        }
    }

    /// Set the contents of a structure type.
    ///
    /// # Details
    ///
    /// Sets the body of a structure type in LLVM IR.
    ///
    /// This function wraps the `LLVMStructSetBody` function from the LLVM core library. It defines the fields
    /// (elements) of a structure type represented by `self`. You can specify the types of the fields and whether
    /// the structure should be packed (i.e., its fields are laid out contiguously in memory without padding).
    ///
    /// This function is typically used after creating a named structure type with `struct_create_named` to define
    /// its actual contents.
    ///
    /// # Parameters
    ///
    /// - `element_types`: A slice of `Self` representing the types of the elements (fields) in the structure. Each element in this slice corresponds to a field in the structure.
    /// - `packed`: A boolean indicating whether the structure should be packed (`true`) or unpacked (`false`). A packed structure has its fields tightly packed without padding.
    pub fn struct_set_body(&self, element_types: &[Self], packed: bool) {
        let mut element_types = element_types.iter().map(|v| v.0).collect::<Vec<_>>();
        let elements = if element_types.is_empty() {
            std::ptr::null_mut()
        } else {
            element_types.as_mut_ptr()
        };
        unsafe {
            core::LLVMStructSetBody(
                self.0,
                elements,
                *CUint::from(element_types.len()),
                *CInt::from(packed),
            );
        }
    }

    /// Get the number of elements defined inside the structure.
    ///
    /// # Details
    ///
    /// Counts the number of element types in a structure type in LLVM IR.
    ///
    /// This function wraps the `LLVMCountStructElementTypes` function from the LLVM core library. It returns
    /// the number of elements (fields) that the structure type represented by `self` has. This is useful for
    /// determining the size of the structure in terms of the number of fields it contains.
    ///
    /// # Returns
    ///
    /// Returns a `u32` representing the number of element types (fields) in the structure type.
    ///
    /// # Example
    ///
    /// ```rust
    /// let struct_type = ...; // Assume this is a structure type.
    /// let field_count = struct_type.count_struct_element_types();
    /// ```
    ///
    /// This retrieves the number of fields in the structure.
    #[must_use]
    pub fn count_struct_element_types(&self) -> u32 {
        unsafe { core::LLVMCountStructElementTypes(self.0) }
    }

    /// Get the elements within a structure.
    ///
    /// The function is passed the address of a pre-allocated array of
    /// `TypeRef` at least `count_struct_element_types()` long. After
    /// invocation, this array will be populated with the structure's
    /// elements. The objects in the destination array will have a lifetime
    /// of the structure type itself, which is the lifetime of the context it
    /// is contained in.
    ///
    /// # Details
    ///
    /// Retrieves the element types of a structure type in LLVM IR.
    ///
    /// This function wraps the `LLVMGetStructElementTypes` function from the LLVM core library. It returns a `Vec<TypeRef>`
    /// containing the types of all elements (fields) in the structure type represented by `self`. This is useful for inspecting
    /// the types of the fields within a structure in LLVM IR.
    ///
    /// # Returns
    ///
    /// Returns a `Vec<TypeRef>` representing the types of the elements (fields) in the structure type.
    pub fn get_struct_element_types(&self) -> Vec<TypeRef> {
        let count = self.count_struct_element_types() as usize;
        let mut raw_element_types: Vec<LLVMTypeRef> = Vec::with_capacity(count);
        unsafe {
            core::LLVMGetStructElementTypes(self.0, raw_element_types.as_mut_ptr());
            raw_element_types.set_len(count);
        }
        raw_element_types.into_iter().map(TypeRef).collect()
    }

    /// Get the type of the element at a given index in the structure.
    ///
    /// # Details
    ///
    /// Retrieves the type of the element at the specified index within a structure type in LLVM IR.
    ///
    /// This function wraps the `LLVMStructGetTypeAtIndex` function from the LLVM core library. It returns the `TypeRef`
    /// representing the type of the element (field) at the given `index` in the structure type represented by `self`.
    /// This is useful for accessing the type of a specific field in a structure.
    ///
    /// # Parameters
    ///
    /// - `index`: The index of the element (field) within the structure. The index is zero-based, meaning `0` refers to the first field.
    ///
    /// # Returns
    ///
    /// Returns a `TypeRef` representing the type of the element at the specified index.
    ///
    /// # Panics
    ///
    /// This function may panic if the index is out of bounds for the structure, depending on how the underlying LLVM function handles it.
    #[must_use]
    pub fn struct_get_type_at_index(&self, index: u32) -> TypeRef {
        unsafe { TypeRef(core::LLVMStructGetTypeAtIndex(self.0, *CUint::from(index))) }
    }

    /// Determine whether a structure is packed.
    ///
    /// # Details
    ///
    /// Checks whether a structure type in LLVM IR is packed.
    ///
    /// This function wraps the `LLVMIsPackedStruct` function from the LLVM core library. It determines whether
    /// the structure type represented by `self` is packed. A packed structure has its elements laid out contiguously
    /// in memory without any padding between them, which can save space but may result in alignment issues on some architectures.
    ///
    /// # Returns
    ///
    /// Returns `true` if the structure is packed, otherwise returns `false`.
    #[must_use]
    pub fn is_packed_struct(&self) -> bool {
        unsafe { core::LLVMIsPackedStruct(self.0) != 0 }
    }

    /// Determine whether a structure is opaque.
    ///
    /// # Details
    ///
    /// Checks whether a structure type in LLVM IR is opaque.
    ///
    /// This function wraps the `LLVMIsOpaqueStruct` function from the LLVM core library. It determines whether
    /// the structure type represented by `self` is opaque. An opaque structure is a structure type whose elements
    /// (fields) have not been defined or are intentionally hidden, often used in forward declarations or when the
    /// structure's layout is unknown at the time of creation.
    ///
    /// # Returns
    ///
    /// Returns `true` if the structure is opaque, otherwise returns `false`.
    #[must_use]
    pub fn is_opaque_struct(&self) -> bool {
        unsafe { core::LLVMIsOpaqueStruct(self.0) != 0 }
    }

    /// Determine whether a structure is literal.
    ///
    /// # Details
    ///
    /// Checks whether a structure type in LLVM IR is a literal structure.
    ///
    /// This function wraps the `LLVMIsLiteralStruct` function from the LLVM core library. It determines whether
    /// the structure type represented by `self` is a literal structure. A literal structure is defined directly by its elements
    /// without needing a named type, meaning that the structure's type is determined solely by the types of its fields.
    ///
    /// # Returns
    ///
    /// Returns `true` if the structure is a literal structure, otherwise returns `false`.
    #[must_use]
    pub fn is_literal_struct(&self) -> bool {
        unsafe { core::LLVMIsLiteralStruct(self.0) != 0 }
    }
}
