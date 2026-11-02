// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b04b8: u8;
    static mut DAT_100b04c4: u32;
    static mut DAT_100b3c54: u32;
    static mut DAT_100c7ce8: u32;
    static mut DAT_100c7cec: u32;
    static mut DAT_100c7cf0: u8;
    static mut DAT_100c7d00: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10046175() -> u32;
    fn FUN_10046563() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10046dbc(param_1: *mut i32) -> u32;
    fn FUN_10046f3b(param_1: *mut u32) -> u8;
    fn FUN_10047052(param_1: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004643b() -> u32 {
    // SAFETY: the globals and callees are the game's own memory and code;
    // the addresses passed are those of the game's buffers.
    unsafe {
        DAT_100c7ce8 = 0;
        DAT_100c7cec = 0;
        let i_var1 = FUN_10046175() as i32;
        if i_var1 == 0 {
            DAT_100b04c4 = 1;
            DAT_100c7d00 = FUN_10046563();
            match DAT_100c7d00 {
                1 => {
                    DAT_100b04c4 = 0;
                    return 0;
                }
                2 => {
                    FUN_10046dbc((&raw mut DAT_100b04b8).cast::<i32>());
                }
                3 => {
                    FUN_10046dbc((&raw mut DAT_100b04b8).cast::<i32>());
                }
                4 => {
                    FUN_10046dbc((&raw mut DAT_100b04b8).cast::<i32>());
                    FUN_10046f3b((&raw mut DAT_100c7cf0).cast::<u32>());
                }
                _ => {}
            }
            FUN_10047052(DAT_100b3c54);
            1
        } else {
            0
        }
    }
}
