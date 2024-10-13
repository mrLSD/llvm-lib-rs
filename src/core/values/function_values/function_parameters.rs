use super::ValueRef;
use llvm_sys::core;

/// Obtains the number of parameters in a function.
///
/// This function wraps the `LLVMCountParams` function from the LLVM core library. It returns the count of parameters
/// that the specified function accepts.
///
/// # Parameters
///
/// - `fn_val`: The `ValueRef` representing the function.
///
/// # Returns
///
/// Returns an `unsigned` integer (`u32`) indicating the number of parameters in the function.
///
/// # Safety
///
/// - The `ValueRef` must represent a valid function within a module.
#[must_use]
pub fn count_params(fn_val: &ValueRef) -> u32 {
    unsafe { core::LLVMCountParams(fn_val.0) }
}

/// Retrieves the parameters of a function.
///
/// This function wraps the `LLVMGetParams` function from the LLVM core library. It fills the provided mutable slice
/// with the parameters of the specified function. Each parameter is represented as a `ValueRef`.
///
/// # Parameters
///
/// - `fn_val`: The `ValueRef` representing the function.
/// - `params`: A mutable slice of `ValueRef` where the function's parameters will be stored.
///
/// # Safety
///
/// - The `ValueRef` must represent a valid function within a module.
/// - The `params` slice must be pre-allocated and have a length of at least `count_params(fn_val)`.
pub fn get_params(fn_val: &ValueRef, params: &mut &[ValueRef]) {
    let params_ptr = crate::to_mut_ptr!(params);
    unsafe {
        core::LLVMGetParams(fn_val.0, params_ptr);
    }
}

/// Retrieves a specific parameter of a function by index.
///
/// This function wraps the `LLVMGetParam` function from the LLVM core library. It returns the parameter at the
/// specified index within the function's parameter list.
///
/// # Parameters
///
/// - `fn_val`: The `ValueRef` representing the function.
/// - `index`: The zero-based index of the parameter to retrieve.
///
/// # Returns
///
/// Returns an `Option<ValueRef>` containing the parameter if the index is valid, or `None` if the index is out of bounds.
///
/// # Safety
///
/// - The `ValueRef` must represent a valid function within a module.
/// - `index` must be less than the number of parameters in the function.
#[must_use]
pub fn get_param(fn_val: &ValueRef, index: u32) -> Option<ValueRef> {
    unsafe {
        let param = core::LLVMGetParam(fn_val.0, index);
        if param.is_null() {
            None
        } else {
            Some(ValueRef(param))
        }
    }
}

/// Retrieves the parent function of a given argument.
///
/// This function wraps the `LLVMGetParamParent` function from the LLVM core library. It returns the function to
/// which the specified argument belongs.
///
/// # Parameters
///
/// - `arg`: The `ValueRef` representing the argument.
///
/// # Returns
///
/// Returns an `Option<ValueRef>` containing the parent function if it exists, or `None` otherwise.
///
/// # Safety
///
/// - The `ValueRef` must represent a valid argument within a function.
#[must_use]
pub fn get_param_parent(arg: &ValueRef) -> Option<ValueRef> {
    unsafe {
        let parent = core::LLVMGetParamParent(arg.0);
        if parent.is_null() {
            None
        } else {
            Some(ValueRef(parent))
        }
    }
}

/// Retrieves the first parameter of a function.
///
/// This function wraps the `LLVMGetFirstParam` function from the LLVM core library. It returns the first parameter
/// of the specified function.
///
/// # Parameters
///
/// - `fn_val`: The `ValueRef` representing the function.
///
/// # Returns
///
/// Returns an `Option<ValueRef>` containing the first parameter if it exists, or `None` if the function has no parameters.
///
/// # Safety
///
/// - The `ValueRef` must represent a valid function within a module.
#[must_use]
pub fn get_first_param(fn_val: &ValueRef) -> Option<ValueRef> {
    unsafe {
        let first = core::LLVMGetFirstParam(fn_val.0);
        if first.is_null() {
            None
        } else {
            Some(ValueRef(first))
        }
    }
}

/// Retrieves the last parameter of a function.
///
/// This function wraps the `LLVMGetLastParam` function from the LLVM core library. It returns the last parameter
/// of the specified function.
///
/// # Parameters
///
/// - `fn_val`: The `ValueRef` representing the function.
///
/// # Returns
///
/// Returns an `Option<ValueRef>` containing the last parameter if it exists, or `None` if the function has no parameters.
///
/// # Safety
///
/// - The `ValueRef` must represent a valid function within a module.
#[must_use]
pub fn get_last_param(fn_val: &ValueRef) -> Option<ValueRef> {
    unsafe {
        let last = core::LLVMGetLastParam(fn_val.0);
        if last.is_null() {
            None
        } else {
            Some(ValueRef(last))
        }
    }
}

/// Retrieves the next parameter in a function's parameter list.
///
/// This function wraps the `LLVMGetNextParam` function from the LLVM core library. Given a current parameter, it returns
/// the next parameter in the function's parameter list.
///
/// # Parameters
///
/// - `arg`: The `ValueRef` representing the current parameter.
///
/// # Returns
///
/// Returns an `Option<ValueRef>` containing the next parameter if it exists, or `None` if there is no next parameter.
///
/// # Safety
///
/// - The `ValueRef` must represent a valid parameter within a function.
#[must_use]
pub fn get_next_param(arg: &ValueRef) -> Option<ValueRef> {
    unsafe {
        let next = core::LLVMGetNextParam(arg.0);
        if next.is_null() {
            None
        } else {
            Some(ValueRef(next))
        }
    }
}

/// Retrieves the previous parameter in a function's parameter list.
///
/// This function wraps the `LLVMGetPreviousParam` function from the LLVM core library. Given a current parameter, it returns
/// the previous parameter in the function's parameter list.
///
/// # Parameters
///
/// - `arg`: The `ValueRef` representing the current parameter.
///
/// # Returns
///
/// Returns an `Option<ValueRef>` containing the previous parameter if it exists, or `None` if there is no previous parameter.
///
/// # Safety
///
/// - The `ValueRef` must represent a valid parameter within a function.
#[must_use]
pub fn get_previous_param(arg: &ValueRef) -> Option<ValueRef> {
    unsafe {
        let previous = core::LLVMGetPreviousParam(arg.0);
        if previous.is_null() {
            None
        } else {
            Some(ValueRef(previous))
        }
    }
}

/// Sets the alignment for a function parameter.
///
/// This function wraps the `LLVMSetParamAlignment` function from the LLVM core library. It sets the alignment requirement
/// for the specified function parameter.
///
/// # Parameters
///
/// - `arg`: The `ValueRef` representing the parameter.
/// - `align`: The alignment (`unsigned`, typically `u32`) to set for the parameter.
///
/// # Safety
///
/// - The `ValueRef` must represent a valid parameter within a function.
/// - `align` must be a valid alignment value as per LLVM's requirements.
pub fn set_param_alignment(arg: &ValueRef, align: u32) {
    unsafe {
        core::LLVMSetParamAlignment(arg.0, align);
    }
}
