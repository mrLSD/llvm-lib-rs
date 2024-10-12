//! Functions in this group operate on composite constants.

use super::ValueRef;
use crate::core::context::ContextRef;
use crate::core::types::TypeRef;
use crate::{CInt, CStr, CString, CUint, GetRef, SizeT};
use llvm_sys::core;

/// Create a `ConstantDataSequential` and initialize it with a string.
///
/// # Details
///
/// Creates a constant string value in a specified LLVM context.
///
/// This function wraps the `LLVMConstStringInContext2` function from the LLVM core library. It generates a constant
/// string value within the specified LLVM context (`context`) using the provided string slice (`string`). The function
/// also allows you to specify whether the string should be null-terminated.
///
/// # Parameters
///
/// - `context`: A reference to the LLVM context (`ContextRef`) in which the constant string should be created.
/// - `string`: A string slice that represents the content of the constant string. This string will be used to generate the LLVM constant.
/// - `dont_null_terminate`: A boolean value indicating whether the string should not be null-terminated. If `true`, the string will not be null-terminated; if `false`, a null terminator will be added.
///
/// # Returns
///
/// Returns an instance of `ValueRef`, which encapsulates the constant string value created in the specified context.
#[must_use]
pub fn const_string_in_context2(
    context: &ContextRef,
    string: &str,
    dont_null_terminate: bool,
) -> ValueRef {
    let c_string = CString::from(string);
    unsafe {
        ValueRef(core::LLVMConstStringInContext2(
            context.get_ref(),
            c_string.as_ptr(),
            *SizeT::from(string.len()),
            *CInt::from(dont_null_terminate),
        ))
    }
}

/// Create a `ConstantDataSequential` with string content in the global context.
///
/// This is the same as `const_string_in_context` except it operates on the
/// global context.
///
/// # Details
///
/// Creates a constant string value in the global LLVM context.
///
/// This function wraps the `LLVMConstString` function from the LLVM core library. It generates a constant
/// string value within the global LLVM context using the provided string slice (`string`). The function
/// also allows you to specify whether the string should be null-terminated. This function is similar to
/// `const_string_in_context`, but it operates on the global context instead of a specified context.
///
/// # Parameters
///
/// - `string`: A string slice that represents the content of the constant string. This string will be used to generate the LLVM constant.
/// - `dont_null_terminate`: A boolean value indicating whether the string should not be null-terminated. If `true`, the string will not be null-terminated; if `false`, a null terminator will be added.
///
/// # Returns
///
/// Returns an instance of `ValueRef`, which encapsulates the constant string value created in the global context.
#[must_use]
pub fn const_string(string: &str, dont_null_terminate: bool) -> ValueRef {
    let c_string = CString::from(string);
    unsafe {
        ValueRef(core::LLVMConstString(
            c_string.as_ptr(),
            *CUint::from(string.len()),
            *CInt::from(dont_null_terminate),
        ))
    }
}

/// Returns true if the specified constant is an array of `i8`.
///
/// # Details
///
/// Checks if the value is a constant string.
///
/// This function wraps the `LLVMIsConstantString` function from the LLVM core library. It determines whether
/// the value represented by `ValueRef` is a constant string. This is useful when you need to verify if a particular
/// LLVM value is a constant string within the IR.
///
/// # Returns
///
/// Returns `true` if the value is a constant string, otherwise returns `false`.
#[must_use]
pub fn is_constant_string(val: &ValueRef) -> bool {
    unsafe { core::LLVMIsConstantString(val.get_ref()) != 0 }
}

/// Get the given constant data sequential as a string.
///
/// # Details
///
/// Retrieves the value as a string if the value is a constant string.
///
/// This function wraps the `LLVMGetAsString` function from the LLVM core library. It attempts to extract the value
/// represented by `ValueRef` as a string if it is a constant string. The function returns `None` if the value is not a
/// constant string, or `Some(String)` containing the string representation if it is.
///
/// # Returns
///
/// Returns an `Option<String>`:
/// - `Some(String)` containing the string representation of the constant if it is a constant string.
/// - `None` if the value is not a constant string.
#[must_use]
pub fn get_as_string(val: &ValueRef) -> Option<String> {
    unsafe {
        let mut length = 0;
        let c_str = core::LLVMGetAsString(val.get_ref(), &mut length);
        if c_str.is_null() {
            None
        } else {
            Some(CStr::new(c_str).to_string())
        }
    }
}

/// Create an anonymous `ConstantStruct` with the specified values.
///
/// # Details
///
/// Creates a constant struct value in a specified LLVM context.
///
/// This function wraps the `LLVMConstStructInContext` function from the LLVM core library. It generates a constant
/// struct value within the specified LLVM context (`context`) using an array of constant values (`constant_vals`).
/// The `packed` parameter allows you to specify whether the struct should be packed, meaning that its fields are
/// laid out without padding.
///
/// # Parameters
///
/// - `context`: A reference to the LLVM context [`ContextRef`] in which the constant struct should be created.
/// - `constant_vals`: A slice of constant values &[`ValueRef`] that will be used as the fields of the struct. Each element in this slice corresponds to a field in the struct.
/// - `packed`: A boolean value indicating whether the struct should be packed (`true` for packed, `false` for unpacked). A packed struct has its fields tightly packed without padding.
///
/// # Returns
///
/// Returns an instance of [`ValueRef`], which encapsulates the constant struct value created in the specified context.
#[must_use]
pub fn const_struct_in_context(
    context: &ContextRef,
    constant_vals: &[ValueRef],
    packed: bool,
) -> ValueRef {
    let mut constant_vals = constant_vals.iter().map(|v| v.0).collect::<Vec<_>>();
    let constant_vals_ptr = if constant_vals.is_empty() {
        std::ptr::null_mut()
    } else {
        constant_vals.as_mut_ptr()
    };
    unsafe {
        ValueRef(core::LLVMConstStructInContext(
            context.get_ref(),
            constant_vals_ptr,
            *CUint::from(constant_vals.len()),
            *CInt::from(packed),
        ))
    }
}

/// Create a `ConstantStruct` in the global `Context`.
///
/// This is the same as `constStruct_in_context` except it operates on the
/// global context.
///
/// # Details
///
/// Creates a constant struct value in the global LLVM context.
///
/// This function wraps the `LLVMConstStruct` function from the LLVM core library. It generates a constant
/// struct value using an array of constant values (`constant_vals`). The `packed` parameter allows you to specify
/// whether the struct should be packed, meaning that its fields are laid out without padding. This function operates
/// in the global LLVM context, as opposed to a specific context.
///
/// # Parameters
///
/// - `constant_vals`: A slice of constant values &[`ValueRef`] that will be used as the fields of the struct. Each element in this slice corresponds to a field in the struct.
/// - `packed`: A boolean value indicating whether the struct should be packed (`true` for packed, `false` for unpacked). A packed struct has its fields tightly packed without padding.
///
/// # Returns
///
/// Returns an instance of `ValueRef`, which encapsulates the constant struct value created in the global context.
#[must_use]
pub fn const_struct(constant_vals: &[ValueRef], packed: bool) -> ValueRef {
    let mut constant_vals = constant_vals.iter().map(|v| v.0).collect::<Vec<_>>();
    let constant_vals_ptr = if constant_vals.is_empty() {
        std::ptr::null_mut()
    } else {
        constant_vals.as_mut_ptr()
    };
    unsafe {
        ValueRef(core::LLVMConstStruct(
            constant_vals_ptr,
            *CUint::from(constant_vals.len()),
            *CInt::from(packed),
        ))
    }
}

/// Create a `ConstantArray` from values.
///
/// # Details
///
/// Creates a constant array value with elements of a specified type.
///
/// This function wraps the `LLVMConstArray2` function from the LLVM core library. It generates a constant
/// array value with the specified element type (`element_type`) using an array of constant values (`constant_vals`).
/// Each element in `constant_vals` must be of the same type as `element_type`.
///
/// # Parameters
///
/// - `element_type`: A reference to the type of elements in the array (`TypeRef`). This specifies the type that each element in the array should have.
/// - `constant_vals`: A slice of constant values (`&[ValueRef]`) that will be used as the elements of the array. Each element in this slice corresponds to an element in the resulting array.
///
/// # Returns
///
/// Returns an instance of [`ValueRef`], which encapsulates the constant array value created with the specified element type and elements.
#[must_use]
pub fn const_array2(element_type: &TypeRef, constant_vals: &[ValueRef]) -> ValueRef {
    let mut constant_vals = constant_vals.iter().map(|v| v.0).collect::<Vec<_>>();
    let constant_vals_ptr = if constant_vals.is_empty() {
        std::ptr::null_mut()
    } else {
        constant_vals.as_mut_ptr()
    };
    unsafe {
        ValueRef(core::LLVMConstArray2(
            element_type.get_ref(),
            constant_vals_ptr,
            u64::try_from(constant_vals.len()).unwrap_or(u64::MAX),
        ))
    }
}

/// Create a non-anonymous `ConstantStruct` from values.
///
/// # Details
///
/// Creates a constant named struct value with specified field values.
///
/// This function wraps the `LLVMConstNamedStruct` function from the LLVM core library. It generates a constant
/// struct value of the specified named struct type (`struct_type`) using an array of constant values (`constant_vals`).
/// Each element in `constant_vals` corresponds to a field in the struct.
///
/// # Parameters
///
/// - `struct_type`: A reference to the named struct type (`TypeRef`) for the constant value. This type specifies the structure that the constant will represent.
/// - `constant_vals`: A slice of constant values (`&[ValueRef]`) that will be used as the fields of the struct. Each element in this slice corresponds to a field in the struct.
///
/// # Returns
///
/// Returns an instance of [`ValueRef`], which encapsulates the constant named struct value created with the specified fields.
#[must_use]
pub fn const_named_struct(struct_type: &TypeRef, constant_vals: &[ValueRef]) -> ValueRef {
    let mut constant_vals = constant_vals.iter().map(|v| v.0).collect::<Vec<_>>();
    let constant_vals_ptr = if constant_vals.is_empty() {
        std::ptr::null_mut()
    } else {
        constant_vals.as_mut_ptr()
    };
    unsafe {
        ValueRef(core::LLVMConstNamedStruct(
            struct_type.get_ref(),
            constant_vals_ptr,
            *CUint::from(constant_vals.len()),
        ))
    }
}

/// Get element of a constant aggregate `(struct, array or vector)` at the
/// specified index. Returns `None` if the index is out of range, or it's not
/// possible to determine the element (e.g., because the constant is a
/// constant expression.)
///
/// # Details
///
/// Retrieves a specific element from an aggregate constant (e.g., an array or struct).
///
/// This function wraps the `LLVMGetAggregateElement` function from the LLVM core library. It returns the element
/// at the specified index (`idx`) from the aggregate constant represented by [`ValueRef`]. If the index is out of bounds
/// or the element cannot be retrieved, the function returns `None`.
///
/// # Parameters
///
/// - `idx`: The index of the element to retrieve from the aggregate constant. This index should be within the bounds of the aggregate's elements.
///
/// # Returns
///
/// Returns an [Option<ValueRef>]:
/// - `Some(ValueRef)` containing the retrieved element if the index is valid and the element is found.
/// - `None` if the index is out of bounds or the element cannot be retrieved.
#[must_use]
pub fn get_aggregate_element(val: &ValueRef, idx: u32) -> Option<ValueRef> {
    let element = unsafe { core::LLVMGetAggregateElement(val.get_ref(), *CUint::from(idx)) };
    if element.is_null() {
        None
    } else {
        Some(ValueRef(element))
    }
}

/// Create a `ConstantVector` from values.
///
/// # Details
///
/// Creates a constant vector value from an array of scalar constant values.
///
/// This function wraps the `LLVMConstVector` function from the LLVM core library. It generates a constant
/// vector using the provided array of scalar constant values (`scalar_constant_vals`). Each element in the array
/// corresponds to an element in the resulting vector, and the type of the vector is inferred from the types of the scalar constants.
///
/// # Parameters
///
/// - `scalar_constant_vals`: A slice of scalar constant values [`ValueRef`] that will be used as the elements of the vector. Each element in this slice corresponds to an element in the resulting vector.
///
/// # Returns
///
/// Returns an instance of [`ValueRef`], which encapsulates the constant vector value created from the specified scalar constants.
#[must_use]
pub fn const_vector(scalar_constant_vals: &[ValueRef]) -> ValueRef {
    let mut scalar_constant_vals = scalar_constant_vals.iter().map(|v| v.0).collect::<Vec<_>>();
    let scalar_constant_vals_ptr = if scalar_constant_vals.is_empty() {
        std::ptr::null_mut()
    } else {
        scalar_constant_vals.as_mut_ptr()
    };
    unsafe {
        ValueRef(core::LLVMConstVector(
            scalar_constant_vals_ptr,
            *CUint::from(scalar_constant_vals.len()),
        ))
    }
}
