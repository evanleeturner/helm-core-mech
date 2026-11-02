// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adca0: u32;
    static mut DAT_100adca4: u32;
    static mut DAT_100adcd0: u32;
    static mut DAT_100adcd4: u32;
    static mut DAT_100adcd8: u32;
    static mut DAT_100adcdc: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10034a3b(param_1: u32, param_2: u32, param_3: i32, param_4: i32) -> u8;
    fn FUN_1003481a(
        param_1: *mut i32,
        param_2: u32,
        param_3: u32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> u8;
    fn FUN_10033f29(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32) -> u32;
    fn FUN_1003468e(param_1: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10033980(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
) {
    // SAFETY: the globals and callees are the game's own; param_1 is passed
    // through untouched to the callees as the original does.
    unsafe {
        if DAT_100adcd0 != 0 {
            if DAT_100adcdc != 0 {
                FUN_10034a3b(DAT_100adca0, DAT_100adca4, param_2, param_3);
                FUN_1003481a(
                    param_1,
                    DAT_100adca0,
                    DAT_100adca4,
                    param_4,
                    param_5,
                    param_6,
                );
            }
            if DAT_100adcd4 != 0 {
                FUN_10033f29(param_1, param_5, param_6, param_7);
            }
            if DAT_100adcd8 != 0 {
                FUN_1003468e(param_1);
            }
        }
    }
}
