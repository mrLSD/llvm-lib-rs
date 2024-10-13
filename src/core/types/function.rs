//! Functions in this section operate on function types.

use crate::core::types::TypeRef;
use crate::{CInt, CUint, GetRef};
use llvm_sys::core;
use llvm_sys::prelude::LLVMTypeRef;

/// These functions relate to `TypeRef` of `LLVMTypeRef` instances.
#[derive(Debug)]
pub struct FunctionTypeRef(LLVMTypeRef);

impl From<LLVMTypeRef> for FunctionTypeRef {
    fn from(value: LLVMTypeRef) -> Self {
        Self(value)
    }
}

impl From<FunctionTypeRef> for TypeRef {
    fn from(value: FunctionTypeRef) -> Self {
        Self(value.0)
    }
}

impl GetRef for FunctionTypeRef {
    type RawRef = LLVMTypeRef;
    fn get_ref(&self) -> Self::RawRef {
        self.0
    }
}

impl FunctionTypeRef {
    /// Obtain a function type consisting of a specified signature.
    ///
    /// The function is defined as a tuple of a return Type, a list of
    /// parameter types, and whether the function is variadic.
    ///
    /// # Details
    ///
    /// Creates a function type in LLVM IR.
    ///
    /// This function wraps the `LLVMFunctionType` function from the LLVM core library. It creates and returns
    /// a function type with the specified return type, parameter types, and whether the function accepts a variable
    /// number of arguments (varargs). Function types are essential in LLVM IR for defining the signature of functions,
    /// including the types of their return values and parameters.
    ///
    /// # Parameters
    ///
    /// - `return_type`: A reference to the `TypeRef` representing the return type of the function.
    /// - `param_types`: A slice of `TypeRef` representing the types of the function's parameters. If the function takes no parameters, this slice can be empty.
    /// - `is_var_arg`: A boolean indicating whether the function accepts a variable number of arguments. Set to `true` for varargs functions, and `false` otherwise.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` representing the function type with the specified return type, parameters, and varargs setting.
    #[must_use]
    pub fn function_type(return_type: &TypeRef, param_types: &[TypeRef], is_var_arg: bool) -> Self {
        let parameters = crate::to_mut_ptr!(param_types);
        unsafe {
            Self(core::LLVMFunctionType(
                return_type.0,
                parameters,
                *CUint::from(param_types.len()),
                *CInt::from(is_var_arg),
            ))
        }
    }

    /// Returns whether a function type is variadic.
    ///
    /// # Details
    ///
    /// Checks whether a function type is a variadic function (supports variable arguments).
    ///
    /// This function wraps the `LLVMIsFunctionVarArg` function from the LLVM core library. It determines whether
    /// the function type represented by `self` supports variable arguments (varargs). Variadic functions can accept
    /// a variable number of arguments beyond the fixed parameters defined in their signature.
    ///
    /// # Returns
    ///
    /// Returns `true` if the function type is variadic, otherwise returns `false`.
    #[must_use]
    pub fn is_function_var_arg(&self) -> bool {
        unsafe { core::LLVMIsFunctionVarArg(self.0) != 0 }
    }

    /// Obtain the Type this function Type returns.
    ///
    /// # Details
    ///
    /// Retrieves the return type of a function type in LLVM IR.
    ///
    /// This function wraps the `LLVMGetReturnType` function from the LLVM core library. It returns the `TypeRef`
    /// representing the return type of the function type associated with `self`. This is useful for inspecting the
    /// return type of a function in LLVM IR.
    ///
    /// # Returns
    ///
    /// Returns a `TypeRef` representing the return type of the function type.
    #[must_use]
    pub fn get_return_type(&self) -> TypeRef {
        unsafe { TypeRef(core::LLVMGetReturnType(self.0)) }
    }

    /// Obtain the number of parameters this function accepts.
    ///
    /// # Details
    ///
    /// Counts the number of parameter types in a function type in LLVM IR.
    ///
    /// This function wraps the `LLVMCountParamTypes` function from the LLVM core library. It returns the number
    /// of parameters that the function type associated with `self` has. This is useful for determining the arity
    /// of a function in LLVM IR.
    ///
    /// # Returns
    ///
    /// Returns a `u32` representing the number of parameter types in the function type.
    #[must_use]
    pub fn count_param_types(&self) -> u32 {
        unsafe { core::LLVMCountParamTypes(self.0) }
    }

    /// Obtain the types of a function's parameters.
    ///
    /// The Dest parameter should point to a pre-allocated array of
    /// `TypeRef` at least `count_param_types` large. On return, the
    /// first `count_param_types` entries in the array will be populated
    /// with `TypeRef` instances.
    ///
    /// # Details
    ///
    /// Retrieves the parameter types of a function type in LLVM IR.
    ///
    /// This function wraps the `LLVMGetParamTypes` function from the LLVM core library. It returns a `Vec<TypeRef>`
    /// containing the types of all parameters in the function type associated with `self`. This is useful for inspecting
    /// the types of parameters in a function signature in LLVM IR.
    ///
    /// # Returns
    ///
    /// Returns a `Vec<TypeRef>` representing the types of the parameters in the function type.
    #[must_use]
    pub fn get_param_types(&self) -> Vec<TypeRef> {
        let count = self.count_param_types() as usize;
        let mut param_types: Vec<LLVMTypeRef> = Vec::with_capacity(count);
        unsafe {
            core::LLVMGetParamTypes(self.0, param_types.as_mut_ptr());
            param_types.set_len(count);
        }
        param_types.into_iter().map(TypeRef).collect()
    }
}
