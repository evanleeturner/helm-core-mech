// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008d150: *mut c_void;
    static mut DAT_100c605c: i32;
    static mut DAT_100c6060: i32;
    static mut DAT_1008d55c: *mut c_void;
    static mut DAT_100d1540: i32;
    static mut DAT_1008da0c: i8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn BitBlt(
        hdc: *mut c_void,
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        hdc_src: *mut c_void,
        x1: i32,
        y1: i32,
        rop: u32,
    ) -> i32;
    fn GetLastError() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100640a0(fmt: *const i8, ...);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000df80() -> i32 {
    // SAFETY: the globals, the format string and the callees are the game's
    // own; the arguments are passed as the original passes them.
    unsafe {
        DAT_100d1540 = BitBlt(
            DAT_1008d150,
            0,
            0,
            DAT_100c605c,
            DAT_100c6060,
            DAT_1008d55c,
            0,
            0,
            0xcc0020,
        );
        if DAT_100d1540 == 0 {
            FUN_100640a0((&raw mut DAT_1008da0c).cast_const(), GetLastError());
        }
        ((DAT_100c6060 == DAT_100d1540) as u32).wrapping_sub(1) as i32
    }
}
