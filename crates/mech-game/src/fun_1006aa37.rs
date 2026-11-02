// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3c40: u32;
    static mut DAT_100b3c44: u32;
    static mut DAT_100b3c58: u8;
    static mut DAT_100b3ca4: u32;
    static mut DAT_100b3cb4: [i8; 1];
    static mut DAT_10165940: u32;
    static mut DAT_100af104: u32;
    static mut DAT_100b3cac: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16;
    fn sscanf(src: *const i8, format: *const i8, ...) -> i32;
    fn FUN_1008664f(param_1: u32, param_2: *mut i32) -> u8;
    fn FUN_100108a0(param_1: u32, param_2: *const i8) -> u8;
    fn FUN_10046fc9(param_1: i32) -> u32;
    fn FUN_10046b4d(param_1: i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10046fb4() -> u32;
    fn FUN_10046563() -> u32;
    fn FUN_1004663a() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006aa37() {
    mech_trace::enter!("1006aa37");
    // SAFETY: all globals and callees are the game's own memory and code;
    // accesses happen in the same order as the original.
    unsafe {
        if mech_trace::gr!("1006aa37", "100b3ca4", DAT_100b3ca4) == 0 {
            mech_trace::gw!("1006aa37", "100b3ca4", DAT_100b3ca4, 1);
            if (mech_trace::gr!("1006aa37", "10165940", DAT_10165940) as i32) < 1 {
                mech_trace::gw!("1006aa37", "100b3ca4", DAT_100b3ca4, 1);
                return;
            }
            let src = mech_trace::cr!(
                "1006aa37",
                "1008668b",
                FUN_1008668b(
                    0,
                    mech_trace::gr!("1006aa37", "10165940", DAT_10165940),
                    mech_trace::gr!("1006aa37", "100af104", DAT_100af104) as *mut i32
                )
            );
            if src.is_null() {
                return;
            }
            mech_trace::cr!(
                "1006aa37",
                "sscanf",
                sscanf(
                    src as *const i8,
                    (&raw mut DAT_100b3cac) as *const i8,
                    &raw mut DAT_100b3c40,
                    &raw mut DAT_100b3c44,
                )
            );
            mech_trace::cr!(
                "1006aa37",
                "1008664f",
                FUN_1008664f(
                    mech_trace::gr!("1006aa37", "10165940", DAT_10165940),
                    mech_trace::gr!("1006aa37", "100af104", DAT_100af104) as *mut i32
                )
            );
        }
        let mut i_var1 = mech_trace::cr!("1006aa37", "10046fb4", FUN_10046fb4());
        if i_var1 == 0 {
            mech_trace::gw!("1006aa37", "100b3c40", DAT_100b3c40, 0xffff_ffff);
        } else {
            i_var1 = mech_trace::cr!("1006aa37", "10046563", FUN_10046563());
            if i_var1 == 1 {
                mech_trace::gw!("1006aa37", "100b3c40", DAT_100b3c40, 0xffff_ffff);
                mech_trace::cr!(
                    "1006aa37",
                    "100108a0",
                    FUN_100108a0(0x53, (&raw mut DAT_100b3cb4) as *const i8)
                );
            } else {
                i_var1 = mech_trace::cr!(
                    "1006aa37",
                    "10046fc9",
                    FUN_10046fc9(mech_trace::gr!("1006aa37", "100b3c40", DAT_100b3c40) as i32)
                );
                if i_var1 == 0 {
                    mech_trace::gw!("1006aa37", "100b3c40", DAT_100b3c40, 0xffff_ffff);
                } else if (mech_trace::gr!("1006aa37", "100b3c58", DAT_100b3c58) & 8) != 0 {
                    mech_trace::cr!(
                        "1006aa37",
                        "10046b4d",
                        FUN_10046b4d(mech_trace::gr!("1006aa37", "100b3c40", DAT_100b3c40) as i32)
                    );
                    i_var1 = mech_trace::cr!("1006aa37", "1004663a", FUN_1004663a());
                    if i_var1 != 3 {
                        mech_trace::gw!("1006aa37", "100b3c40", DAT_100b3c40, 0xffff_ffff);
                    }
                }
            }
        }
        mech_trace::gw!("1006aa37", "100b3c44", DAT_100b3c44, 0xffff_ffff);
    }
}
