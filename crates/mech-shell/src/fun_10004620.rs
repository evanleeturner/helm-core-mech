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
    // SAFETY: the globals are the game's own memory; the division uses the
    // machine's idiv so a zero divisor faults as in the original.
    unsafe {
        let n = DAT_10083720.wrapping_mul(100);
        let d = DAT_10083710;
        let q: i32;
        core::arch::asm!(
            "cdq",
            "idiv {d:e}",
            d = in(reg) d,
            inout("eax") n => q,
            out("edx") _,
            options(nostack)
        );
        DAT_10083790 = q;
        _DAT_10083794 = DAT_10083790.wrapping_mul(3).wrapping_add(2) / 2;
    }
}
