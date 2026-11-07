// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083790: i32;
    static mut DAT_10083720: i32;
    static mut DAT_10083710: i32;
    static mut _DAT_10083794: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10004620() {
    mech_trace::enter!("10004620");
    // SAFETY: the globals are the game's own memory; the division uses the
    // machine's idiv so a zero divisor faults as in the original.
    unsafe {
        let n = mech_trace::gr!("10004620", "10083720", DAT_10083720).wrapping_mul(100);
        let d = mech_trace::gr!("10004620", "10083710", DAT_10083710);
        let q: i32;
        mech_trace::log::debug!(
            f = "10004620", b = "idiv", a:% = mech_trace::Args([
                mech_trace::__shown!(d),
                mech_trace::__shown!(n),
            ]); "asm.in"
        );
        core::arch::asm!(
            "cdq",
            "idiv {d:e}",
            d = in(reg) d,
            inout("eax") n => q,
            out("edx") _,
            options(nostack)
        );
        mech_trace::log::debug!(
            f = "10004620", b = "idiv", r = "eax", v:% = mech_trace::__shown!(q); "asm.out"
        );
        mech_trace::gw!("10004620", "10083790", DAT_10083790, q);
        mech_trace::gw!(
            "10004620",
            "_DAT_10083794",
            _DAT_10083794,
            mech_trace::gr!("10004620", "10083790", DAT_10083790)
                .wrapping_mul(3)
                .wrapping_add(2)
                / 2
        );
    }
}
