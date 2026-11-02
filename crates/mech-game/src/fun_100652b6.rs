// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c7e88: u8;
    static mut DAT_100c7f98: u32;
    static mut DAT_100c7fa0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10064e6a(param_1: u8) -> u32;
    fn FUN_10064cd8(param_1: *mut core::ffi::c_void, param_2: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100652b6() -> u32 {
    // SAFETY: the globals and callees are the game's own memory and code;
    // they are read, written and called in the original's order.
    unsafe {
        let b_var1: u8 = ((DAT_100c7fa0 != 8) as u8).wrapping_add(DAT_100c7f98 as u8);
        if b_var1 != 0 {
            let i_var2 = FUN_10064e6a(b_var1);
            if i_var2 != 0 {
                return 0xffffffff;
            }
            let i_var2 = FUN_10064cd8(
                (&raw mut DAT_100c7e88).cast::<core::ffi::c_void>(),
                b_var1 as u32,
            );
            if i_var2 != 0 {
                return 0xffffffff;
            }
            DAT_100c7f98 = 0;
            DAT_100c7e88 = 0;
            DAT_100c7fa0 = 8;
        }
        0
    }
}
