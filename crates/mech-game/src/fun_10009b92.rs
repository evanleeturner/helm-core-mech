// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10215658: i32;
    static mut DAT_10215660: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10009b92() {
    // SAFETY: the globals are the game's own memory, read and written where the original does.
    unsafe {
        if DAT_10215658 < 3 && DAT_10215660 < 3 {
            DAT_10215660 = ((DAT_10215658 as u32).wrapping_add(1) as i32) % 3;
        }
    }
}
