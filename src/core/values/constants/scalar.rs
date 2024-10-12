//! Functions in this group model `ValueRef` instances that correspond
//! to constants referring to scalar types.

use super::ValueRef;
use crate::core::types::TypeRef;
use crate::{CDouble, CInt, CString, CUint, GetRef};
use llvm_sys::core;

/// Obtain a constant value for an integer type.
/// The returned value corresponds to a `llvm ConstantInt`.
///
/// # Details
///
/// Creates a constant integer value of a specified type.
///
/// This function wraps the `LLVMConstInt` function from the LLVM core library. It generates a constant integer
/// value of the type specified by `ty`, using the provided `n` as the value. The `sign_extend` parameter determines
/// whether the value should be sign-extended to the specified type if the type is larger than the original value.
///
/// # Parameters
///
/// - `ty`: A reference to the integer type (`TypeRef`) for the constant value. This specifies the bit width and signedness of the integer.
/// - `n`: The integer value to be used for the constant. It will be interpreted according to the bit width and signedness of the target type.
/// - `sign_extend`: A boolean value indicating whether the constant should be sign-extended to the target type. If `true`, the value will be sign-extended; if `false`, it will be zero-extended.
///
/// # Returns
///
/// Returns an instance of `ValueRef`, which encapsulates the constant integer value determined at compile time.
#[must_use]
pub fn const_int(ty: &TypeRef, n: u64, sign_extend: bool) -> ValueRef {
    unsafe {
        ValueRef(core::LLVMConstInt(
            ty.get_ref(),
            n,
            *CInt::from(sign_extend),
        ))
    }
}

/// Obtain a constant value for an integer of arbitrary precision.
///
/// # Details
///
/// Creates a constant integer value of arbitrary precision.
///
/// This function wraps the `LLVMConstIntOfArbitraryPrecision` function from the LLVM core library. It generates a constant
/// integer value of the specified type (`ty`) using an array of 64-bit words (`words`). This allows for the creation of
/// integers that exceed the typical bit width limitations by providing multiple 64-bit words to represent the value.
///
/// # Parameters
///
/// - `ty`: A reference to the integer type (`TypeRef`) for the constant value. This type specifies the bit width and signedness of the integer.
/// - `words`: A slice of 64-bit words (`u64`) that represents the value of the constant integer. Each word in the array contributes to the overall bit representation of the integer, allowing for arbitrary precision.
///
/// # Returns
///
/// Returns an instance of `ValueRef`, which encapsulates the constant integer value with arbitrary precision, as determined at compile time.
#[must_use]
pub fn const_int_of_arbitrary_precision(ty: &TypeRef, words: &[u64]) -> ValueRef {
    unsafe {
        ValueRef(core::LLVMConstIntOfArbitraryPrecision(
            ty.get_ref(),
            *CUint::from(words.len()),
            words.as_ptr(),
        ))
    }
}

/// Obtain a constant value for an integer parsed from a string.
///
/// A similar API, `const_int_of_string_and_size` is also available. If the
/// string's length is available, it is preferred to call that function
/// instead.
///
/// # Details
///
/// Creates a constant integer value by parsing a string representation of the integer.
///
/// This function wraps the `LLVMConstIntOfString` function from the LLVM core library. It generates a constant
/// integer value of the specified type (`ty`) by parsing the provided string (`text`) according to the specified
/// radix (`radix`). This function is useful when you need to create constant integers from string literals in various bases
/// (e.g., binary, octal, decimal, hexadecimal).
///
/// # Parameters
///
/// - `ty`: A reference to the integer type (`TypeRef`) for the constant value. This type specifies the bit width and signedness of the integer.
/// - `text`: A string slice that represents the integer value to be parsed. The string should be a valid representation of an integer in the specified radix.
/// - `radix`: The radix (or base) used to interpret the string. Common values include 2 (binary), 8 (octal), 10 (decimal), and 16 (hexadecimal).
///
/// # Returns
///
/// Returns an instance of `ValueRef`, which encapsulates the constant integer value parsed from the string at compile time.
#[must_use]
pub fn const_int_of_string(ty: &TypeRef, text: &str, radix: u8) -> ValueRef {
    let c_text = CString::from(text);
    unsafe {
        ValueRef(core::LLVMConstIntOfString(
            ty.get_ref(),
            c_text.as_ptr(),
            radix,
        ))
    }
}

/// Obtain a constant value for an integer parsed from a string with
/// specified length.
///
/// # Details
///
/// Creates a constant integer value by parsing a string representation of the integer, with a specified string length.
///
/// This function wraps the `LLVMConstIntOfStringAndSize` function from the LLVM core library. It generates a constant
/// integer value of the specified type (`ty`) by parsing the provided string (`text`) according to the specified
/// radix (`radix`). The length of the string is explicitly provided, which can be more efficient when the string length
/// is known or can be easily determined, as it avoids the need for additional computation or checks.
///
/// # Parameters
///
/// - `ty`: A reference to the integer type (`TypeRef`) for the constant value. This type specifies the bit width and signedness of the integer.
/// - `text`: A string slice that represents the integer value to be parsed. The string should be a valid representation of an integer in the specified radix.
/// - `radix`: The radix (or base) used to interpret the string. Common values include 2 (binary), 8 (octal), 10 (decimal), and 16 (hexadecimal).
///
/// # Returns
///
/// Returns an instance of `ValueRef`, which encapsulates the constant integer value parsed from the string at compile time.
///
/// # Efficiency
///
/// This function is recommended when the length of the string is known, as it may offer better performance
/// compared to `const_int_of_string` by avoiding the overhead of calculating the string length within the function.
#[must_use]
pub fn const_int_of_string_and_size(ty: &TypeRef, text: &str, radix: u8) -> ValueRef {
    let c_text = CString::from(text);
    unsafe {
        ValueRef(core::LLVMConstIntOfStringAndSize(
            ty.get_ref(),
            c_text.as_ptr(),
            *CUint::from(text.len()),
            radix,
        ))
    }
}

/// Obtain a constant value referring to a double floating point value.
///
/// # Details
///
/// Creates a constant floating-point value of a specified type.
///
/// This function wraps the `LLVMConstReal` function from the LLVM core library. It generates a constant
/// floating-point value of the type specified by `ty`, using the provided floating-point number `n`. This is
/// typically used to create floating-point constants within LLVM's Intermediate Representation (IR) at compile time.
///
/// # Parameters
///
/// - `ty`: A reference to the floating-point type (`TypeRef`) for the constant value. This type specifies the bit width of the floating-point value (e.g., `f32`, `f64`).
/// - `n`: The floating-point value to be used for the constant. It will be interpreted according to the bit width of the target floating-point type.
///
/// # Returns
///
/// Returns an instance of `ValueRef`, which encapsulates the constant floating-point value determined at compile time.
#[must_use]
pub fn const_real(ty: &TypeRef, n: f64) -> ValueRef {
    unsafe { ValueRef(core::LLVMConstReal(ty.get_ref(), *CDouble::from(n))) }
}

/// Obtain a constant for a floating point value parsed from a string.
///
/// A similar API, `const_real_of_string_and_size` is also available. It
/// should be used if the input string's length is known.
///
/// # Details
///
/// Creates a constant floating-point value by parsing a string representation of the number.
///
/// This function wraps the `LLVMConstRealOfString` function from the LLVM core library. It generates a constant
/// floating-point value of the specified type (`ty`) by parsing the provided string (`text`). This is useful when
/// creating floating-point constants from string literals, especially when the value is specified in textual form
/// rather than directly as a floating-point number.
///
/// # Parameters
///
/// - `ty`: A reference to the floating-point type (`TypeRef`) for the constant value. This type specifies the bit width of the floating-point value (e.g., `f32`, `f64`).
/// - `text`: A string slice that represents the floating-point value to be parsed. The string should be a valid representation of a floating-point number in the expected format.
///
/// # Returns
///
/// Returns an instance of `ValueRef`, which encapsulates the constant floating-point value parsed from the string at compile time.
#[must_use]
pub fn const_real_of_string(ty: &TypeRef, text: &str) -> ValueRef {
    let c_text = CString::from(text);
    unsafe { ValueRef(core::LLVMConstRealOfString(ty.get_ref(), c_text.as_ptr())) }
}

/// Obtain a constant for a floating point value parsed from a string with specified length.
///
/// # Details
///
/// Creates a constant floating-point value by parsing a string representation of the number, with a specified string length.
///
/// This function wraps the `LLVMConstRealOfStringAndSize` function from the LLVM core library. It generates a constant
/// floating-point value of the specified type (`ty`) by parsing the provided string (`text`) according to its length.
/// This function is useful when the length of the input string is known, as it may provide better performance by
/// avoiding the need to compute the string length internally.
///
/// # Parameters
///
/// - `ty`: A reference to the floating-point type (`TypeRef`) for the constant value. This type specifies the bit width of the floating-point value (e.g., `f32`, `f64`).
/// - `text`: A string slice that represents the floating-point value to be parsed. The string should be a valid representation of a floating-point number in the expected format.
///
/// # Returns
///
/// Returns an instance of `ValueRef`, which encapsulates the constant floating-point value parsed from the string at compile time.
///
/// # Efficiency
///
/// This function is recommended when the length of the string is known, as it may offer better performance
/// compared to `const_real_of_string` by avoiding the overhead of calculating the string length within the function.
#[must_use]
pub fn const_real_of_string_and_size(ty: &TypeRef, text: &str) -> ValueRef {
    let c_text = CString::from(text);
    unsafe {
        ValueRef(core::LLVMConstRealOfStringAndSize(
            ty.get_ref(),
            c_text.as_ptr(),
            *CUint::from(text.len()),
        ))
    }
}

/// Obtain the zero extended value for an integer constant value.
///
/// # Details
///
/// Retrieves the zero-extended value of a constant integer as a `u64`.
///
/// This function wraps the `LLVMConstIntGetZExtValue` function from the LLVM core library. It returns the value
/// of the constant integer represented by `ValueRef`, zero-extending it to 64 bits if necessary. This is useful when you need
/// to extract the numeric value of a constant integer in a form that can be used in regular Rust code.
///
/// # Returns
///
/// Returns a `u64` that represents the zero-extended value of the constant integer.
#[must_use]
pub fn const_int_get_zext_value(val: &ValueRef) -> u64 {
    unsafe { core::LLVMConstIntGetZExtValue(val.get_ref()) }
}

/// Obtain the sign extended value for an integer constant value.
///
/// # Details
///
/// Retrieves the sign-extended value of a constant integer as an `i64`.
///
/// This function wraps the `LLVMConstIntGetSExtValue` function from the LLVM core library. It returns the value
/// of the constant integer represented by `ValueRef`, sign-extending it to 64 bits if necessary. This is useful when you need
/// to extract the numeric value of a constant integer in a signed form that can be used in regular Rust code.
///
/// # Returns
///
/// Returns an `i64` that represents the sign-extended value of the constant integer.
#[must_use]
pub fn const_int_get_sext_value(val: &ValueRef) -> i64 {
    unsafe { core::LLVMConstIntGetSExtValue(val.get_ref()) }
}

/// Obtain the double value for a floating point constant value.
/// `losesInfo` indicates if some precision was lost in the conversion.
///
/// # Details
///
/// Retrieves the double-precision floating-point value from a constant floating-point value.
///
/// This function wraps the `LLVMConstRealGetDouble` function from the LLVM core library. It extracts the value
/// of the constant floating-point represented by `ValueRef` as an `f64`. The function also indicates if any precision
/// was lost during the conversion by setting the `losesInfo` flag.
///
/// # Returns
///
/// Returns a tuple containing:
/// - An `f64` representing the double-precision floating-point value.
/// - A `bool` flag (`losesInfo`) indicating whether some precision was lost in the conversion (`true` if precision was lost, `false` otherwise).
#[must_use]
pub fn const_real_get_double(val: &ValueRef) -> (f64, bool) {
    let mut loses_info_c = 0;
    let result = unsafe { core::LLVMConstRealGetDouble(val.get_ref(), &mut loses_info_c) };
    (result, loses_info_c != 0)
}
