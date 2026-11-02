// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f52e0: i32;
    static mut DAT_100aaf84: i32;
    static mut DAT_100aaf88: i32;
    static mut DAT_100aafa4: i32;
    static mut DAT_100aafd4: unsafe extern "C" fn(i32, *mut i32, u32);
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100562d8(param_1: *mut i32, param_2: i32, param_3: i32, param_4: u8);
    fn FUN_10056489(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: u32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10019e33(param_1: i32, param_2: *mut i32, param_3: u32) {
    mech_trace::enter!("10019e33", param_1, param_2, param_3);
    // SAFETY: the globals, callbacks and callees are the game's own; param_2
    // points at the caller's int array, read where the original reads it.
    unsafe {
        if param_1 == 1 && mech_trace::gr!("10019e33", "100aaf88", DAT_100aaf88) != 0 {
            if param_3 == 0x1000 {
                return;
            }
            FUN_100562d8(
                &raw mut DAT_101f52e0,
                mech_trace::rd!("10019e33", param_2, read_unaligned),
                mech_trace::rd!("10019e33", param_2.add(1), read_unaligned),
                param_3 as u8,
            );
            return;
        }
        if param_1 == 2 && mech_trace::gr!("10019e33", "100aaf84", DAT_100aaf84) != 0 {
            FUN_10056489(
                &raw mut DAT_101f52e0,
                mech_trace::rd!("10019e33", param_2, read_unaligned),
                mech_trace::rd!("10019e33", param_2.add(1), read_unaligned),
                mech_trace::rd!("10019e33", param_2.add(6), read_unaligned),
                mech_trace::rd!("10019e33", param_2.add(7), read_unaligned),
                0,
                param_3,
            );
            return;
        }
        if mech_trace::gr!("10019e33", "100aafa4", DAT_100aafa4) == 0 {
            DAT_100aafd4(param_1, param_2, param_3);
            return;
        }
        if mech_trace::gr!("10019e33", "100aafa4", DAT_100aafa4) == 3 {
            DAT_100aafd4(param_1, param_2, param_3);
            return;
        }
        if mech_trace::gr!("10019e33", "100aafa4", DAT_100aafa4) != 1 {
            DAT_100aafd4(param_1, param_2, param_3 | 0xa000);
            return;
        }
        let u_var1 = param_3 & 0xf000;
        if param_3 == 0 {
            return;
        }
        if u_var1 < 0x5001 {
            if u_var1 == 0x5000 || u_var1 == 0x3000 {
                DAT_100aafd4(param_1, param_2, param_3);
                return;
            }
        } else if u_var1 < 0x7001 {
            if u_var1 == 0x7000 || u_var1 == 0x6000 {
                DAT_100aafd4(param_1, param_2, param_3);
                return;
            }
        } else {
            if u_var1 == 0x8000 {
                DAT_100aafd4(param_1, param_2, param_3);
                return;
            }
            if u_var1 == 0x9000 {
                return;
            }
        }
        DAT_100aafd4(param_1, param_2, 0);
        DAT_100aafd4(param_1, param_2, param_3 | 0xa000);
    }
}
