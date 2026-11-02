// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7584: u32;
    static mut DAT_100a7588: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000d05f(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000d14a() {
    let mut local_8: i32 = 0;
    // SAFETY: the callee and the globals are the game's own memory.
    unsafe {
        while local_8 < 0x402 {
            FUN_1000d05f(local_8);
            local_8 = local_8.wrapping_add(1);
        }
        DAT_100a7584 = 0;
        DAT_100a7588 = 1;
    }
}
