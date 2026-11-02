// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aaf70: [i32; 26];
    static mut DAT_100aaf74: i32;
    static mut DAT_100aaf78: i32;
    static mut DAT_100aaf7c: i32;
    static mut DAT_100aaf80: i32;
    static mut DAT_100aaf88: i32;
    static mut DAT_100aafbc: i32;
    static mut DAT_100aafc0: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10068962(param_1: *mut i32) {
    mech_trace::enter!("10068962", param_1);
    // SAFETY: the globals are the game's own memory; param_1 points at the
    // caller's buffer of at least 0x1a ints, written as the original does.
    unsafe {
        let mut p_var2 = (&raw mut DAT_100aaf70).cast::<i32>() as *const i32;
        let mut dst = param_1;
        let mut i_var1: i32 = 0x1a;
        while i_var1 != 0 {
            mech_trace::wr!(
                "10068962",
                dst,
                write_unaligned,
                mech_trace::rd!("10068962", p_var2, read_unaligned)
            );
            p_var2 = p_var2.add(1);
            dst = dst.add(1);
            i_var1 = i_var1.wrapping_sub(1);
        }
        mech_trace::gw!("10068962", "100aaf88", DAT_100aaf88, 0);
        mech_trace::gw!("10068962", "100aaf78", DAT_100aaf78, 0);
        mech_trace::gw!("10068962", "100aaf7c", DAT_100aaf7c, 1);
        mech_trace::gw!("10068962", "100aaf74", DAT_100aaf74, 1);
        mech_trace::gw!("10068962", "100aafc0", DAT_100aafc0, 0xb00);
        mech_trace::gw!("10068962", "100aafbc", DAT_100aafbc, 1);
        mech_trace::gw!(
            "10068962",
            "100aaf80",
            DAT_100aaf80,
            mech_trace::gr!("10068962", "100aaf80", DAT_100aaf80) & (0xfffffffbu32 as i32)
        );
    }
}
