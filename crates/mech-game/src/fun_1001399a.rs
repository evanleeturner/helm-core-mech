// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1cc: u32;
    static mut DAT_100aa2c0: u16;
    static mut DAT_100aa2c2: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10006e00(param_1: u32, param_2: u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1001399a() {
    let mut local_8: u16;
    let mut local_6: u8;
    // SAFETY: the globals and the callee are the game's own; local_8 is a
    // stack buffer whose address is passed to the callee as the original does.
    unsafe {
        local_8 = DAT_100aa2c0;
        local_6 = DAT_100aa2c2;
        let _ = local_6;
        if DAT_100aa1cc == 0 {
            FUN_10006e00((&raw mut local_8) as u32, 2);
            DAT_100aa1cc = 1;
        }
        local_6 = local_6;
    }
}
