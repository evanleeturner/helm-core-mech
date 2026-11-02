// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0700: u32;
    static mut DAT_100b3904: u32;
    static mut DAT_100c7e08: u32;
    static mut DAT_100c7e2c: u32;
    static mut DAT_100c7e30: u32;
    static mut DAT_100c7e34: u32;
    static mut DAT_100c7e38: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004b839(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    ) -> u8;
    fn FUN_1004b9f3(
        param_1: u32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> u8;
    fn FUN_1007ca5c(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004c273() {
    // SAFETY: the globals and callees are the game's own memory and code.
    unsafe {
        if 0 < (DAT_100b0700 as i32) {
            DAT_100c7e2c = DAT_100c7e2c.wrapping_add(DAT_100b3904.wrapping_mul(200));
            if (DAT_100c7e08 as i32) < (DAT_100c7e2c as i32) {
                DAT_100c7e2c = DAT_100c7e08;
            }
            FUN_1004b839(
                -2,
                DAT_100c7e30 as i32,
                DAT_100c7e34 as i32,
                DAT_100c7e38 as i32,
                DAT_100c7e2c as i32,
                0x1000,
                0,
            );
            FUN_1004b9f3(
                0xfffffffe,
                DAT_100c7e30 as i32,
                DAT_100c7e34 as i32,
                DAT_100c7e38 as i32,
                DAT_100c7e2c as i32,
                0x1000,
            );
            FUN_1007ca5c(
                DAT_100c7e30 as i32,
                DAT_100c7e34 as i32,
                DAT_100c7e38 as i32,
                DAT_100c7e2c as i32,
                0x1000,
            );
            DAT_100b0700 = DAT_100b0700.wrapping_sub(DAT_100b3904);
        }
    }
}
