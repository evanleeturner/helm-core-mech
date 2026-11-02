// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100afbc0: *mut core::ffi::c_void;
    static mut DAT_100b9d30: *const core::ffi::c_void;
    static mut DAT_100c9250: u8;
    static mut DAT_100c967c: u32;
    static mut DAT_100c9680: u32;
    static mut DAT_10160bec: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn StretchDIBits(
        param_1: *mut core::ffi::c_void,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
        param_8: i32,
        param_9: i32,
        param_10: *const core::ffi::c_void,
        param_11: *const core::ffi::c_void,
        param_12: u32,
        param_13: u32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100457f5(x_src: i32, param_2: i32, param_3: i32, param_4: i32) -> u32 {
    // SAFETY: the globals and StretchDIBits are the game's own; the header
    // address is passed through without forming a reference.
    unsafe {
        let r = StretchDIBits(
            DAT_100afbc0,
            0,
            0,
            DAT_100c967c as i32,
            DAT_100c9680 as i32,
            x_src,
            (DAT_100c9680 as i32).wrapping_sub(param_4).wrapping_add(-1),
            param_3.wrapping_sub(x_src).wrapping_add(1),
            param_4.wrapping_sub(param_2).wrapping_add(1),
            DAT_100b9d30,
            (&raw mut DAT_100c9250) as *const core::ffi::c_void,
            1,
            0xcc0020,
        );
        DAT_10160bec = r as u32;
        if DAT_10160bec == DAT_100c9680 {
            0
        } else {
            0xffffffff
        }
    }
}
