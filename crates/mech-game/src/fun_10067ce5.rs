// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10067ce5(param_1: i32, param_2: i32) -> i32 {
    mech_trace::enter!("10067ce5", param_1, param_2);
    // SAFETY: param_1 points at the caller's struct; the divisor at +0x10 is
    // read and the result written at +4 as the original does. The idiv faults
    // on a zero divisor just as the machine does.
    unsafe {
        let base = param_1 as *mut u8;
        let d = mech_trace::rd!(
            "10067ce5",
            base.byte_add(0x10).cast::<i32>(),
            read_unaligned
        );
        let i_var1 = d >> 1;
        let mut local_c: i32;
        mech_trace::log::debug!(
            f = "10067ce5", b = "idiv", a:% = mech_trace::Args([
                mech_trace::__shown!(d),
                mech_trace::__shown!(param_2),
            ]); "asm.in"
        );
        core::arch::asm!(
            "cdq",
            "idiv {d:e}",
            d = in(reg) d,
            inout("eax") param_2 => _,
            out("edx") local_c,
        );
        mech_trace::log::debug!(
            f = "10067ce5", b = "idiv", r = "edx", v:% = mech_trace::__shown!(local_c); "asm.out"
        );
        if i_var1 < local_c {
            local_c = local_c.wrapping_sub(d);
        } else if 0i32.wrapping_sub(local_c) != i_var1 && local_c <= 0i32.wrapping_sub(i_var1) {
            local_c = local_c.wrapping_add(d);
        }
        mech_trace::wr!(
            "10067ce5",
            base.byte_add(4).cast::<i32>(),
            write_unaligned,
            local_c
        );
        mech_trace::rt!("10067ce5", local_c)
    }
}
