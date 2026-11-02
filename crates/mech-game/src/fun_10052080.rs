// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10115180: u8;
    static mut DAT_10115181: u8;
    static mut DAT_101151be: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FID_conflict__memcpy(_Dst: *mut c_void, _Src: *mut c_void, _Size: usize) -> *mut c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10052080(param_1: u16) -> bool {
    mech_trace::enter!("10052080", param_1);
    let b_var1 = (param_1 >> 8) as u8 == 7;
    if b_var1 {
        // SAFETY: the globals and the copy routine are the game's own; the
        // addresses of the globals are passed as the original does.
        unsafe {
            mech_trace::cr!(
                "10052080",
                "FID_conflict__memcpy",
                FID_conflict__memcpy(
                    (&raw mut DAT_10115180).cast::<c_void>(),
                    (&raw mut DAT_10115181).cast::<c_void>(),
                    0x3e,
                )
            );
            mech_trace::gw!("10052080", "101151be", DAT_101151be, param_1 as u8);
        }
    }
    mech_trace::rt!("10052080", b_var1)
}
