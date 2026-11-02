// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aaf70: u32;
    static mut DAT_100aaf88: u32;
    static mut DAT_100aaf78: u32;
    static mut DAT_100aaf7c: u32;
    static mut DAT_100aaf74: u32;
    static mut DAT_100aafc0: u32;
    static mut DAT_100aafbc: u32;
    static mut DAT_100aaf80: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10068905(param_1: *mut u32) {
    mech_trace::enter!("10068905", param_1);
    // SAFETY: the globals are the game's own memory; param_1 points at a
    // buffer of at least 0x1a dwords, written as the original does.
    unsafe {
        let mut pu_var2: *const u32 = &raw mut DAT_100aaf70 as *const u32;
        let mut p = param_1;
        let mut i_var1: i32 = 0x1a;
        while i_var1 != 0 {
            mech_trace::wr!(
                "10068905",
                p,
                write_unaligned,
                mech_trace::rd!("10068905", pu_var2, read_unaligned)
            );
            pu_var2 = pu_var2.add(1);
            p = p.add(1);
            i_var1 = i_var1.wrapping_sub(1);
        }
        mech_trace::gw!("10068905", "100aaf88", DAT_100aaf88, 0);
        mech_trace::gw!("10068905", "100aaf78", DAT_100aaf78, 0);
        mech_trace::gw!("10068905", "100aaf7c", DAT_100aaf7c, 1);
        mech_trace::gw!("10068905", "100aaf74", DAT_100aaf74, 1);
        mech_trace::gw!("10068905", "100aafc0", DAT_100aafc0, 0xb00);
        mech_trace::gw!("10068905", "100aafbc", DAT_100aafbc, 1);
        mech_trace::gu!("10068905", "100aaf80", DAT_100aaf80, &=, 0xfffffffb);
    }
}
