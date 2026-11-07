// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10067af1(param_1: *mut i32) -> i32 {
    mech_trace::enter!("10067af1", param_1);
    // SAFETY: param_1 points at the caller's int array; the global is the game's own.
    unsafe {
        let p1 = mech_trace::rd!("10067af1", param_1.add(1), read_unaligned);
        let p2 = mech_trace::rd!("10067af1", param_1.add(2), read_unaligned);
        let i_var1 = (p1 as u32).wrapping_sub(p2 as u32) as i32;
        if i_var1 != 0 {
            let p0 = mech_trace::rd!("10067af1", param_1, read_unaligned);
            let num = ((mech_trace::gr!("10067af1", "100b38fc", DAT_100b38fc) as u32)
                .wrapping_sub(p0 as u32))
            .wrapping_mul(i_var1 as u32) as i32;
            let divisor = mech_trace::rd!("10067af1", param_1.add(3), read_unaligned);
            let i_var2: i32;
            mech_trace::log::debug!(
                f = "10067af1", b = "idiv", a:% = mech_trace::Args([
                    mech_trace::__shown!(divisor),
                    mech_trace::__shown!(num),
                ]); "asm.in"
            );
            core::arch::asm!(
                "cdq",
                "idiv {d:e}",
                d = in(reg) divisor,
                inout("eax") num => i_var2,
                out("edx") _,
            );
            mech_trace::log::debug!(
                f = "10067af1", b = "idiv", r = "eax", v:% = mech_trace::__shown!(i_var2); "asm.out"
            );
            if (i_var1 < 0 && i_var2 <= i_var1) || (0 < i_var1 && i_var1 <= i_var2) {
                let v = mech_trace::rd!("10067af1", param_1.add(1), read_unaligned);
                mech_trace::wr!("10067af1", param_1.add(2), write_unaligned, v);
            } else {
                let v = (mech_trace::rd!("10067af1", param_1.add(2), read_unaligned) as u32)
                    .wrapping_add(i_var2 as u32) as i32;
                mech_trace::wr!("10067af1", param_1.add(2), write_unaligned, v);
            }
        }
        mech_trace::wr!(
            "10067af1",
            param_1,
            write_unaligned,
            mech_trace::gr!("10067af1", "100b38fc", DAT_100b38fc)
        );
        mech_trace::rt!(
            "10067af1",
            mech_trace::rd!("10067af1", param_1.add(2), read_unaligned)
        )
    }
}
