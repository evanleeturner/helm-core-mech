// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7178: i32;
    static mut DAT_100a7180: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002570(param_1: i32, param_2: i32) -> i32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000a37c(param_1: i32, param_2: i32) -> i32 {
    mech_trace::enter!("1000a37c", param_1, param_2);
    // SAFETY: param_1 points at the caller's struct; the callees and globals
    // are the game's own.
    unsafe {
        let field = mech_trace::rd!(
            "1000a37c",
            (param_1 as usize as *const i32).byte_add(0x18),
            read_unaligned
        );
        let arg = field
            .wrapping_sub(param_2 >> 2)
            .wrapping_sub(mech_trace::gr!("1000a37c", "100a7178", DAT_100a7178));
        let i_var1 = mech_trace::cr!(
            "1000a37c",
            "10002570",
            FUN_10002570(arg, mech_trace::gr!("1000a37c", "100a7180", DAT_100a7180))
        );
        let mut local_8 = mech_trace::cr!("1000a37c", "10003d60", FUN_10003d60(i_var1, 0x10));
        if local_8 < 0 {
            local_8 = 0;
        } else if 0xf < local_8 {
            local_8 = 0xf;
        }
        mech_trace::rt!("1000a37c", local_8)
    }
}
