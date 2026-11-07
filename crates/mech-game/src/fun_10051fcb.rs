// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b09e8: u32;
    static mut DAT_101153c0: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10051fcb(param_1: i32) -> i32 {
    mech_trace::enter!("10051fcb", param_1);
    // SAFETY: the globals are the game's own memory; the table index is
    // kept in range by the original's modulo, and the division is done by
    // the machine's idiv as in the original.
    unsafe {
        let i_var1 = (mech_trace::gr!("10051fcb", "100b09e8", DAT_100b09e8).wrapping_mul(4)) as i32;
        mech_trace::gw!(
            "10051fcb",
            "100b09e8",
            DAT_100b09e8,
            mech_trace::gr!("10051fcb", "100b09e8", DAT_100b09e8).wrapping_add(1)
        );
        mech_trace::gw!(
            "10051fcb",
            "100b09e8",
            DAT_100b09e8,
            mech_trace::gr!("10051fcb", "100b09e8", DAT_100b09e8) % 0x7f
        );
        let val = mech_trace::rd!(
            "10051fcb",
            (&raw mut DAT_101153c0)
                .cast::<u8>()
                .offset(i_var1 as isize)
                .cast::<i32>(),
            read_unaligned
        );
        let rem: i32;
        mech_trace::log::debug!(
            f = "10051fcb", b = "idiv", a:% = mech_trace::Args([
                mech_trace::__shown!(param_1),
                mech_trace::__shown!(val),
            ]); "asm.in"
        );
        core::arch::asm!(
            "cdq",
            "idiv {d:e}",
            d = in(reg) param_1,
            inout("eax") val => _,
            out("edx") rem,
        );
        mech_trace::log::debug!(
            f = "10051fcb", b = "idiv", r = "edx", v:% = mech_trace::__shown!(rem); "asm.out"
        );
        mech_trace::rt!("10051fcb", rem)
    }
}
