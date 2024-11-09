use crate::sys::dpi as sys;
use std::{
    ffi::{c_char, c_void, CStr, CString},
    ptr::{self, NonNull},
};

/// Get current simulation time in _simulation time unit_. See also [`sys::svGetTime`]
///
/// # Panics
///
/// Panics if underlying `svGetTime` fails
#[cfg(feature = "sv2023")]
pub fn get_time() -> u64 {
    let mut time = sys::svTimeVal {
        type_: sys::sv_sim_time as i32,
        high: 0,
        low: 0,
        real: 0.0,
    };
    unsafe {
        let ret = sys::svGetTime(ptr::null_mut(), &mut time);
        assert!(ret == 0, "svGetTime failed");
    }

    ((time.high as u64) << 32) + (time.low as u64)
}

#[deprecated]
/// Equivalent to `SvScope::from_name(name).unwrap().make_current()`
pub fn set_scope_by_name(name: &str) {
    let scope = SvScope::from_name(name).unwrap_or_else(|| panic!("unrecognized scope `{name}`"));
    scope.make_current();
}

#[deprecated]
/// Instead use [`SvScope::make_current`]
pub fn set_scope(scope: SvScope) {
    unsafe {
        sys::svSetScope(scope.ptr.as_ptr());
    }
}

/// A non-null handle to a scope. See also [`sys::svScope`]
#[derive(Debug, Clone, Copy)]
pub struct SvScope {
    ptr: NonNull<c_void>,
}

unsafe impl Send for SvScope {}
unsafe impl Sync for SvScope {}

impl SvScope {
    unsafe fn from_raw_optional(ptr: sys::svScope) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr })
    }

    unsafe fn into_raw_optional(scope: Option<Self>) -> sys::svScope {
        match scope {
            None => ptr::null_mut(),
            Some(scope) => scope.ptr.as_ptr(),
        }
    }
}

impl SvScope {
    /// See also [`sys::svGetScopeFromName`]
    ///
    /// # Panics
    ///
    /// Panics if [`CString::new`] returns an error.
    pub fn from_name(name: &str) -> Option<Self> {
        let name = CString::new(name).unwrap();
        Self::from_name_cstr(&name)
    }

    /// See also [`sys::svGetScopeFromName`]
    pub fn from_name_cstr(name: &CStr) -> Option<Self> {
        unsafe { Self::from_name_raw(name.as_ptr()) }
    }

    /// See also [`sys::svGetScopeFromName`]
    ///
    /// # Safety
    ///
    /// `name` shall be a valid pointer to a C-style string
    pub unsafe fn from_name_raw(name: *const c_char) -> Option<Self> {
        Self::from_raw_optional(sys::svGetScopeFromName(name))
    }
}

impl SvScope {
    /// See also [`sys::svGetScope`]
    pub fn get_current() -> Option<Self> {
        unsafe { Self::from_raw_optional(sys::svGetScope()) }
    }

    /// Set current context. If previous context is needed, instead use [`Self::swap_current`]
    pub fn set_current(scope: Option<Self>) {
        unsafe {
            sys::svSetScope(Self::into_raw_optional(scope));
        }
    }

    /// Set current context, returns previous context. See also [`sys::svSetScope`]
    pub fn swap_current(scope: Option<Self>) -> Option<Self> {
        unsafe {
            let prev = sys::svSetScope(Self::into_raw_optional(scope));
            Self::from_raw_optional(prev)
        }
    }

    /// Set current context. If previous context is needed, instead use [`Self::swap_current`]
    pub fn make_current(self) {
        Self::set_current(Some(self));
    }

    /// Execute code with current context set temporarily, then restore the original context.
    ///
    /// # Unwind Behaviour
    ///
    /// If `f()` panics, the previous context will NOT be restores.
    ///
    /// This is the current behaviour. It may change in the future.
    pub fn with_current<R>(self, f: impl FnOnce() -> R) -> R {
        let prev = Self::swap_current(Some(self));
        let ret = f();
        Self::set_current(prev);
        ret
    }
}
