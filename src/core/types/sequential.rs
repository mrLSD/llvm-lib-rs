use crate::core::context::ContextRef;
use crate::core::types::TypeRef;
use crate::{CUint, GetRef};
use llvm_sys::core;
use llvm_sys::prelude::LLVMTypeRef;

/// These functions relate to `SequentialTypeRef` of `LLVMTypeRef` instances.
///
/// Sequential types represents "arrays" of types. This is a super class
/// for array, vector, and pointer types.
#[derive(Debug)]
pub struct SequentialTypeRef(LLVMTypeRef);

impl From<LLVMTypeRef> for SequentialTypeRef {
    fn from(value: LLVMTypeRef) -> Self {
        Self(value)
    }
}

impl GetRef for SequentialTypeRef {
    type RawRef = LLVMTypeRef;
    fn get_ref(&self) -> Self::RawRef {
        self.0
    }
}

impl SequentialTypeRef {
    /// Obtain the element type of array or vector type.
    #[must_use]
    pub fn get_element_type(&self) -> TypeRef {
        unsafe { TypeRef(core::LLVMGetElementType(self.0)) }
    }

    /// Returns type's subtypes
    #[must_use]
    pub fn get_subtypes(&self) -> Vec<TypeRef> {
        let count = self.get_num_contained_types() as usize;
        let mut subtypes: Vec<LLVMTypeRef> = Vec::with_capacity(count);
        unsafe {
            core::LLVMGetSubtypes(self.0, subtypes.as_mut_ptr());
            subtypes.set_len(count);
        }
        subtypes.into_iter().map(TypeRef).collect()
    }

    /// Return the number of types in the derived type.
    #[must_use]
    pub fn get_num_contained_types(&self) -> u32 {
        unsafe { core::LLVMGetNumContainedTypes(self.0) }
    }

    /// Create a fixed size array type that refers to a specific type.
    ///
    /// The created type will exist in the context that its element type exists in.
    #[must_use]
    pub fn array_type2(element_type: &TypeRef, element_count: u64) -> Self {
        unsafe { Self(core::LLVMArrayType2(element_type.0, element_count)) }
    }

    /// Obtain the length of an array type.
    ///
    /// This only works on types that represent arrays.
    #[must_use]
    pub fn get_array_length2(&self) -> u64 {
        unsafe { core::LLVMGetArrayLength2(self.0) }
    }

    /// Create a pointer type that points to a defined type.
    ///
    /// The created type will exist in the context that its pointee type exists in.
    #[must_use]
    pub fn pointer_type(element_type: &TypeRef, address_space: u32) -> Self {
        unsafe { Self(core::LLVMPointerType(element_type.0, address_space)) }
    }

    /// Determine whether a pointer is opaque.
    ///
    /// True if this is an instance of an opaque `PointerType`.
    #[must_use]
    pub fn is_pointer_opaque(&self) -> bool {
        unsafe { core::LLVMPointerTypeIsOpaque(self.0) != 0 }
    }

    /// Create an opaque pointer type in a context.
    #[must_use]
    pub fn opaque_pointer_type_in_context(context: &ContextRef, address_space: u32) -> Self {
        unsafe {
            Self(core::LLVMPointerTypeInContext(
                context.get_ref(),
                address_space,
            ))
        }
    }

    /// Obtain the address space of a pointer type.
    ///
    /// This only works on types that represent pointers.
    #[must_use]
    pub fn get_pointer_address_space(&self) -> u32 {
        unsafe { core::LLVMGetPointerAddressSpace(self.0) }
    }

    /// Create a vector type that contains a defined type and has a specific
    /// number of elements.
    ///
    /// The created type will exist in the context that its element type exists in.
    #[must_use]
    pub fn vector_type(element_type: &TypeRef, element_count: u32) -> Self {
        unsafe {
            Self(core::LLVMVectorType(
                element_type.0,
                *CUint::from(element_count),
            ))
        }
    }

    /// Create a vector type that contains a defined type and has a scalable
    /// number of elements.
    ///
    /// The created type will exist in the context that its element type
    /// exists in.
    #[must_use]
    pub fn scalable_vector_type(element_type: &TypeRef, element_count: u32) -> Self {
        unsafe {
            Self(core::LLVMScalableVectorType(
                element_type.0,
                *CUint::from(element_count),
            ))
        }
    }

    /// Obtain the (possibly scalable) number of elements in a vector type.
    ///
    /// This only works on types that represent vectors (fixed or scalable).
    #[must_use]
    pub fn get_vector_size(&self) -> u32 {
        unsafe { core::LLVMGetVectorSize(self.0) }
    }
}
