// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7b30: [u8; 0];
    static mut DAT_100aaf74: u32;
    static mut DAT_101f52e0: [u8; 0];
    static mut DAT_10215658: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000a37c(param_1: i32, param_2: i32) -> i32;
    fn FUN_1003f893(param_1: *mut i32, param_2: i32, param_3: i32, param_4: *mut u32) -> u8;
    fn FUN_1003eb9c(param_1: *mut i32, param_2: i32, param_3: *mut u32) -> u8;
    fn FUN_10055f9e(
        param_1: i32,
        param_2: i32,
        param_3: *mut i32,
        param_4: i32,
        param_5: i32,
    ) -> u32;
    fn FUN_10016e60(param_1: i32, param_2: *mut u32, param_3: u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000a1d2(param_1: i32, param_2: *mut u32, param_3: u32) {
    mech_trace::enter!("1000a1d2", param_1, param_2, param_3);
    // SAFETY: the globals and callees are the game's own; param_2 points at
    // the caller's records of six u32 each, read and written where the
    // original does.
    unsafe {
        let i_var1: i32 = mech_trace::rd!(
            "1000a1d2",
            (&raw mut DAT_100a7b30)
                .cast::<u8>()
                .byte_add(
                    mech_trace::gr!("1000a1d2", "10215658", DAT_10215658).wrapping_mul(4) as usize
                )
                .cast::<i32>(),
            read_unaligned
        );
        if i_var1 == 0 {
            return;
        }
        let u_var2 = param_3 & 0xf000;
        let mut local_14 = param_2;
        if u_var2 < 0x3001 {
            if u_var2 == 0x3000 {
                mech_trace::cr!(
                    "1000a1d2",
                    "10055f9e",
                    FUN_10055f9e(
                        ((param_3 & 0xff0) >> 4) as i32,
                        param_1,
                        param_2.cast::<i32>(),
                        -1,
                        1,
                    )
                );
                return;
            }
            if u_var2 == 0 {
                mech_trace::cr!("1000a1d2", "10016e60", FUN_10016e60(param_1, param_2, 0));
                mech_trace::cr!(
                    "1000a1d2",
                    "10016e60",
                    FUN_10016e60(param_1, param_2, param_3 | 0xa000)
                );
                return;
            }
        } else {
            if u_var2 == 0x4000 {
                let mut local_10: i32 = 0;
                while local_10 < param_1 {
                    let v = mech_trace::cr!(
                        "1000a1d2",
                        "1000a37c",
                        FUN_1000a37c(
                            i_var1,
                            mech_trace::rd!("1000a1d2", local_14.add(5), read_unaligned) as i32
                        )
                    ) as u32;
                    mech_trace::wr!(
                        "1000a1d2",
                        local_14.add(2),
                        write_unaligned,
                        (v | (param_3 & 0xf0)).wrapping_shl(0x10)
                    );
                    local_14 = local_14.wrapping_add(6);
                    local_10 = local_10.wrapping_add(1);
                }
                if mech_trace::gr!("1000a1d2", "100aaf74", DAT_100aaf74) != 0 {
                    mech_trace::cr!(
                        "1000a1d2",
                        "1003f893",
                        FUN_1003f893(
                            (&raw mut DAT_101f52e0).cast::<i32>(),
                            0x7fff,
                            param_1,
                            param_2,
                        )
                    );
                    return;
                }
                mech_trace::cr!(
                    "1000a1d2",
                    "1003eb9c",
                    FUN_1003eb9c((&raw mut DAT_101f52e0).cast::<i32>(), param_1, param_2)
                );
                return;
            }
            if u_var2 == 0x8000 {
                return;
            }
            if u_var2 == 0x9000 {
                return;
            }
        }
        mech_trace::cr!(
            "1000a1d2",
            "10016e60",
            FUN_10016e60(param_1, param_2, param_3)
        );
    }
}
