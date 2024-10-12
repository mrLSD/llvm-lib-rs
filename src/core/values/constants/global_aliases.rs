use super::ValueRef;
use crate::core::module::ModuleRef;
use crate::core::types::TypeRef;
use crate::core::AddressSpace;
use crate::{CString, GetRef};
use llvm_sys::core;

/// Adds a `GlobalAlias` to the module.
///
/// This function wraps the `LLVMAddAlias2` function from the LLVM core library. It creates a new global alias within
/// the LLVM module represented by [`ModuleRef`]. A global alias is an LLVM construct that allows one global value to alias
/// another, effectively creating an alternative name for the aliasee.
///
/// # Parameters
///
/// - `value_ty`: The LLVM type (`LLVMTypeRef`) of the alias.
/// - `addr_space`: The address space (`u32`) where the alias resides.
/// - `aliasee`: A reference to the [`ValueRef`] that the alias will point to.
/// - `name`: The name (`&str`) of the alias.
///
/// # Returns
///
/// Returns a `ValueRef` representing the newly created `GlobalAlias`. If the creation fails, the returned
/// `ValueRef` may be null, so users should ensure that the alias was created successfully.
#[must_use]
pub fn add_alias2(
    module: &ModuleRef,
    value_ty: &TypeRef,
    addr_space: &AddressSpace,
    aliasee: &ValueRef,
    name: &str,
) -> ValueRef {
    let c_string = CString::from(name);
    unsafe {
        let alias = core::LLVMAddAlias2(
            module.get_ref(),
            value_ty.get_ref(),
            ***addr_space,
            aliasee.get_ref(),
            c_string.as_ptr(),
        );
        ValueRef::from(alias)
    }
}

/// Retrieves a `GlobalAlias` by its name.
///
/// This function wraps the `LLVMGetNamedGlobalAlias` function from the LLVM core library. It searches the LLVM module
/// represented by [`ModuleRef`] for a global alias with the specified name.
///
/// # Parameters
///
/// - `name`: The name (`&str`) of the alias to retrieve.
///
/// # Returns
///
/// Returns an `Option<ValueRef>` which is `Some(ValueRef)` if an alias with the given name exists, or `None` if
/// no such alias is found within the module.
#[must_use]
pub fn get_named_global_alias(module: &ModuleRef, name: &str) -> Option<ValueRef> {
    let c_string = CString::from(name);
    unsafe {
        let alias = core::LLVMGetNamedGlobalAlias(
            module.get_ref(),
            c_string.as_ptr(),
            c_string.as_bytes().len(),
        );
        if alias.is_null() {
            None
        } else {
            Some(ValueRef::from(alias))
        }
    }
}

/// Returns an iterator to the first `GlobalAlias` in the module.
///
/// This function wraps the `LLVMGetFirstGlobalAlias` function from the LLVM core library. It initializes an iterator
/// that starts at the first global alias within the LLVM module represented by [`ModuleRef`].
///
/// # Returns
///
/// Returns a `ValueRef` that can be used to traverse the global aliases in the module.
#[must_use]
pub fn get_first_global_alias(module: &ModuleRef) -> ValueRef {
    let val = unsafe { core::LLVMGetFirstGlobalAlias(module.get_ref()) };
    ValueRef::from(val)
}

/// Returns an iterator to the last `GlobalAlias` in the module.
///
/// This function wraps the `LLVMGetLastGlobalAlias` function from the LLVM core library. It initializes an iterator
/// that starts at the last global alias within the LLVM module represented by [`ModuleRef`].
///
/// # Returns
///
/// Returns a `ValueRef` that can be used to traverse the global aliases in the module in reverse order.
#[must_use]
pub fn get_last_global_alias(module: &ModuleRef) -> ValueRef {
    let val = unsafe { core::LLVMGetLastGlobalAlias(module.get_ref()) };
    ValueRef::from(val)
}

/// Retrieves the next `GlobalAlias` in the module.
///
/// This function wraps the `LLVMGetNextGlobalAlias` function from the LLVM core library. It advances the iterator
/// to the next global alias relative to the current alias represented by [`ValueRef`].
///
/// # Returns
///
/// Returns an `Option<ValueRef>` which is `Some(ValueRef)` if there is a next alias, or `None` if the current
/// alias is the last one in the module.
#[must_use]
pub fn get_next_global_alias(val: &ValueRef) -> Option<ValueRef> {
    unsafe {
        let next = core::LLVMGetNextGlobalAlias(val.get_ref());
        if next.is_null() {
            None
        } else {
            Some(ValueRef(next))
        }
    }
}

/// Retrieves the previous `GlobalAlias` in the module.
///
/// This function wraps the `LLVMGetPreviousGlobalAlias` function from the LLVM core library. It moves the iterator
/// to the previous global alias relative to the current alias represented by `ValueRef`.
///
/// # Returns
///
/// Returns an `Option<ValueRef>` which is `Some(ValueRef)` if there is a previous alias, or `None` if the current
/// alias is the first one in the module.
#[must_use]
pub fn get_previous_global_alias(val: &ValueRef) -> Option<ValueRef> {
    unsafe {
        let prev = core::LLVMGetPreviousGlobalAlias(val.0);
        if prev.is_null() {
            None
        } else {
            Some(ValueRef(prev))
        }
    }
}

/// Retrieves the aliasee of this `GlobalAlias`.
///
/// This function wraps the `LLVMAliasGetAliasee` function from the LLVM core library. It obtains the value that
/// the alias represented by `ValueRef` is pointing to. The aliasee is typically another global value within the LLVM
/// module.
///
/// # Returns
///
/// Returns a `ValueRef` representing the aliasee of the alias. If the alias does not have a valid aliasee, the
/// returned `ValueRef` may be null.
#[must_use]
pub fn alias_get_aliasee(val: &ValueRef) -> ValueRef {
    unsafe { ValueRef(core::LLVMAliasGetAliasee(val.0)) }
}

/// Sets the aliasee for an alias global value.
///
/// This function wraps the `LLVMAliasSetAliasee` function from the LLVM core library. It assigns a new aliasee
/// to the alias represented by `ValueRef≠`. An aliasee is the value that the alias points to, typically another global
/// value. By setting a new aliasee, you are changing the target of the alias.
///
/// # Parameters
///
/// - `new_aliasee`: A reference to the new global value (`ValueRef`) that the alias will point to.
pub fn alias_set_aliasee(val: &ValueRef, new_aliasee: &ValueRef) {
    unsafe {
        core::LLVMAliasSetAliasee(val.0, new_aliasee.0);
    }
}
