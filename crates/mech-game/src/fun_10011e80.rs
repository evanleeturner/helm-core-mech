// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1d8: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10011e80(param_1: i32) -> i32 {
    mech_trace::enter!("10011e80", param_1);
    // SAFETY: the global is the game's own memory; when non-zero it holds the
    // address of a structure whose field at offset 0x10 points to an array of
    // at least eight i32 values.
    unsafe {
        if mech_trace::gr!("10011e80", "100aa1d8", DAT_100aa1d8) == 0 {
            return mech_trace::rt!("10011e80", param_1.wrapping_add(-1));
        }
        let mut local_8: i32 = 0;
        while local_8 < 8 {
            let base = mech_trace::rd!(
                "10011e80",
                ((mech_trace::gr!("10011e80", "100aa1d8", DAT_100aa1d8) as u32).wrapping_add(0x10)
                    as *const u32),
                read_unaligned
            );
            let addr = base.wrapping_add((local_8 as u32).wrapping_mul(4)) as *const i32;
            if mech_trace::rd!("10011e80", addr, read_unaligned) == param_1 {
                return mech_trace::rt!("10011e80", local_8);
            }
            local_8 = local_8.wrapping_add(1);
        }
        mech_trace::rt!("10011e80", 0)
    }
}
