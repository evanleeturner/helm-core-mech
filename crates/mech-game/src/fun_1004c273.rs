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
    mech_trace::enter!("1004c273");
    // SAFETY: the globals and callees are the game's own memory and code.
    unsafe {
        if 0 < (mech_trace::gr!("1004c273", "100b0700", DAT_100b0700) as i32) {
            mech_trace::gw!(
                "1004c273",
                "100c7e2c",
                DAT_100c7e2c,
                mech_trace::gr!("1004c273", "100c7e2c", DAT_100c7e2c).wrapping_add(
                    mech_trace::gr!("1004c273", "100b3904", DAT_100b3904).wrapping_mul(200)
                )
            );
            if (mech_trace::gr!("1004c273", "100c7e08", DAT_100c7e08) as i32)
                < (mech_trace::gr!("1004c273", "100c7e2c", DAT_100c7e2c) as i32)
            {
                mech_trace::gw!(
                    "1004c273",
                    "100c7e2c",
                    DAT_100c7e2c,
                    mech_trace::gr!("1004c273", "100c7e08", DAT_100c7e08)
                );
            }
            mech_trace::cr!(
                "1004c273",
                "1004b839",
                FUN_1004b839(
                    -2,
                    mech_trace::gr!("1004c273", "100c7e30", DAT_100c7e30) as i32,
                    mech_trace::gr!("1004c273", "100c7e34", DAT_100c7e34) as i32,
                    mech_trace::gr!("1004c273", "100c7e38", DAT_100c7e38) as i32,
                    mech_trace::gr!("1004c273", "100c7e2c", DAT_100c7e2c) as i32,
                    0x1000,
                    0,
                )
            );
            mech_trace::cr!(
                "1004c273",
                "1004b9f3",
                FUN_1004b9f3(
                    0xfffffffe,
                    mech_trace::gr!("1004c273", "100c7e30", DAT_100c7e30) as i32,
                    mech_trace::gr!("1004c273", "100c7e34", DAT_100c7e34) as i32,
                    mech_trace::gr!("1004c273", "100c7e38", DAT_100c7e38) as i32,
                    mech_trace::gr!("1004c273", "100c7e2c", DAT_100c7e2c) as i32,
                    0x1000,
                )
            );
            mech_trace::cr!(
                "1004c273",
                "1007ca5c",
                FUN_1007ca5c(
                    mech_trace::gr!("1004c273", "100c7e30", DAT_100c7e30) as i32,
                    mech_trace::gr!("1004c273", "100c7e34", DAT_100c7e34) as i32,
                    mech_trace::gr!("1004c273", "100c7e38", DAT_100c7e38) as i32,
                    mech_trace::gr!("1004c273", "100c7e2c", DAT_100c7e2c) as i32,
                    0x1000,
                )
            );
            mech_trace::gw!(
                "1004c273",
                "100b0700",
                DAT_100b0700,
                mech_trace::gr!("1004c273", "100b0700", DAT_100b0700)
                    .wrapping_sub(mech_trace::gr!("1004c273", "100b3904", DAT_100b3904))
            );
        }
    }
}
