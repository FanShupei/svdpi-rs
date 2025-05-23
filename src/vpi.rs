use crate::sys::vpi as sys;
use std::ptr;

/// Get current simulation time in _simulation time unit_
pub fn get_time() -> u64 {
    let mut time = sys::s_vpi_time {
        type_: sys::vpiSimTime,
        high: 0,
        low: 0,
        real: 0.0,
    };
    unsafe {
        sys::vpi_get_time(ptr::null_mut(), &mut time);
    }
    ((time.high as u64) << 32) + (time.low as u64)
}
