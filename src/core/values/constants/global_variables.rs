use super::{TypeRef, ValueRef};
use crate::core::module::ModuleRef;
use crate::core::AddressSpace;
use crate::{CInt, CString, GetRef};
use llvm_sys::{core, LLVMThreadLocalMode};

/// Represents the thread-local storage (TLS) model for a global variable in LLVM.
///
/// Thread-local storage allows each thread to have its own instance of a global variable. The different TLS models
/// dictate how the runtime handles accessing the variable across threads and whether the variable is accessed
/// dynamically or statically.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThreadLocalMode {
    /// The global variable is not thread-local.
    NotThreadLocal,
    /// General dynamic TLS model, suitable for global variables that are accessed across
    /// multiple modules and can be dynamically allocated for each thread.
    GeneralDynamicTLSModel,
    /// Local dynamic TLS model, suitable for global variables that are dynamically allocated
    /// but only accessed within the same module.
    LocalDynamicTLSModel,
    /// Initial execution TLS model, allowing for faster access to TLS variables when they are
    /// known to be used early during program execution (such as in dynamic libraries).
    InitialExecTLSModel,
    /// Local execution TLS model, providing fast access to thread-local variables that are
    /// only accessed within the current module, without requiring relocation.
    LocalExecTLSModel,
}

impl From<LLVMThreadLocalMode> for ThreadLocalMode {
    fn from(mode: LLVMThreadLocalMode) -> Self {
        match mode {
            LLVMThreadLocalMode::LLVMNotThreadLocal => Self::NotThreadLocal,
            LLVMThreadLocalMode::LLVMGeneralDynamicTLSModel => Self::GeneralDynamicTLSModel,
            LLVMThreadLocalMode::LLVMLocalDynamicTLSModel => Self::LocalDynamicTLSModel,
            LLVMThreadLocalMode::LLVMInitialExecTLSModel => Self::InitialExecTLSModel,
            LLVMThreadLocalMode::LLVMLocalExecTLSModel => Self::LocalExecTLSModel,
        }
    }
}

impl From<ThreadLocalMode> for LLVMThreadLocalMode {
    fn from(mode: ThreadLocalMode) -> Self {
        match mode {
            ThreadLocalMode::NotThreadLocal => Self::LLVMNotThreadLocal,
            ThreadLocalMode::GeneralDynamicTLSModel => Self::LLVMGeneralDynamicTLSModel,
            ThreadLocalMode::LocalDynamicTLSModel => Self::LLVMLocalDynamicTLSModel,
            ThreadLocalMode::InitialExecTLSModel => Self::LLVMInitialExecTLSModel,
            ThreadLocalMode::LocalExecTLSModel => Self::LLVMLocalExecTLSModel,
        }
    }
}

/// Adds a new global variable of the specified type to the module.
///
/// This function wraps the `LLVMAddGlobal` function from the LLVM core library. It creates a new global variable
/// in the current module, with the provided type and name. The global variable is initialized with a null value by default
/// and can be further configured using other methods such as setting an initializer or modifying its linkage.
///
/// # Parameters
///
/// - `ty`: A reference to the `TypeRef` representing the type of the global variable.
/// - `name`: A string slice (`&str`) representing the name of the global variable.
///
/// # Returns
///
/// Returns a `ValueRef` representing the newly added global variable.
#[must_use]
pub fn add_global(m: &ModuleRef, ty: &TypeRef, name: &str) -> ValueRef {
    let c_name = CString::from(name);
    unsafe {
        ValueRef(core::LLVMAddGlobal(
            m.get_ref(),
            ty.get_ref(),
            c_name.as_ptr(),
        ))
    }
}

/// Adds a new global variable of the specified type to the module in a specific address space.
///
/// This function wraps the `LLVMAddGlobalInAddressSpace` function from the LLVM core library. It creates a new global
/// variable in the specified address space within the current module, with the provided type and name. Address spaces
/// are used in LLVM to specify different memory regions for global variables, such as GPU memory or specialized
/// hardware regions.
///
/// # Parameters
///
/// - `ty`: A reference to the `TypeRef` representing the type of the global variable.
/// - `name`: A string slice (`&str`) representing the name of the global variable.
/// - `address_space`: A reference to the `AddressSpace` where the global variable should be allocated.
///
/// # Returns
///
/// Returns a `ValueRef` representing the newly added global variable in the specified address space.
#[must_use]
pub fn add_global_in_address_space(
    m: &ModuleRef,
    ty: &TypeRef,
    name: &str,
    address_space: &AddressSpace,
) -> ValueRef {
    let c_name = CString::from(name);
    unsafe {
        ValueRef(core::LLVMAddGlobalInAddressSpace(
            m.get_ref(),
            ty.get_ref(),
            c_name.as_ptr(),
            ***address_space,
        ))
    }
}

/// Retrieves a global variable by its name from the module.
///
/// This function wraps the `LLVMGetNamedGlobal` function from the LLVM core library. It searches for a global
/// variable with the specified name in the current module and returns it if found. If no global variable with the
/// given name exists in the module, it returns `None`.
///
/// # Parameters
///
/// - `name`: A string slice (`&str`) representing the name of the global variable to search for.
///
/// # Returns
///
/// Returns an `Option<ValueRef>`:
/// - `Some(ValueRef)` if a global variable with the specified name is found.
/// - `None` if no global variable with the specified name exists in the module.
#[must_use]
pub fn get_named_global(m: &ModuleRef, name: &str) -> Option<ValueRef> {
    let c_name = CString::from(name);
    let global = unsafe { core::LLVMGetNamedGlobal(m.get_ref(), c_name.as_ptr()) };
    if global.is_null() {
        None
    } else {
        Some(ValueRef(global))
    }
}

/// Retrieves the first global variable defined in the module.
///
/// This function wraps the `LLVMGetFirstGlobal` function from the LLVM core library. It returns the first global
/// variable in the current module, which can be useful for iterating through all global variables in the module.
///
/// # Returns
///
/// Returns an `Option<ValueRef>`:
/// - `Some(ValueRef)` if the module contains at least one global variable.
/// - `None` if the module does not have any global variables.
#[must_use]
pub fn get_first_global(m: &ModuleRef) -> Option<ValueRef> {
    let global = unsafe { core::LLVMGetFirstGlobal(m.get_ref()) };
    if global.is_null() {
        None
    } else {
        Some(ValueRef(global))
    }
}

/// Retrieves the last global variable defined in the module.
///
/// This function wraps the `LLVMGetLastGlobal` function from the LLVM core library. It returns the last global
/// variable in the current module, which can be useful for iterating through all global variables or accessing the
/// most recently defined one.
///
/// # Returns
///
/// Returns an `Option<ValueRef>`:
/// - `Some(ValueRef)` if the module contains at least one global variable.
/// - `None` if the module does not have any global variables.
#[must_use]
pub fn get_last_global(m: &ModuleRef) -> Option<ValueRef> {
    let global = unsafe { core::LLVMGetLastGlobal(m.get_ref()) };
    if global.is_null() {
        None
    } else {
        Some(ValueRef(global))
    }
}

/// Retrieves the next global variable following the current one in the module.
///
/// This function wraps the `LLVMGetNextGlobal` function from the LLVM core library. It returns the global variable
/// that comes after the current global variable in the module. This is useful for iterating through all global variables
/// in a module.
///
/// # Returns
///
/// Returns an `Option<ValueRef>`:
/// - `Some(ValueRef)` if there is another global variable following the current one.
/// - `None` if the current global variable is the last one in the module.
#[must_use]
pub fn get_next_global(val: &ValueRef) -> Option<ValueRef> {
    let global = unsafe { core::LLVMGetNextGlobal(val.get_ref()) };
    if global.is_null() {
        None
    } else {
        Some(ValueRef(global))
    }
}

/// Retrieves the previous global variable preceding the current one in the module.
///
/// This function wraps the `LLVMGetPreviousGlobal` function from the LLVM core library. It returns the global variable
/// that comes before the current global variable in the module. This is useful for iterating backward through all global
/// variables in a module.
///
/// # Returns
///
/// Returns an `Option<ValueRef>`:
/// - `Some(ValueRef)` if there is a global variable preceding the current one.
/// - `None` if the current global variable is the first one in the module.
#[must_use]
pub fn get_previous_global(val: &ValueRef) -> Option<ValueRef> {
    let global = unsafe { core::LLVMGetPreviousGlobal(val.get_ref()) };
    if global.is_null() {
        None
    } else {
        Some(ValueRef(global))
    }
}

/// Deletes the specified global variable.
///
/// This function wraps the `LLVMDeleteGlobal` function from the LLVM core library. It removes the global variable
/// represented by `ValueRef` from the module and deletes it. After this function is called, the global variable is no
/// longer valid and cannot be used.
///
/// # Note
///
/// Once a global variable is deleted, it cannot be accessed or modified. Be cautious when deleting global variables
/// to ensure that there are no further references to them.
///
/// # Example
/// ```rust
/// let global_var = module.add_global(&int32_type, "my_global");
/// global_var.delete_global();  // Deletes the global variable
/// ```
pub fn delete_global(val: &ValueRef) {
    unsafe {
        core::LLVMDeleteGlobal(val.get_ref());
    }
}

/// Get the initializer for a global variable.
///
/// This function wraps the `LLVMGetInitializer` function from the LLVM core library. It returns the initializer of the
/// global variable represented by `ValueRef`. If the global variable has no initializer, the function returns `None`.
/// The initializer is the constant value assigned to the global variable at the time of its definition.
///
/// # Returns
///
/// Returns an `Option<ValueRef>`:
/// - `Some(ValueRef)` if the global variable has an initializer.
/// - `None` if the global variable does not have an initializer.
#[must_use]
pub fn get_initializer(val: &ValueRef) -> Option<ValueRef> {
    let initializer = unsafe { core::LLVMGetInitializer(val.get_ref()) };
    if initializer.is_null() {
        None
    } else {
        Some(ValueRef(initializer))
    }
}

/// Sets the initializer for a global variable.
///
/// This function wraps the `LLVMSetInitializer` function from the LLVM core library. It assigns the provided constant value
/// as the initializer for the global variable represented by `ValueRef`. The initializer is a constant value that the global
/// variable will be set to when the program starts. Only constant values can be used as initializers for global variables.
///
/// # Parameters
///
/// - `constant_val`: A reference to the constant value (`ValueRef`) that will be used as the initializer for the global variable.
pub fn set_initializer(val: &ValueRef, constant_val: &ValueRef) {
    unsafe {
        core::LLVMSetInitializer(val.get_ref(), constant_val.get_ref());
    }
}

/// Determines if the global variable is thread-local.
///
/// This function wraps the `LLVMIsThreadLocal` function from the LLVM core library. It checks whether the global
/// variable represented by `ValueRef` is marked as thread-local. A thread-local variable has a separate instance for each
/// thread in a multi-threaded program, ensuring that threads do not share the same global variable.
///
/// # Returns
///
/// Returns `true` if the global variable is thread-local, otherwise returns `false`.
#[must_use]
pub fn is_thread_local(val: &ValueRef) -> bool {
    unsafe { core::LLVMIsThreadLocal(val.get_ref()) != 0 }
}

/// Sets whether the global variable is thread-local.
///
/// This function wraps the `LLVMSetThreadLocal` function from the LLVM core library. It marks the global variable
/// represented by `ValueRef` as either thread-local or not, based on the provided boolean value. A thread-local variable
/// has a separate instance for each thread in a multi-threaded program, ensuring that threads do not share the same
/// global variable.
///
/// # Parameters
///
/// - `is_thread_local`: A boolean value. If `true`, the global variable is marked as thread-local. If `false`, it is not thread-local.
pub fn set_thread_local(val: &ValueRef, is_thread_local: bool) {
    unsafe {
        core::LLVMSetThreadLocal(val.get_ref(), *CInt::from(is_thread_local));
    }
}

/// Determines if the global variable is a constant.
///
/// This function wraps the `LLVMIsGlobalConstant` function from the LLVM core library. It checks whether the global
/// variable represented by `ValueRef` is marked as a constant. A global constant cannot be modified after its initialization
/// and remains the same throughout the execution of the program.
///
/// # Returns
///
/// Returns `true` if the global variable is a constant, otherwise returns `false`.
#[must_use]
pub fn is_global_constant(val: &ValueRef) -> bool {
    unsafe { core::LLVMIsGlobalConstant(val.get_ref()) != 0 }
}

/// Sets whether the global variable is a constant.
///
/// This function wraps the `LLVMSetGlobalConstant` function from the LLVM core library. It marks the global variable
/// represented by `ValueRef` as either a constant or not, based on the provided boolean value. A global constant cannot
/// be modified after its initialization and remains constant throughout the execution of the program.
///
/// # Parameters
///
/// - `is_constant`: A boolean value. If `true`, the global variable is marked as a constant. If `false`, it is not a constant.
pub fn set_global_constant(val: &ValueRef, is_constant: bool) {
    unsafe {
        core::LLVMSetGlobalConstant(val.get_ref(), *CInt::from(is_constant));
    }
}

/// Retrieves the thread-local storage (TLS) mode of the global variable.
///
/// This function wraps the `LLVMGetThreadLocalMode` function from the LLVM core library. It returns the thread-local
/// mode of the global variable represented by `ValueRef`. The TLS mode defines how the thread-local variable is handled
/// by the runtime and can affect performance and behavior in multi-threaded environments.
///
/// # Returns
///
/// Returns a `ThreadLocalMode` enum value representing the thread-local mode of the global variable:
/// - `ThreadLocalMode::NotThreadLocal`: The global variable is not thread-local.
/// - `ThreadLocalMode::GeneralDynamicTLSModel`: General dynamic TLS model.
/// - `ThreadLocalMode::LocalDynamicTLSModel`: Local dynamic TLS model.
/// - `ThreadLocalMode::InitialExecTLSModel`: Initial exec TLS model.
/// - `ThreadLocalMode::LocalExecTLSModel`: Local exec TLS model.
#[must_use]
pub fn get_thread_local_mode(val: &ValueRef) -> ThreadLocalMode {
    unsafe { core::LLVMGetThreadLocalMode(val.get_ref()).into() }
}

/// Sets the thread-local storage (TLS) mode for the global variable.
///
/// This function wraps the `LLVMSetThreadLocalMode` function from the LLVM core library. It configures the thread-local
/// mode for the global variable represented by `ValueRef`. The TLS mode defines how the runtime handles the thread-local
/// variable, influencing its performance and behavior in multi-threaded environments.
///
/// # Parameters
///
/// - `mode`: A `ThreadLocalMode` enum value representing the desired thread-local mode:
///   - `ThreadLocalMode::NotThreadLocal`: The global variable is not thread-local.
///   - `ThreadLocalMode::GeneralDynamicTLSModel`: General dynamic TLS model.
///   - `ThreadLocalMode::LocalDynamicTLSModel`: Local dynamic TLS model.
///   - `ThreadLocalMode::InitialExecTLSModel`: Initial exec TLS model.
///   - `ThreadLocalMode::LocalExecTLSModel`: Local exec TLS model.
pub fn set_thread_local_mode(val: &ValueRef, mode: ThreadLocalMode) {
    unsafe {
        core::LLVMSetThreadLocalMode(val.get_ref(), mode.into());
    }
}

/// Determines if the global variable is externally initialized.
///
/// This function wraps the `LLVMIsExternallyInitialized` function from the LLVM core library. It checks whether
/// the global variable represented by `ValueRef` is marked as externally initialized. A global variable that is externally
/// initialized may have its initial value provided by external code, such as during dynamic linking.
///
/// # Returns
///
/// Returns `true` if the global variable is externally initialized, otherwise returns `false`.
#[must_use]
pub fn is_externally_initialized(val: &ValueRef) -> bool {
    unsafe { core::LLVMIsExternallyInitialized(val.get_ref()) != 0 }
}

/// Sets whether the global variable is externally initialized.
///
/// This function wraps the `LLVMSetExternallyInitialized` function from the LLVM core library. It marks the global variable
/// represented by `ValueRef` as externally initialized or not, based on the provided boolean value. Externally initialized
/// global variables may receive their initial values from external code, such as during dynamic linking.
///
/// # Parameters
///
/// - `is_ext_init`: A boolean value. If `true`, the global variable is marked as externally initialized. If `false`, it is not externally initialized.
pub fn set_externally_initialized(val: &ValueRef, is_ext_init: bool) {
    unsafe {
        core::LLVMSetExternallyInitialized(val.get_ref(), *CInt::from(is_ext_init));
    }
}
