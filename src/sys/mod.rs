/// Raw bindings for `svdpi.h`
#[rustfmt::skip]
pub mod dpi;

/// Raw bindings for `sv_vpi_user.h`
#[cfg(feature = "vpi")]
#[rustfmt::skip]
pub mod vpi;
