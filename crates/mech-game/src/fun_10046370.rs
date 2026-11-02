// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b04b0: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn mciSendCommandA(param_1: u32, param_2: u32, param_3: u32, param_4: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10046370() {
    let mut local_8: u32 = 0;
    // SAFETY: the callee and the global are the game's own; local_8 is a
    // live stack slot whose address is passed to the callee.
    unsafe {
        mciSendCommandA(DAT_100b04b0, 0x809, 0, (&raw mut local_8) as usize as u32);
    }
}
