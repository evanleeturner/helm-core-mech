// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008d0b0: u8;
    static mut DAT_1008d0fc: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005b1b0(param_1: *const i8);
    fn FUN_1000e970(param_1: *mut u8, param_2: i32, param_3: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000fe40();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000d5a0() {
    // SAFETY: the callees and the string data are the game's own; the
    // strings are passed by address, in the original's call order.
    unsafe {
        FUN_1005b1b0((&raw mut DAT_1008d0b0).cast::<i8>() as *const i8);
        FUN_1000fe40();
        FUN_1000e970(&raw mut DAT_1008d0fc, 0xc02b, 0xc01c);
    }
}
