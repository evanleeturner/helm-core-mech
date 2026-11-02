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
    mech_trace::enter!(
        "10033980", param_1, param_2, param_3, param_4, param_5, param_6, param_7
    );
    // SAFETY: the globals and callees are the game's own; param_1 is passed
    // through untouched to the callees as the original does.
    unsafe {
        if mech_trace::gr!("10033980", "100adcd0", DAT_100adcd0) != 0 {
            if mech_trace::gr!("10033980", "100adcdc", DAT_100adcdc) != 0 {
                mech_trace::cr!(
                    "10033980",
                    "10034a3b",
                    FUN_10034a3b(
                        mech_trace::gr!("10033980", "100adca0", DAT_100adca0),
                        mech_trace::gr!("10033980", "100adca4", DAT_100adca4),
                        param_2,
                        param_3
                    )
                );
                mech_trace::cr!(
                    "10033980",
                    "1003481a",
                    FUN_1003481a(
                        param_1,
                        mech_trace::gr!("10033980", "100adca0", DAT_100adca0),
                        mech_trace::gr!("10033980", "100adca4", DAT_100adca4),
                        param_4,
                        param_5,
                        param_6,
                    )
                );
            }
            if mech_trace::gr!("10033980", "100adcd4", DAT_100adcd4) != 0 {
                mech_trace::cr!(
                    "10033980",
                    "10033f29",
                    FUN_10033f29(param_1, param_5, param_6, param_7)
                );
            }
            if mech_trace::gr!("10033980", "100adcd8", DAT_100adcd8) != 0 {
                mech_trace::cr!("10033980", "1003468e", FUN_1003468e(param_1));
            }
        }
    }
}
