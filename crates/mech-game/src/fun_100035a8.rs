// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3c4c: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000387d() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002ba7(
        param_1: i32,
        param_2: u32,
        param_3: u32,
        param_4: *mut core::ffi::c_void,
        param_5: i32,
        param_6: i32,
        param_7: u32,
        param_8: u32,
        param_9: i32,
        param_10: u16,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100035a8() {
    // SAFETY: the callees and the global are the game's own; arguments are
    // passed in the original's order.
    unsafe {
        let u_var3: u16 = 0x250;
        let i_var2: i32 = -1;
        let u_var1 = FUN_1000387d();
        FUN_10002ba7(
            0,
            0,
            0xe3,
            core::ptr::null_mut(),
            100,
            DAT_100b3c4c as i32,
            0x40,
            u_var1,
            i_var2,
            u_var3,
        );
    }
}
