// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_100a1210: f64;
    static mut _DAT_100a1218: f64;
    static mut DAT_100b38fc: i32;
    static mut DAT_100afb88: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004403d(param_1: i32, param_2: i8);
    fn sprintf(dst: *mut i8, fmt: *const i8, ...) -> i32;
    fn FUN_1005dd5a(param_1: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10043f2c(param_1: i32) -> i32 {
    mech_trace::enter!("10043f2c", param_1);
    let mut local_5c = [0i8; 80];
    let mut local_c: u32 = 0;
    // SAFETY: param_1 points at the game's own structures; offsets, globals
    // and callees are the original's, accessed in the original's order.
    unsafe {
        let local_8 = mech_trace::rd!(
            "10043f2c",
            (param_1.wrapping_add(0x20) as *const i32),
            read_unaligned
        );
        let a = mech_trace::rd!(
            "10043f2c",
            (local_8.wrapping_add(0x3f4) as *const i32),
            read_unaligned
        );
        let b = mech_trace::rd!(
            "10043f2c",
            (local_8.wrapping_add(0x420) as *const i32),
            read_unaligned
        );
        if a < 0 || b == 0 {
            return mech_trace::rt!(
                "10043f2c",
                mech_trace::rd!(
                    "10043f2c",
                    (param_1.wrapping_add(0x78) as *const i32),
                    read_unaligned
                )
            );
        }
        let p = mech_trace::rd!(
            "10043f2c",
            (param_1.wrapping_add(0x4c) as *const i32),
            read_unaligned
        );
        let c = mech_trace::rd!("10043f2c", (p.wrapping_add(0x1d) as *const i8), read);
        local_c = (local_c & 0xFFFF0000) | ((c as i16 as u16) as u32);
        let t = mech_trace::rd!(
            "10043f2c",
            (param_1.wrapping_add(0x54) as *const i32),
            read_unaligned
        );
        if t < 20000 {
            let v = mech_trace::rd!(
                "10043f2c",
                (local_8.wrapping_add(0x42c) as *const i32),
                read_unaligned
            );
            if mech_trace::gr!("10043f2c", "100a1210", _DAT_100a1210) <= v as f64 {
                let v2 = mech_trace::rd!(
                    "10043f2c",
                    (local_8.wrapping_add(0x42c) as *const i32),
                    read_unaligned
                );
                if mech_trace::gr!("10043f2c", "100a1218", _DAT_100a1218) < v2 as f64 {
                    local_c &= 0xFFFF0000;
                }
            } else {
                local_c = (local_c & 0xFFFF0000) | 1;
            }
        }
        FUN_1004403d(param_1, local_c as i8);
        let v3 = mech_trace::rd!(
            "10043f2c",
            (local_8.wrapping_add(0x42c) as *const i32),
            read_unaligned
        );
        if v3 < -0x102762 {
            let a8 = mech_trace::rd!(
                "10043f2c",
                (param_1.wrapping_add(8) as *const i32),
                read_unaligned
            );
            let a4 = mech_trace::rd!(
                "10043f2c",
                (param_1.wrapping_add(4) as *const i32),
                read_unaligned
            );
            mech_trace::cr!(
                "10043f2c",
                "sprintf",
                sprintf(
                    local_5c.as_mut_ptr(),
                    &raw const DAT_100afb88,
                    mech_trace::gr!("10043f2c", "100b38fc", DAT_100b38fc),
                    a8,
                    a4,
                )
            );
            FUN_1005dd5a(local_5c.as_mut_ptr());
        }
        mech_trace::rt!(
            "10043f2c",
            mech_trace::rd!(
                "10043f2c",
                (param_1.wrapping_add(0x78) as *const i32),
                read_unaligned
            )
        )
    }
}
