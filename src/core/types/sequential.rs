//! Functions in this section operate on sequential types.

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

impl From<SequentialTypeRef> for TypeRef {
    fn from(value: SequentialTypeRef) -> Self {
        Self(value.0)
    }
}

impl SequentialTypeRef {
    /// Obtain the element type of array or vector type.
    ///
    /// # Details
    ///
    /// Retrieves the element type of a composite LLVM type.
    ///
    /// This function wraps the `LLVMGetElementType` function from the LLVM core library. It returns the `TypeRef`
    /// representing the element type of the composite type associated with `self`. Composite types in LLVM include
    /// arrays, pointers, vectors, and structures. For example, for an array type, this function returns the type of the
    /// array elements.
    ///
    /// # Returns
    ///
    /// Returns a `TypeRef` representing the element type of the composite type.
    #[must_use]
    pub fn get_element_type(&self) -> TypeRef {
        unsafe { TypeRef(core::LLVMGetElementType(self.0)) }
    }

    /// Returns type's subtypes
    ///
    /// # Details
    ///
    /// Retrieves the subtypes of a composite LLVM type.
    ///
    /// This function wraps the `LLVMGetSubtypes` function from the LLVM core library. It returns a vector of `TypeRef`
    /// instances representing the subtypes contained within the composite type associated with `self`. Composite types,
    /// such as structures, arrays, and vectors, may contain multiple subtypes, and this function provides access to them.
    ///
    /// # Returns
    ///
    /// Returns a `Vec<TypeRef>` containing the subtypes of the composite type.
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
    ///
    /// # Details
    ///
    /// Retrieves the number of contained types in a composite LLVM type.
    ///
    /// This function wraps the `LLVMGetNumContainedTypes` function from the LLVM core library. It returns the number
    /// of subtypes contained within the composite type associated with `self`. Composite types, such as structures, arrays,
    /// and vectors, can contain multiple types, and this function provides the count of those contained types.
    ///
    /// # Returns
    ///
    /// Returns a `u32` representing the number of contained types in the composite type.
    #[must_use]
    pub fn get_num_contained_types(&self) -> u32 {
        unsafe { core::LLVMGetNumContainedTypes(self.0) }
    }

    /// Create a fixed size array type that refers to a specific type.
    /// The created type will exist in the context that its element type exists in.
    ///
    /// # Details
    ///
    /// Creates an array type in LLVM IR with the specified element type and number of elements.
    ///
    /// This function wraps the `LLVMArrayType2` function from the LLVM core library. It creates and returns an array type
    /// where each element is of the specified type, and the array contains the specified number of elements. Array types are
    /// fixed-size sequences of elements of the same type.
    ///
    /// # Parameters
    ///
    /// - `element_type`: A reference to the `TypeRef` representing the type of each element in the array.
    /// - `element_count`: The number of elements in the array.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the array type with the specified element type and number of elements.
    #[must_use]
    pub fn array_type2(element_type: &TypeRef, element_count: u64) -> Self {
        unsafe { Self(core::LLVMArrayType2(element_type.0, element_count)) }
    }

    /// Obtain the length of an array type.
    /// This only works on types that represent arrays.
    ///
    /// # Details
    ///
    /// Retrieves the number of elements in an LLVM array type.
    ///
    /// This function wraps the `LLVMGetArrayLength2` function from the LLVM core library. It returns the number
    /// of elements in the array type represented by `self`. This is useful for determining the size of the array
    /// in terms of the number of elements it contains.
    ///
    /// # Returns
    ///
    /// Returns a `u64` representing the number of elements in the array type.
    #[must_use]
    pub fn get_array_length2(&self) -> u64 {
        unsafe { core::LLVMGetArrayLength2(self.0) }
    }

    /// Create a pointer type that points to a defined type.
    /// The created type will exist in the context that its pointee type exists in.
    ///
    /// # Details
    ///
    /// Creates a pointer type in LLVM IR with the specified element type and address space.
    ///
    /// This function wraps the `LLVMPointerType` function from the LLVM core library. It creates and returns a pointer type
    /// that points to elements of the specified type in the given address space. In LLVM IR, pointers can reference
    /// memory in different address spaces, which is useful in various architectures and scenarios.
    ///
    /// # Parameters
    ///
    /// - `element_type`: A reference to the `TypeRef` representing the type of the elements the pointer will point to.
    /// - `address_space`: A `u32` representing the address space where the pointer will reside. Different address spaces
    ///   can be used to distinguish between different memory regions, such as global, local, or private memory.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the pointer type with the specified element type and address space.
    #[must_use]
    pub fn pointer_type(element_type: &TypeRef, address_space: u32) -> Self {
        unsafe { Self(core::LLVMPointerType(element_type.0, address_space)) }
    }

    /// Determine whether a pointer is opaque.
    /// True if this is an instance of an opaque `PointerType`.
    ///
    /// # Details
    ///
    /// Checks whether a pointer type in LLVM IR is opaque.
    ///
    /// This function wraps the `LLVMPointerTypeIsOpaque` function from the LLVM core library. It determines whether
    /// the pointer type represented by `self` is opaque. An opaque pointer type is one where the pointee type is not specified,
    /// meaning the type of the elements the pointer points to is not known or defined.
    ///
    /// # Returns
    ///
    /// Returns `true` if the pointer type is opaque, otherwise returns `false`.
    #[must_use]
    pub fn is_pointer_opaque(&self) -> bool {
        unsafe { core::LLVMPointerTypeIsOpaque(self.0) != 0 }
    }

    /// Create an opaque pointer type in a context.
    ///
    /// # Details
    ///
    /// Creates an opaque pointer type in the specified LLVM context with the given address space.
    ///
    /// This function wraps the `LLVMPointerTypeInContext` function from the LLVM core library. It creates and returns
    /// an opaque pointer type within the specified LLVM context. An opaque pointer type is a pointer type where the
    /// pointee type is not specified or defined, which can be useful in generic programming or when the type is
    /// not yet determined.
    ///
    /// # Parameters
    ///
    /// - `context`: A reference to the `ContextRef` in which the opaque pointer type will be created.
    /// - `address_space`: A `u32` representing the address space where the pointer will reside. Different address spaces
    ///   can be used to distinguish between different memory regions, such as global, local, or private memory.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the opaque pointer type in the specified context and address space.
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
    /// This only works on types that represent pointers.
    ///
    /// # Details
    ///
    /// Retrieves the address space of a pointer type in LLVM IR.
    ///
    /// This function wraps the `LLVMGetPointerAddressSpace` function from the LLVM core library. It returns the address space
    /// associated with the pointer type represented by `self`. In LLVM IR, pointers can exist in different address spaces,
    /// which are used to distinguish between various memory regions, such as global, local, or private memory.
    ///
    /// # Returns
    ///
    /// Returns a `u32` representing the address space of the pointer type.
    #[must_use]
    pub fn get_pointer_address_space(&self) -> u32 {
        unsafe { core::LLVMGetPointerAddressSpace(self.0) }
    }

    /// Create a vector type that contains a defined type and has a specific
    /// number of elements.
    /// The created type will exist in the context that its element type exists in.
    ///
    /// # Details
    ///
    /// Creates a vector type in LLVM IR with the specified element type and number of elements.
    ///
    /// This function wraps the `LLVMVectorType` function from the LLVM core library. It creates and returns a vector type
    /// where each element is of the specified type, and the vector contains the specified number of elements. Vector types
    /// are used in LLVM IR to represent a sequence of elements that can be processed in parallel, often in SIMD (Single Instruction, Multiple Data) operations.
    ///
    /// # Parameters
    ///
    /// - `element_type`: A reference to the `TypeRef` representing the type of each element in the vector.
    /// - `element_count`: The number of elements in the vector.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the vector type with the specified element type and number of elements.
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
    /// The created type will exist in the context that its element type
    /// exists in.
    ///
    /// # Details
    ///
    /// Creates a scalable vector type in LLVM IR with the specified element type and minimum number of elements.
    ///
    /// This function wraps the `LLVMScalableVectorType` function from the LLVM core library. It creates and returns a scalable vector type
    /// where each element is of the specified type, and the vector contains at least the specified number of elements. Scalable vector types
    /// are used in LLVM IR to represent vectors whose size can vary at runtime, depending on the target's hardware capabilities, while ensuring
    /// a minimum number of elements.
    ///
    /// # Parameters
    ///
    /// - `element_type`: A reference to the `TypeRef` representing the type of each element in the scalable vector.
    /// - `element_count`: The minimum number of elements in the scalable vector.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the scalable vector type with the specified element type and minimum number of elements.
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
    /// This only works on types that represent vectors (fixed or scalable).
    ///
    /// # Details
    ///
    /// Retrieves the number of elements in a fixed-size vector type in LLVM IR.
    ///
    /// This function wraps the `LLVMGetVectorSize` function from the LLVM core library. It returns the number
    /// of elements in the vector type represented by `self`. This function is typically used with fixed-size vector types
    /// in LLVM IR to determine how many elements the vector contains.
    ///
    /// # Returns
    ///
    /// Returns a `u32` representing the number of elements in the vector type.
    #[must_use]
    pub fn get_vector_size(&self) -> u32 {
        unsafe { core::LLVMGetVectorSize(self.0) }
    }
}
