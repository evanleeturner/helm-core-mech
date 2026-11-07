// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005b547(param_1: i32, param_2: i32) -> u32;
    fn FUN_10056fc4(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    ) -> u32;
}

#[inline(always)]
unsafe fn idiv(a: i32, b: i32) -> i32 {
    let q: i32;
    // SAFETY: the machine's own signed divide; a zero divisor faults as in the original.
    unsafe {
        mech_trace::log::debug!(
            f = "10066dc9", b = "idiv", a:% = mech_trace::Args([
                mech_trace::__shown!(b),
                mech_trace::__shown!(a),
            ]); "asm.in"
        );
        core::arch::asm!(
            "cdq",
            "idiv {b:e}",
            b = in(reg) b,
            inout("eax") a => q,
            out("edx") _,
        );
        mech_trace::log::debug!(
            f = "10066dc9", b = "idiv", r = "eax", v:% = mech_trace::__shown!(q); "asm.out"
        );
    }
    q
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10066dc9(param_1: *mut i32, param_2: i32, param_3: i32) {
    mech_trace::enter!("10066dc9", param_1, param_2, param_3);
    let mut local_10: i32 = 0;
    if !param_1.is_null() && param_2 != 0 {
        // SAFETY: the callees are the game's own; param_1 points at the caller's
        // structure of at least five ints, read where the original reads.
        unsafe {
            let u_var5 = mech_trace::cr!("10066dc9", "1005b547", FUN_1005b547(param_2, param_3));
            let i_var6 = (u_var5 as i32) >> 0x10;
            let u_var5 = u_var5 & 0xffff;
            let i_var1 = mech_trace::rd!("10066dc9", param_1.add(3), read_unaligned);
            let i_var2 = mech_trace::rd!("10066dc9", param_1.add(1), read_unaligned);
            let i_var3 = mech_trace::rd!("10066dc9", param_1.add(4), read_unaligned);
            let i_var4 = mech_trace::rd!("10066dc9", param_1.add(2), read_unaligned);
            let mut local_28: i32 = 0;
            loop {
                let mut local_14: i32 = 0;
                let n = idiv(
                    i_var3.wrapping_sub(i_var4).wrapping_add(u_var5 as i32),
                    u_var5 as i32,
                );
                if !(local_28 < n) {
                    break;
                }
                let mut local_24: i32 = 0;
                loop {
                    let m = idiv(i_var1.wrapping_sub(i_var2).wrapping_add(i_var6), i_var6);
                    if !(local_24 < m) {
                        break;
                    }
                    mech_trace::cr!(
                        "10066dc9",
                        "10056fc4",
                        FUN_10056fc4(param_1, param_2, param_3, local_14, local_10)
                    );
                    local_14 = local_14.wrapping_add(i_var6);
                    local_24 = local_24.wrapping_add(1);
                }
                local_10 = local_10.wrapping_add(u_var5 as i32);
                local_28 = local_28.wrapping_add(1);
            }
        }
    }
}
