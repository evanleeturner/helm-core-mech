// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009fbc8: i8;
    static mut DAT_1009fbdc: i8;
    static mut DAT_1009fb98: i8;
    static mut DAT_1009fbb0: i8;
    static mut DAT_1009fbf0: i8;
    static mut DAT_1009fc04: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FID_conflict__fwprintf(file: *mut u8, format: *const u8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10053470(param_1: *mut u8, param_2: i32) {
    mech_trace::enter!("10053470", param_1, param_2);
    let flags = (param_2.wrapping_add(8)) as *const u32;
    // SAFETY: param_2 points at the caller's structure; the format strings
    // and fwprintf are the game's own, and the reads and calls keep the
    // original's order.
    unsafe {
        if mech_trace::rd!("10053470", flags, read_unaligned) & 1 == 0 {
            if mech_trace::rd!("10053470", flags, read_unaligned) & 0x10 != 0 {
                mech_trace::cr!(
                    "10053470",
                    "FID_conflict__fwprintf",
                    FID_conflict__fwprintf(param_1, (&raw mut DAT_1009fbb0) as *const u8)
                );
            }
        } else {
            mech_trace::cr!(
                "10053470",
                "FID_conflict__fwprintf",
                FID_conflict__fwprintf(param_1, (&raw mut DAT_1009fb98) as *const u8)
            );
        }
        let fmt: *const u8;
        if mech_trace::rd!("10053470", flags, read_unaligned) & 2 == 0 {
            if mech_trace::rd!("10053470", flags, read_unaligned) & 0x20 != 0 {
                fmt = (&raw mut DAT_1009fbdc) as *const u8;
                mech_trace::cr!(
                    "10053470",
                    "FID_conflict__fwprintf",
                    FID_conflict__fwprintf(param_1, fmt)
                );
            }
        } else {
            fmt = (&raw mut DAT_1009fbc8) as *const u8;
            mech_trace::cr!(
                "10053470",
                "FID_conflict__fwprintf",
                FID_conflict__fwprintf(param_1, fmt)
            );
        }
        if mech_trace::rd!("10053470", flags, read_unaligned) & 4 == 0 {
            if mech_trace::rd!("10053470", flags, read_unaligned) & 0x40 != 0 {
                mech_trace::cr!(
                    "10053470",
                    "FID_conflict__fwprintf",
                    FID_conflict__fwprintf(param_1, (&raw mut DAT_1009fc04) as *const u8)
                );
            }
            return;
        }
        mech_trace::cr!(
            "10053470",
            "FID_conflict__fwprintf",
            FID_conflict__fwprintf(param_1, (&raw mut DAT_1009fbf0) as *const u8)
        );
    }
}
