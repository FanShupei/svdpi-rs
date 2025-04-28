/// High-level wrappers for DPI
pub mod dpi;

/// High-level wrappers for VPI
#[cfg(feature = "vpi")]
pub mod vpi;

/// Raw Bindings for C headers
pub mod sys;

pub use dpi::{SvScope, set_scope, set_scope_by_name};

/// Get current simulation time in _simulation time unit_.
///
/// Use `svGetTime` is `sv2023` feature is enabled. Otherwise, use `vpi_get_time`.
#[cfg(any(feature = "sv2023", feature = "vpi"))]
pub fn get_time() -> u64 {
    #[cfg(feature = "sv2023")]
    {
        dpi::get_time()
    }

    #[cfg(all(not(feature = "sv2023"), feature = "vpi"))]
    {
        vpi::get_time()
    }
}
