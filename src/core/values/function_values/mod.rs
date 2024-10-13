use super::ValueRef;
use crate::core::context::{AttributeRef, ContextRef};
use crate::core::module::ModuleRef;
use crate::core::types::TypeRef;
use crate::{CStr, CString, CUint, GetRef, SizeT};
use llvm_sys::core;

pub mod function_parameters;

/// Removes a function from its containing module and deallocates it.
///
/// This function wraps the `LLVMDeleteFunction` function from the LLVM core library. It removes the function
/// represented by `val` from its parent module and deallocates all associated resources. After calling this
/// function, the `ValueRef` should no longer be used, as it references deallocated memory.
///
/// # Safety
///
/// - The `ValueRef` must represent a valid function within a module.
/// - After deletion, the `ValueRef` becomes invalid and should not be used.
pub fn delete_function(val: &ValueRef) {
    unsafe {
        core::LLVMDeleteFunction(val.0);
    }
}

/// Checks whether the given function has an associated personality function.
///
/// This function wraps the `LLVMHasPersonalityFn` function from the LLVM core library. A personality function
/// is used in exception handling to provide language-specific semantics for stack unwinding.
///
/// # Parameters
///
/// - `val`: The `ValueRef` representing the function to check.
///
/// # Returns
///
/// Returns `true` if the function has an associated personality function, or `false` otherwise.
#[must_use]
pub fn has_personality_fn(val: &ValueRef) -> bool {
    unsafe { core::LLVMHasPersonalityFn(val.0) != 0 }
}

/// Retrieves the personality function attached to the function.
///
/// This function wraps the `LLVMGetPersonalityFn` function from the LLVM core library. The personality
/// function is used in exception handling to provide language-specific semantics for stack unwinding.
///
/// # Parameters
///
/// - `val`: The `ValueRef` representing the function.
///
/// # Returns
///
/// Returns an `Option<ValueRef>` representing the personality function if one is attached to the function,
/// or `None` if no personality function is present.
#[must_use]
pub fn get_personality_fn(val: &ValueRef) -> Option<ValueRef> {
    unsafe {
        let personality_fn = core::LLVMGetPersonalityFn(val.0);
        if personality_fn.is_null() {
            None
        } else {
            Some(ValueRef(personality_fn))
        }
    }
}

/// Sets the personality function attached to the function.
///
/// This function wraps the `LLVMSetPersonalityFn` function from the LLVM core library. The personality
/// function is used in exception handling to provide language-specific semantics for stack unwinding. By
/// setting a personality function, you define how exceptions are handled within the function.
///
/// # Parameters
///
/// - `val`: The `ValueRef` representing the function.
/// - `personality_fn`: A reference to the `ValueRef` representing the personality function to attach.
pub fn set_personality_fn(val: &ValueRef, personality_fn: &ValueRef) {
    unsafe {
        core::LLVMSetPersonalityFn(val.0, personality_fn.0);
    }
}

/// Looks up the intrinsic ID number that matches the given function name within the module.
///
/// This function wraps the `LLVMLookupIntrinsicID` function from the LLVM core library. It searches for an intrinsic
/// function within the LLVM module represented by `m` that matches the specified name. Intrinsic functions are
/// special functions provided by LLVM that perform specific operations at a lower level.
///
/// # Parameters
///
/// - `m`: The `ModuleRef` representing the LLVM module.
/// - `name`: The name (`&str`) of the intrinsic function to look up.
///
/// # Returns
///
/// Returns an `Option<u32>` containing the intrinsic ID if a matching intrinsic is found, or `None` if no
/// matching intrinsic exists within the module.
#[must_use]
pub fn lookup_intrinsic_id(name: &str) -> Option<u32> {
    let c_string = CString::from(name);
    unsafe {
        let id =
            core::LLVMLookupIntrinsicID(c_string.as_ptr(), *SizeT::from(c_string.count_bytes()));
        if id == 0 {
            None
        } else {
            Some(id)
        }
    }
}

/// Retrieves the intrinsic ID number from a function instance.
///
/// This function wraps the `LLVMGetIntrinsicID` function from the LLVM core library. Intrinsic functions in
/// LLVM have unique ID numbers that can be used to identify and categorize them.
///
/// # Parameters
///
/// - `val`: The `ValueRef` representing the function.
///
/// # Returns
///
/// Returns a `u32` representing the intrinsic ID of the function. If the function is not an intrinsic, the ID
/// returned will be `0`.
#[must_use]
pub fn get_intrinsic_id(val: &ValueRef) -> u32 {
    unsafe { core::LLVMGetIntrinsicID(val.0) }
}

/// Creates or inserts the declaration of an intrinsic function within the module.
///
/// This function wraps the `LLVMGetIntrinsicDeclaration` function from the LLVM core library. It either retrieves
/// an existing declaration of the specified intrinsic or creates a new one if it does not already exist. For
/// overloaded intrinsics, parameter types must be provided to uniquely identify the desired overload.
///
/// # Parameters
///
/// - `m`: The `ModuleRef` representing the LLVM module.
/// - `id`: The intrinsic ID (`u32`) corresponding to the desired intrinsic function.
/// - `param_types`: A slice of LLVM type references (`&[LLVMTypeRef]`) representing the parameter types of the intrinsic.
///                  This is necessary for overloaded intrinsics to uniquely identify the correct version.
///
/// # Returns
///
/// Returns a `ValueRef` representing the intrinsic function declaration within the module. If the declaration
/// cannot be created or retrieved, the returned `ValueRef` may be null, so users should ensure that the
/// declaration was successfully obtained.
#[must_use]
pub fn get_intrinsic_declaration(m: &ModuleRef, id: u32, param_types: &[TypeRef]) -> ValueRef {
    let param_types_ptr = crate::to_mut_ptr!(param_types);
    unsafe {
        let intrinsic = core::LLVMGetIntrinsicDeclaration(
            m.get_ref(),
            *CUint::from(id),
            param_types_ptr,
            *SizeT::from(param_types.len()),
        );
        ValueRef(intrinsic)
    }
}

/// Retrieves the type of intrinsic. For overloaded intrinsics, parameter
/// types must be provided to uniquely identify an overload.
///
/// This function wraps the `LLVMIntrinsicGetType` function from the LLVM core library. It obtains the
/// LLVM type (`LLVMTypeRef`) of the intrinsic identified by `id` within the given context. For overloaded
/// intrinsics, providing the correct parameter types ensures that the correct type is retrieved.
///
/// # Parameters
///
/// - `ctx`: The `LLVMContextRef` representing the LLVM context.
/// - `id`: The intrinsic ID (`u32`) corresponding to the desired intrinsic function.
/// - `param_types`: A slice of LLVM type references (`&[LLVMTypeRef]`) representing the parameter types of the intrinsic.
///                  This is necessary for overloaded intrinsics to uniquely identify the correct version.
///
/// # Returns
///
/// Returns an `LLVMTypeRef` representing the type of the intrinsic. If the intrinsic does not exist or the
/// parameter types do not match any overload, the returned type may be null.
#[must_use]
pub fn intrinsic_get_type(ctx: &ContextRef, id: u32, param_types: &[TypeRef]) -> TypeRef {
    let param_types_ptr = crate::to_mut_ptr!(param_types);
    unsafe {
        let type_ref = core::LLVMIntrinsicGetType(
            ctx.get_ref(),
            *CUint::from(id),
            param_types_ptr,
            *SizeT::from(param_types.len()),
        );
        TypeRef::from(type_ref)
    }
}

/// Retrieves the name of an intrinsic.
///
/// This function wraps the `LLVMIntrinsicGetName` function from the LLVM core library. It obtains the name of
/// the intrinsic identified by `id`.
///
/// # Parameters
///
/// - `id`: The intrinsic ID (`u32`) corresponding to the desired intrinsic function.
/// - `name_length`: A mutable reference to a `usize` where the length of the name will be stored.
///
/// # Returns
///
/// Returns a raw pointer to a null-terminated string (`*const i8`) representing the name of the intrinsic.
/// The length of the name is stored in `name_length`. The returned string should not be modified or freed by
/// the caller.
pub fn intrinsic_get_name(id: u32, name_length: &mut usize) -> *const i8 {
    unsafe { core::LLVMIntrinsicGetName(id, name_length) }
}

/// Copies the name of an overloaded intrinsic identified by a given list of
/// parameter types.
///
/// Unlike `intrinsic_get_name`, the caller is responsible for freeing the
/// returned string.
///
/// This function wraps the `LLVMIntrinsicCopyOverloadedName2` function from the LLVM core library. It retrieves
/// the name of an overloaded intrinsic based on the provided parameter types.
///
/// # Parameters
///
/// - `m`: The `ModuleRef` representing the LLVM module.
/// - `id`: The intrinsic ID (`u32`) corresponding to the desired intrinsic function.
/// - `param_types`: A slice of LLVM type references (`&[LLVMTypeRef]`) representing the parameter types of the intrinsic.
/// - `name_length`: A mutable reference to a `usize` where the length of the name will be stored.
///
/// # Returns
///
/// Returns a raw pointer to a null-terminated string (`*const i8`) representing the name of the overloaded intrinsic.
/// The length of the name is stored in `name_length`. The caller is responsible for freeing the returned string
/// using the appropriate memory deallocation function (e.g., `LLVMDisposeMessage`).
#[must_use]
pub fn intrinsic_copy_overloaded_name2(
    m: &ModuleRef,
    id: u32,
    param_types: &[TypeRef],
) -> Option<String> {
    let param_types_ptr = crate::to_mut_ptr!(param_types);
    unsafe {
        let mut length = *SizeT::from(0_usize);

        let c_str = core::LLVMIntrinsicCopyOverloadedName2(
            m.get_ref(),
            id,
            param_types_ptr,
            *SizeT::from(param_types.len()),
            &mut length,
        );
        if c_str.is_null() {
            return None;
        }
        Some(CStr::new(c_str).to_string())
    }
}

/// Determines if the intrinsic identified by the given ID is overloaded.
///
/// This function wraps the `LLVMIntrinsicIsOverloaded` function from the LLVM core library. Overloaded intrinsics
/// can have multiple versions differentiated by their parameter types.
///
/// # Parameters
///
/// - `id`: The intrinsic ID (`u32`) corresponding to the desired intrinsic function.
///
/// # Returns
///
/// Returns `true` if the intrinsic is overloaded, or `false` otherwise.
#[must_use]
pub fn intrinsic_is_overloaded(id: u32) -> bool {
    unsafe { core::LLVMIntrinsicIsOverloaded(*CUint::from(id)) != 0 }
}

/// Obtains the calling convention of a function.
///
/// The returned value corresponds to the LLVM calling convention enumeration.
///
/// This function wraps the `LLVMGetFunctionCallConv` function from the LLVM core library.
///
/// # Parameters
///
/// - `fn_val`: The `ValueRef` representing the function.
///
/// # Returns
///
/// Returns an `unsigned` integer corresponding to the LLVM calling convention of the function.
#[must_use]
pub fn get_function_call_conv(fn_val: &ValueRef) -> u32 {
    unsafe { core::LLVMGetFunctionCallConv(fn_val.0) }
}

/// Sets the calling convention of a function.
///
/// This function wraps the `LLVMSetFunctionCallConv` function from the LLVM core library.
///
/// # Parameters
///
/// - `fn_val`: The `ValueRef` representing the function.
/// - `cc`: The LLVM calling convention (`u32`) to set for the function.
pub fn set_function_call_conv(fn_val: &ValueRef, cc: u32) {
    unsafe {
        core::LLVMSetFunctionCallConv(fn_val.0, cc);
    }
}

/// Obtains the name of the garbage collector to use during code generation.
///
/// This function wraps the `LLVMGetGC` function from the LLVM core library. The garbage collector name
/// specifies which garbage collection strategy to use for the function.
///
/// # Parameters
///
/// - `fn_val`: The `ValueRef` representing the function.
///
/// # Returns
///
/// Returns a raw pointer to a null-terminated string (`*const i8`) representing the name of the garbage collector.
/// If no garbage collector is specified, it may return a null pointer.
///
/// # Safety
///
/// The returned string should not be modified or freed by the caller.
#[must_use]
pub fn get_gc(fn_val: &ValueRef) -> Option<String> {
    unsafe {
        let c_str = core::LLVMGetGC(fn_val.0);
        if c_str.is_null() {
            return None;
        }
        Some(CStr::new(c_str).to_string())
    }
}

/// Defines the garbage collector to use during code generation.
///
/// This function wraps the `LLVMSetGC` function from the LLVM core library. It sets the name of the garbage collector
/// to be used for the specified function during code generation.
///
/// # Parameters
///
/// - `fn_val`: The `ValueRef` representing the function.
/// - `name`: The name (`&str`) of the garbage collector to set.
///
/// # Safety
///
/// - The `ValueRef` must represent a valid function within a module.
/// - The `name` must not contain null bytes.
///
/// # Example
///
/// ```rust
/// set_gc(function, "my_gc");
/// ```
pub fn set_gc(fn_val: &ValueRef, name: &str) {
    let c_string = CString::from(name);
    unsafe {
        core::LLVMSetGC(fn_val.0, c_string.as_ptr());
    }
}

/// Retrieves the prefix data associated with a function.
///
/// This function wraps the `LLVMGetPrefixData` function from the LLVM core library. It obtains the prefix data
/// attached to the specified function. Prefix data is used to attach additional information to functions.
///
/// # Parameters
///
/// - `fn_val`: The `ValueRef` representing the function.
///
/// # Returns
///
/// Returns an `Option<ValueRef>` containing the prefix data if it exists, or `None` otherwise.
///
/// # Safety
///
/// - The `ValueRef` must represent a valid function within a module.
/// - The returned `ValueRef` should not be used if it is `None`.
#[must_use]
pub fn get_prefix_data(fn_val: &ValueRef) -> Option<ValueRef> {
    unsafe {
        let prefix_data = core::LLVMGetPrefixData(fn_val.0);
        if prefix_data.is_null() {
            None
        } else {
            Some(ValueRef(prefix_data))
        }
    }
}

/// Checks if a given function has prefix data.
///
/// This function wraps the `LLVMHasPrefixData` function from the LLVM core library. It determines whether
/// the specified function has prefix data attached.
///
/// # Parameters
///
/// - `fn_val`: The `ValueRef` representing the function.
///
/// # Returns
///
/// Returns `true` if the function has prefix data, or `false` otherwise.
///
/// # Safety
///
/// - The `ValueRef` must represent a valid function within a module.
#[must_use]
pub fn has_prefix_data(fn_val: &ValueRef) -> bool {
    unsafe { core::LLVMHasPrefixData(fn_val.0) != 0 }
}

/// Sets the prefix data for the function.
///
/// This function wraps the `LLVMSetPrefixData` function from the LLVM core library. It attaches prefix data
/// to the specified function.
///
/// # Parameters
///
/// - `fn_val`: The `ValueRef` representing the function.
/// - `prefix_data`: The `ValueRef` representing the prefix data to attach.
///
/// # Safety
///
/// - The `ValueRef` must represent a valid function within a module.
/// - The `prefix_data` must represent valid prefix data.
pub fn set_prefix_data(fn_val: &ValueRef, prefix_data: &ValueRef) {
    unsafe {
        core::LLVMSetPrefixData(fn_val.0, prefix_data.0);
    }
}

/// Retrieves the prologue data associated with a function.
///
/// This function wraps the `LLVMGetPrologueData` function from the LLVM core library. It obtains the prologue data
/// attached to the specified function. Prologue data is used to attach additional information to functions.
///
/// # Parameters
///
/// - `fn_val`: The `ValueRef` representing the function.
///
/// # Returns
///
/// Returns an `Option<ValueRef>` containing the prologue data if it exists, or `None` otherwise.
///
/// # Safety
///
/// - The `ValueRef` must represent a valid function within a module.
/// - The returned `ValueRef` should not be used if it is `None`.
#[must_use]
pub fn get_prologue_data(fn_val: &ValueRef) -> Option<ValueRef> {
    unsafe {
        let prologue_data = core::LLVMGetPrologueData(fn_val.0);
        if prologue_data.is_null() {
            None
        } else {
            Some(ValueRef(prologue_data))
        }
    }
}

/// Checks if a given function has prologue data.
///
/// This function wraps the `LLVMHasPrologueData` function from the LLVM core library. It determines whether
/// the specified function has prologue data attached.
///
/// # Parameters
///
/// - `fn_val`: The `ValueRef` representing the function.
///
/// # Returns
///
/// Returns `true` if the function has prologue data, or `false` otherwise.
///
/// # Safety
///
/// - The `ValueRef` must represent a valid function within a module.
#[must_use]
pub fn has_prologue_data(fn_val: &ValueRef) -> bool {
    unsafe { core::LLVMHasPrologueData(fn_val.0) != 0 }
}

/// Sets the prologue data for the function.
///
/// This function wraps the `LLVMSetPrologueData` function from the LLVM core library. It attaches prologue data
/// to the specified function.
///
/// # Parameters
///
/// - `fn_val`: The `ValueRef` representing the function.
/// - `prologue_data`: The `ValueRef` representing the prologue data to attach.
///
/// # Safety
///
/// - The `ValueRef` must represent a valid function within a module.
/// - The `prologue_data` must represent valid prologue data.
pub fn set_prologue_data(fn_val: &ValueRef, prologue_data: &ValueRef) {
    unsafe {
        core::LLVMSetPrologueData(fn_val.0, prologue_data.0);
    }
}

/// Adds an attribute to a function at a specified index.
///
/// This function wraps the `LLVMAddAttributeAtIndex` function from the LLVM core library. Attributes provide
/// additional metadata about functions, such as optimization hints or specific calling conventions.
///
/// # Parameters
///
/// - `fn_val`: The `ValueRef` representing the function.
/// - `idx`: The `LLVMAttributeIndex` indicating where to add the attribute (e.g., function attributes, return attributes).
/// - `attr`: The `LLVMAttributeRef` representing the attribute to add.
///
/// # Safety
///
/// - The `ValueRef` must represent a valid function within a module.
/// - The `LLVMAttributeRef` must represent a valid attribute.
pub fn add_attribute_at_index(fn_val: &ValueRef, idx: u32, attr: &AttributeRef) {
    unsafe {
        core::LLVMAddAttributeAtIndex(fn_val.0, *CUint::from(idx), attr.get_ref());
    }
}

/// Retrieves the number of attributes at a specified index for a function.
///
/// This function wraps the `LLVMGetAttributeCountAtIndex` function from the LLVM core library. It returns the
/// number of attributes present at the specified index.
///
/// # Parameters
///
/// - `fn_val`: The `ValueRef` representing the function.
/// - `idx`: The `LLVMAttributeIndex` indicating which attribute set to query.
///
/// # Returns
///
/// Returns an `unsigned` integer representing the number of attributes at the specified index.
///
/// # Safety
///
/// - The `ValueRef` must represent a valid function within a module.
/// - The `LLVMAttributeIndex` must be valid for the function.
#[must_use]
pub fn get_attribute_count_at_index(fn_val: &ValueRef, idx: u32) -> u32 {
    unsafe { core::LLVMGetAttributeCountAtIndex(fn_val.0, *CUint::from(idx)) }
}

/// Retrieves all attributes at a specified index for a function.
///
/// This function wraps the `LLVMGetAttributesAtIndex` function from the LLVM core library. It fills the provided
/// slice with the attributes present at the specified index.
///
/// # Parameters
///
/// - `fn_val`: The `ValueRef` representing the function.
/// - `idx`: The `LLVMAttributeIndex` indicating which attribute set to query.
/// - `attrs`: A mutable slice of `LLVMAttributeRef` where the attributes will be stored.
///
/// # Safety
///
/// - The `ValueRef` must represent a valid function within a module.
/// - The `LLVMAttributeIndex` must be valid for the function.
/// - The `attrs` slice must be large enough to hold all attributes at the specified index.
pub fn get_attributes_at_index(fn_val: &ValueRef, idx: u32, attrs: &[AttributeRef]) {
    let attrs_ptr = crate::to_mut_ptr!(attrs);
    unsafe {
        core::LLVMGetAttributesAtIndex(fn_val.0, *CUint::from(idx), attrs_ptr);
    }
}

/// Retrieves an enum attribute at a specified index for a function.
///
/// This function wraps the `LLVMGetEnumAttributeAtIndex` function from the LLVM core library. It fetches the
/// enum attribute corresponding to the provided `KindID` at the specified index.
///
/// # Parameters
///
/// - `fn_val`: The `ValueRef` representing the function.
/// - `idx`: The `LLVMAttributeIndex` indicating where to retrieve the attribute.
/// - `kind_id`: The `unsigned` integer representing the kind of enum attribute to retrieve.
///
/// # Returns
///
/// Returns an `Option<LLVMAttributeRef>` containing the attribute if found, or `None` otherwise.
///
/// # Safety
///
/// - The `ValueRef` must represent a valid function within a module.
/// - The `KindID` must correspond to a valid enum attribute.
#[must_use]
pub fn get_enum_attribute_at_index(
    fn_val: &ValueRef,
    idx: u32,
    kind_id: u32,
) -> Option<AttributeRef> {
    unsafe {
        let attr =
            core::LLVMGetEnumAttributeAtIndex(fn_val.0, *CUint::from(idx), *CUint::from(kind_id));
        if attr.is_null() {
            None
        } else {
            Some(AttributeRef::from(attr))
        }
    }
}

/// Retrieves a string attribute at a specified index for a function.
///
/// This function wraps the `LLVMGetStringAttributeAtIndex` function from the LLVM core library. It fetches the
/// string attribute corresponding to the provided key at the specified index.
///
/// # Parameters
///
/// - `fn_val`: The `ValueRef` representing the function.
/// - `idx`: The `LLVMAttributeIndex` indicating where to retrieve the attribute.
/// - `key`: The key (`&str`) identifying the string attribute.
///
/// # Returns
///
/// Returns an `Option<LLVMAttributeRef>` containing the attribute if found, or `None` otherwise.
///
/// # Safety
///
/// - The `ValueRef` must represent a valid function within a module.
/// - The `key` must not contain null bytes.
#[must_use]
pub fn get_string_attribute_at_index(
    fn_val: &ValueRef,
    idx: u32,
    key: &str,
) -> Option<AttributeRef> {
    let c_key = CString::from(key);
    unsafe {
        let attr = core::LLVMGetStringAttributeAtIndex(
            fn_val.0,
            *CUint::from(idx),
            c_key.as_ptr(),
            *CUint::from(c_key.count_bytes()),
        );
        if attr.is_null() {
            None
        } else {
            Some(AttributeRef::from(attr))
        }
    }
}

/// Removes an enum attribute at a specified index for a function.
///
/// This function wraps the `LLVMRemoveEnumAttributeAtIndex` function from the LLVM core library. It removes the
/// enum attribute corresponding to the provided `KindID` at the specified index.
///
/// # Parameters
///
/// - `fn_val`: The `ValueRef` representing the function.
/// - `idx`: The `LLVMAttributeIndex` indicating where to remove the attribute.
/// - `kind_id`: The `unsigned` integer representing the kind of enum attribute to remove.
///
/// # Safety
///
/// - The `ValueRef` must represent a valid function within a module.
/// - The `KindID` must correspond to a valid enum attribute.
pub fn remove_enum_attribute_at_index(fn_val: &ValueRef, idx: u32, kind_id: u32) {
    unsafe {
        core::LLVMRemoveEnumAttributeAtIndex(fn_val.0, *CUint::from(idx), *CUint::from(kind_id));
    }
}

/// Removes a string attribute at a specified index for a function.
///
/// This function wraps the `LLVMRemoveStringAttributeAtIndex` function from the LLVM core library. It removes the
/// string attribute corresponding to the provided key at the specified index.
///
/// # Parameters
///
/// - `fn_val`: The `ValueRef` representing the function.
/// - `idx`: The `LLVMAttributeIndex` indicating where to remove the attribute.
/// - `key`: The key (`&str`) identifying the string attribute to remove.
///
/// # Safety
///
/// - The `ValueRef` must represent a valid function within a module.
/// - The `key` must not contain null bytes.
pub fn remove_string_attribute_at_index(fn_val: &ValueRef, idx: u32, key: &str) {
    let c_key = CString::from(key);
    unsafe {
        core::LLVMRemoveStringAttributeAtIndex(
            fn_val.0,
            *CUint::from(idx),
            c_key.as_ptr(),
            *CUint::from(c_key.count_bytes()),
        );
    }
}

/// Adds a target-dependent attribute to a function.
///
/// This function wraps the `LLVMAddTargetDependentFunctionAttr` function from the LLVM core library. It attaches
/// a target-specific attribute with a specified value to the given function. Target-dependent attributes can be
/// used to provide additional metadata or optimization hints that are specific to a particular target architecture.
///
/// # Parameters
///
/// - `fn_val`: The `ValueRef` representing the function to which the attribute will be added.
/// - `attribute`: The name (`&str`) of the attribute to add.
/// - `value`: The value (`&str`) associated with the attribute.
///
/// # Safety
///
/// - The `ValueRef` must represent a valid function within a module.
/// - The `attribute` and `value` strings must not contain null bytes.
///
pub fn add_target_dependent_function_attr(fn_val: &ValueRef, attribute: &str, value: &str) {
    let attr_cstr = CString::from(attribute);
    let value_cstr = CString::from(value);
    unsafe {
        core::LLVMAddTargetDependentFunctionAttr(fn_val.0, attr_cstr.as_ptr(), value_cstr.as_ptr());
    }
}
