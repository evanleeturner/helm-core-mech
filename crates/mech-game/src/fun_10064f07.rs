// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38ec: u32;
    static mut DAT_100c7e84: u8;
}

#[inline(always)]
unsafe fn idiv(n: i32, d: i32) -> i32 {
    let q: i32;
    // SAFETY: the machine's own idiv; a zero divisor faults as in the original.
    unsafe {
        mech_trace::log::debug!(
            f = "10064f07", b = "idiv", a:% = mech_trace::Args([
                mech_trace::__shown!(d),
                mech_trace::__shown!(n),
            ]); "asm.in"
        );
        asm!(
            "cdq",
            "idiv {d:e}",
            d = in(reg) d,
            inout("eax") n => q,
            out("edx") _,
            options(nomem, nostack)
        );
        mech_trace::log::debug!(
            f = "10064f07", b = "idiv", r = "eax", v:% = mech_trace::__shown!(q); "asm.out"
        );
    }
    q
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10064f07(param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    mech_trace::enter!("10064f07", param_1, param_2, param_3, param_4);
    // SAFETY: the globals are the game's own; the destination is the game's
    // buffer addressed as the original does.
    unsafe {
        let i_var1 = (1u32
            .wrapping_shl((mech_trace::gr!("10064f07", "100c7e84", DAT_100c7e84) & 0x1f) as u32)
            as i32)
            .wrapping_sub(1);
        let p = (param_1.wrapping_mul(3) as u32).wrapping_add(mech_trace::gr!(
            "10064f07",
            "100b38ec",
            DAT_100b38ec
        )) as *mut u8;
        mech_trace::wr!(
            "10064f07",
            p,
            write,
            idiv(param_2.wrapping_mul(0xff), i_var1) as u8
        );
        mech_trace::wr!(
            "10064f07",
            p.byte_add(1),
            write,
            idiv(param_3.wrapping_mul(0xff), i_var1) as u8
        );
        mech_trace::wr!(
            "10064f07",
            p.byte_add(2),
            write,
            idiv(param_4.wrapping_mul(0xff), i_var1) as u8
        );
    }
}
