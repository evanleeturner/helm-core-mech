// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b1280: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100562b9(param_1: *mut u32) {
    mech_trace::enter!("100562b9", param_1);
    // SAFETY: param_1 points at 13 readable dwords from the caller; the
    // destination is the game's own global buffer, written where the original does.
    unsafe {
        let mut p_var2 = (&raw mut DAT_100b1280).cast::<u32>();
        let mut src = param_1;
        let mut i_var1: i32 = 0xd;
        while i_var1 != 0 {
            mech_trace::wr!(
                "100562b9",
                p_var2,
                write_unaligned,
                mech_trace::rd!("100562b9", src, read_unaligned)
            );
            src = src.wrapping_add(1);
            p_var2 = p_var2.wrapping_add(1);
            i_var1 -= 1;
        }
    }
}
