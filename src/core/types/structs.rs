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

impl StructTypeRef {
    /// Create a new structure type in a context.
    ///
    /// A structure is specified by a list of inner elements/types and
    /// whether these can be packed together.
    #[must_use]
    pub fn struct_type_in_context(
        context: &ContextRef,
        element_types: &[Self],
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
    #[must_use]
    pub fn struct_type(element_types: &[Self], packed: bool) -> Self {
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
    #[must_use]
    pub fn get_struct_name(&self) -> Option<String> {
        unsafe {
            let c_str = core::LLVMGetStructName(self.0);
            if c_str.is_null() {
                None
            } else {
                Some(CStr::new(c_str).to_string())
            }
        }
    }

    /// Set the contents of a structure type.
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
    #[must_use]
    pub fn struct_get_type_at_index(&self, index: u32) -> TypeRef {
        unsafe { TypeRef(core::LLVMStructGetTypeAtIndex(self.0, *CUint::from(index))) }
    }

    /// Determine whether a structure is packed.
    #[must_use]
    pub fn is_packed_struct(&self) -> bool {
        unsafe { core::LLVMIsPackedStruct(self.0) != 0 }
    }

    /// Determine whether a structure is opaque.
    #[must_use]
    pub fn is_opaque_struct(&self) -> bool {
        unsafe { core::LLVMIsOpaqueStruct(self.0) != 0 }
    }

    /// Determine whether a structure is literal.
    #[must_use]
    pub fn is_literal_struct(&self) -> bool {
        unsafe { core::LLVMIsLiteralStruct(self.0) != 0 }
    }
}
