// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a365c: i32;
    static mut DAT_100a3664: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100576a0(index: i32) -> i32 {
    mech_trace::enter!("100576a0", index);
    // SAFETY: the globals are the game's own memory, read by value.
    unsafe {
        if -1 < index && index < mech_trace::gr!("100576a0", "100a365c", DAT_100a365c) {
            return mech_trace::rt!(
                "100576a0",
                (mech_trace::gr!("100576a0", "100a3664", DAT_100a3664) as u32)
                    .wrapping_add((index as u32).wrapping_mul(0x78)) as i32
            );
        }
    }
    mech_trace::rt!("100576a0", 0)
}
