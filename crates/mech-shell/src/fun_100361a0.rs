// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10093b2c: *mut u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100361a0(param_1: i32) {
    mech_trace::enter!("100361a0", param_1);
    // SAFETY: the global is the game's own table pointer; the entry at
    // index * 0x40 + 0x14 is written as the original does.
    unsafe {
        let base = mech_trace::gr!("100361a0", "10093b2c", DAT_10093b2c);
        mech_trace::wr!(
            "100361a0",
            base.byte_offset((param_1.wrapping_mul(0x40).wrapping_add(0x14)) as isize)
                .cast::<i32>(),
            write_unaligned,
            -1
        );
    }
}
