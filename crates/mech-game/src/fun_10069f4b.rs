// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3a40: u32;
    static mut DAT_100b3a44: u32;
    static mut DAT_100b3a58: u32;
    static mut DAT_100b3a5c: u32;
    static mut DAT_100eee64: u32;
    static mut DAT_100b3b5c: [i8; 1];
    static mut DAT_100b3b3c: [i8; 1];
    static mut DAT_100b3b7c: [i8; 1];
}

unsafe extern "C" {
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005c0df(param_1: i32, param_2: i32, param_3: *mut i8, param_4: i32, param_5: i32)
    -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10069f4b(param_1: *mut u32) {
    mech_trace::enter!("10069f4b", param_1);
    let mut local_88: [i8; 132] = [0; 132];
    // SAFETY: globals and callees are the game's own; param_1 points at the
    // caller's array of at least six u32 values, read where the original does.
    unsafe {
        if mech_trace::gr!("10069f4b", "100b3a58", DAT_100b3a58) != 0
            || mech_trace::gr!("10069f4b", "100eee64", DAT_100eee64) != 0
        {
            let p0 = mech_trace::rd!("10069f4b", param_1, read_unaligned);
            let p1 = mech_trace::rd!("10069f4b", param_1.add(1), read_unaligned);
            let p2 = mech_trace::rd!("10069f4b", param_1.add(2), read_unaligned);
            mech_trace::cr!(
                "10069f4b",
                "sprintf",
                sprintf(
                    local_88.as_mut_ptr(),
                    (&raw mut DAT_100b3b3c).cast::<i8>(),
                    p0,
                    p1,
                    p2,
                )
            );
            let r4 =
                (mech_trace::rd!("10069f4b", param_1.add(4), read_unaligned) as i32 >> 16) % 0x168;
            let r3 =
                (mech_trace::rd!("10069f4b", param_1.add(3), read_unaligned) as i32 >> 16) % 0x168;
            let r5 =
                (mech_trace::rd!("10069f4b", param_1.add(5), read_unaligned) as i32 >> 16) % 0x168;
            mech_trace::cr!(
                "10069f4b",
                "sprintf",
                sprintf(
                    local_88.as_mut_ptr(),
                    (&raw mut DAT_100b3b5c).cast::<i8>(),
                    r4,
                    r3,
                    r5,
                )
            );
            if mech_trace::gr!("10069f4b", "100b3a58", DAT_100b3a58) != 0 {
                let p0 = mech_trace::rd!("10069f4b", param_1, read_unaligned);
                let p1 = mech_trace::rd!("10069f4b", param_1.add(1), read_unaligned);
                let p2 = mech_trace::rd!("10069f4b", param_1.add(2), read_unaligned);
                let r4 = (mech_trace::rd!("10069f4b", param_1.add(4), read_unaligned) as i32 >> 16)
                    % 0x168;
                let r3 = (mech_trace::rd!("10069f4b", param_1.add(3), read_unaligned) as i32 >> 16)
                    % 0x168;
                let r5 = (mech_trace::rd!("10069f4b", param_1.add(5), read_unaligned) as i32 >> 16)
                    % 0x168;
                mech_trace::cr!(
                    "10069f4b",
                    "sprintf",
                    sprintf(
                        local_88.as_mut_ptr(),
                        (&raw mut DAT_100b3b7c).cast::<i8>(),
                        p0,
                        p1,
                        p2,
                        r4,
                        r3,
                        r5,
                    )
                );
                mech_trace::cr!(
                    "10069f4b",
                    "1005c0df",
                    FUN_1005c0df(
                        0x37,
                        1,
                        local_88.as_mut_ptr(),
                        mech_trace::gr!("10069f4b", "100b3a40", DAT_100b3a40) as i32,
                        mech_trace::gr!("10069f4b", "100b3a44", DAT_100b3a44) as i32,
                    )
                );
            }
            mech_trace::gw!("10069f4b", "100b3a5c", DAT_100b3a5c, 1);
        }
    }
}
