// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FUN_10063528(param_1: i32, param_2: i32, param_3: i32) {
    mech_trace::enter!("10063528", param_1, param_2, param_3);
    if -1 < param_3 && param_3 < 3 {
        // SAFETY: param_1 points at the game's object whose field at 0x54 is a
        // table of 0x40-byte entries; the entry's field at 0xc is written.
        unsafe {
            let base = mech_trace::rd!(
                "10063528",
                ((param_1.wrapping_add(0x54)) as *const i32),
                read_unaligned
            );
            let addr = base
                .wrapping_add(param_2.wrapping_mul(0x40))
                .wrapping_add(0xc);
            mech_trace::wr!("10063528", (addr as *mut i32), write_unaligned, param_3);
        }
    }
}
