// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f64ac: [i8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000d332(param_1: i32, param_2: u32) {
    mech_trace::enter!("1000d332", param_1, param_2);
    let i_var1: u32 = (param_1 as u32).wrapping_mul(0x7c);
    // SAFETY: the global is the game's own array; the offset addresses a
    // record field there, read and written in the original's order.
    unsafe {
        let p = (&raw mut DAT_101f64ac)
            .cast::<u8>()
            .byte_add(i_var1 as usize)
            .cast::<u32>();
        mech_trace::wr!(
            "1000d332",
            p,
            write_unaligned,
            mech_trace::rd!("1000d332", p, read_unaligned) & 0xffff0fff
        );
        mech_trace::wr!(
            "1000d332",
            p,
            write_unaligned,
            mech_trace::rd!("1000d332", p, read_unaligned) | ((param_2 & 0xf).wrapping_shl(0xc))
        );
    }
}
