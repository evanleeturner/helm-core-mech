// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008d150: *mut c_void;
    static mut DAT_1008d55c: *mut c_void;
    static mut DAT_100c6060: u32;
    static mut DAT_100d1540: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn BitBlt(
        param_1: *mut c_void,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: *mut c_void,
        param_7: i32,
        param_8: i32,
        param_9: u32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000e150(x1: i32, y1: i32, x2: i32) -> i32 {
    let blit_size = x2.wrapping_sub(x1).wrapping_add(1);
    // SAFETY: the globals and BitBlt are the game's own; the handles are
    // passed through unchanged as the original does.
    unsafe {
        DAT_100d1540 = BitBlt(
            DAT_1008d150,
            0xa0,
            0x8c,
            blit_size,
            blit_size,
            DAT_1008d55c,
            x1,
            y1,
            0xcc0020,
        ) as u32;
        ((DAT_100c6060 == DAT_100d1540) as i32).wrapping_sub(1)
    }
}
