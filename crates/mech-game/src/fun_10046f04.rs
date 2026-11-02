// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h_heap: *mut c_void, dw_flags: u32, lp_mem: *mut c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10046f04(param_1: i32) {
    mech_trace::enter!("10046f04", param_1);
    // SAFETY: param_1 is the address of the caller's structure; the field at
    // offset 8 is read, freed through the game's heap handle, and cleared.
    unsafe {
        let p = (param_1 as usize).wrapping_add(8) as *mut i32;
        if mech_trace::rd!("10046f04", p, read_unaligned) != 0 {
            mech_trace::cr!(
                "10046f04",
                "HeapFree",
                HeapFree(
                    mech_trace::gr!("10046f04", "100adf74", DAT_100adf74),
                    1,
                    mech_trace::rd!("10046f04", p, read_unaligned) as usize as *mut c_void
                )
            );
            mech_trace::wr!("10046f04", p, write_unaligned, 0);
        }
    }
}
