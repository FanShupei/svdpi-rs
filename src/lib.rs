pub mod dpi;

#[cfg(feature = "vpi")]
pub mod vpi;

pub mod sys;

pub use dpi::{set_scope, set_scope_by_name, SvScope};

#[cfg(feature = "sv2023")]
pub fn get_time() -> u64 {
    dpi::get_time()
}

#[cfg(all(not(feature = "sv2023"), feature = "vpi"))]
pub fn get_time() -> u64 {
    vpi::get_time()
}
