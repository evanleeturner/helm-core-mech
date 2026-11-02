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
    mech_trace::enter!("1004643b");
    // SAFETY: the globals and callees are the game's own memory and code;
    // the addresses passed are those of the game's buffers.
    unsafe {
        mech_trace::gw!("1004643b", "100c7ce8", DAT_100c7ce8, 0);
        mech_trace::gw!("1004643b", "100c7cec", DAT_100c7cec, 0);
        let i_var1 = mech_trace::cr!("1004643b", "10046175", FUN_10046175()) as i32;
        mech_trace::rt!(
            "1004643b",
            if i_var1 == 0 {
                mech_trace::gw!("1004643b", "100b04c4", DAT_100b04c4, 1);
                mech_trace::gw!(
                    "1004643b",
                    "100c7d00",
                    DAT_100c7d00,
                    mech_trace::cr!("1004643b", "10046563", FUN_10046563())
                );
                match mech_trace::gr!("1004643b", "100c7d00", DAT_100c7d00) {
                    1 => {
                        mech_trace::gw!("1004643b", "100b04c4", DAT_100b04c4, 0);
                        return mech_trace::rt!("1004643b", 0);
                    }
                    2 => {
                        mech_trace::cr!(
                            "1004643b",
                            "10046dbc",
                            FUN_10046dbc((&raw mut DAT_100b04b8).cast::<i32>())
                        );
                    }
                    3 => {
                        mech_trace::cr!(
                            "1004643b",
                            "10046dbc",
                            FUN_10046dbc((&raw mut DAT_100b04b8).cast::<i32>())
                        );
                    }
                    4 => {
                        mech_trace::cr!(
                            "1004643b",
                            "10046dbc",
                            FUN_10046dbc((&raw mut DAT_100b04b8).cast::<i32>())
                        );
                        mech_trace::cr!(
                            "1004643b",
                            "10046f3b",
                            FUN_10046f3b((&raw mut DAT_100c7cf0).cast::<u32>())
                        );
                    }
                    _ => {}
                }
                mech_trace::cr!(
                    "1004643b",
                    "10047052",
                    FUN_10047052(mech_trace::gr!("1004643b", "100b3c54", DAT_100b3c54))
                );
                1
            } else {
                0
            }
        )
    }
}
