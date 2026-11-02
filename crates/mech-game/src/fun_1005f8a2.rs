// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10113170: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005f8a2(param_1: i16, param_2: u16) {
    mech_trace::enter!("1005f8a2", param_1, param_2);
    let off = (param_1 as i32).wrapping_mul(0xe).wrapping_add(4);
    // SAFETY: DAT_10113170 is the game's own table; the 16-bit slot at the
    // computed byte offset is written where the original writes it.
    unsafe {
        mech_trace::wr!(
            "1005f8a2",
            (&raw mut DAT_10113170)
                .cast::<u8>()
                .byte_offset(off as isize)
                .cast::<u16>(),
            write_unaligned,
            param_2
        );
    }
}
