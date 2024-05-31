#![doc = "```rust
use minhook_raw::*;
// use minhook_raw::sys::*;
```"]

pub mod sys;

#[repr(C)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum Status {
    Unknown,

    Ok,

    ErrorAlreadyInitialized,

    ErrorNotInitialized,

    ErrorAlreadyCreated,

    ErrorNotCreated,

    ErrorEnabled,

    ErrorDisabled,

    ErrorNotExecutable,

    ErrorUnsupportedFunction,

    ErrorMemoryAlloc,

    ErrorMemoryProtect,

    ErrorModuleNotFound,

    ErrorFunctionNotFound,
}

impl From<crate::sys::MH_STATUS> for Status {
    fn from(value: crate::sys::MH_STATUS) -> Self {
        match value {
            crate::sys::MH_STATUS::MH_UNKNOWN => Self::Unknown,
            crate::sys::MH_STATUS::MH_OK => Self::Ok,
            crate::sys::MH_STATUS::MH_ERROR_ALREADY_INITIALIZED => Self::ErrorAlreadyInitialized,
            crate::sys::MH_STATUS::MH_ERROR_NOT_INITIALIZED => Self::ErrorNotInitialized,
            crate::sys::MH_STATUS::MH_ERROR_ALREADY_CREATED => Self::ErrorAlreadyCreated,
            crate::sys::MH_STATUS::MH_ERROR_NOT_CREATED => Self::ErrorNotCreated,
            crate::sys::MH_STATUS::MH_ERROR_ENABLED => Self::ErrorEnabled,
            crate::sys::MH_STATUS::MH_ERROR_DISABLED => Self::ErrorDisabled,
            crate::sys::MH_STATUS::MH_ERROR_NOT_EXECUTABLE => Self::ErrorNotExecutable,
            crate::sys::MH_STATUS::MH_ERROR_UNSUPPORTED_FUNCTION => Self::ErrorUnsupportedFunction,
            crate::sys::MH_STATUS::MH_ERROR_MEMORY_ALLOC => Self::ErrorMemoryAlloc,
            crate::sys::MH_STATUS::MH_ERROR_MEMORY_PROTECT => Self::ErrorMemoryProtect,
            crate::sys::MH_STATUS::MH_ERROR_MODULE_NOT_FOUND => Self::ErrorModuleNotFound,
            crate::sys::MH_STATUS::MH_ERROR_FUNCTION_NOT_FOUND => Self::ErrorFunctionNotFound,
        }
    }
}

impl PartialEq<Status> for bool {
    fn eq(&self, other: &Status) -> bool {
        self == &match other {
            Status::Ok => true,
            _ => false,
        }
    }
}

impl PartialEq<bool> for Status {
    fn eq(&self, other: &bool) -> bool {
        &match self {
            Status::Ok => true,
            _ => false,
        } == other
    }
}

impl ::std::fmt::Display for Status {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Status {
    #[inline]
    pub fn is_ok(&self) -> bool {
        self == &Status::Ok
    }

    #[inline]
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    #[inline]
    pub fn is_unk(&self) -> bool {
        self == &Status::Unknown
    }
}

pub fn initialize() -> Status {
    unsafe { crate::sys::MH_Initialize().into() }
}

pub fn uninitialize() -> Status {
    unsafe { crate::sys::MH_Uninitialize().into() }
}

pub fn create_hook(
    p_target: *mut ::std::ffi::c_void,
    p_detour: *mut ::std::ffi::c_void,
    pp_original: *mut *mut ::std::ffi::c_void,
) -> Status {
    unsafe { crate::sys::MH_CreateHook(p_target, p_detour, pp_original).into() }
}

pub fn create_hook_api(
    psz_module: *const u16,
    psz_proc_name: *const i8,
    p_detour: *mut ::std::ffi::c_void,
    pp_original: *mut *mut ::std::ffi::c_void,
) -> Status {
    unsafe { crate::sys::MH_CreateHookApi(psz_module, psz_proc_name, p_detour, pp_original).into() }
}

pub fn create_hook_api_ex(
    psz_module: *const u16,
    psz_proc_name: *const i8,
    p_detour: *mut ::std::ffi::c_void,
    pp_original: *mut *mut ::std::ffi::c_void,
    pp_target: *mut *mut ::std::ffi::c_void,
) -> Status {
    unsafe {
        crate::sys::MH_CreateHookApiEx(psz_module, psz_proc_name, p_detour, pp_original, pp_target)
            .into()
    }
}

pub fn remove_hook(p_target: *mut ::std::ffi::c_void) -> Status {
    unsafe { crate::sys::MH_RemoveHook(p_target).into() }
}

pub fn enable_hook(p_target: *mut ::std::ffi::c_void) -> Status {
    unsafe { crate::sys::MH_EnableHook(p_target).into() }
}

pub fn disable_hook(p_target: *mut ::std::ffi::c_void) -> Status {
    unsafe { crate::sys::MH_DisableHook(p_target).into() }
}

pub fn queue_enable_hook(p_target: *mut ::std::ffi::c_void) -> Status {
    unsafe { crate::sys::MH_QueueEnableHook(p_target).into() }
}

pub fn queue_disable_hook(p_target: *mut ::std::ffi::c_void) -> Status {
    unsafe { crate::sys::MH_QueueDisableHook(p_target).into() }
}

pub fn apply_queued() -> Status {
    unsafe { crate::sys::MH_ApplyQueued().into() }
}
