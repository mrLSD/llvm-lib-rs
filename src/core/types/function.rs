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
    #[must_use]
    pub fn function_type(return_type: &TypeRef, param_types: &[TypeRef], is_var_arg: bool) -> Self {
        let mut param_types = param_types.iter().map(|v| v.0).collect::<Vec<_>>();
        let parameters = if param_types.is_empty() {
            std::ptr::null_mut()
        } else {
            param_types.as_mut_ptr()
        };
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
    #[must_use]
    pub fn is_function_var_arg(&self) -> bool {
        unsafe { core::LLVMIsFunctionVarArg(self.0) != 0 }
    }

    /// Obtain the Type this function Type returns.
    #[must_use]
    pub fn get_return_type(&self) -> TypeRef {
        unsafe { TypeRef(core::LLVMGetReturnType(self.0)) }
    }

    /// Obtain the number of parameters this function accepts.
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
