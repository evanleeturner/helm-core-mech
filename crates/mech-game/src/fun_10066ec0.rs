// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002570(a: u32, b: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10066ec0(divisor: i32, dividend: u32, quotient: *mut u32) -> i32 {
    mech_trace::enter!("10066ec0", divisor, dividend, quotient);
    let failed: i32;
    if divisor == 0 {
        failed = 1;
    } else {
        let q: i32;
        // SAFETY: divisor is nonzero; idiv faults on i32::MIN / -1 exactly as
        // the original's machine division does.
        unsafe {
            core::arch::asm!(
                "cdq",
                "idiv {d:e}",
                d = in(reg) divisor,
                inout("eax") dividend as i32 => q,
                out("edx") _,
            );
        }
        if q > 0x7fff || q < -0x8000 {
            failed = 1;
        } else {
            failed = 0;
        }
    }
    if failed == 0 {
        // SAFETY: the callee is the game's own; quotient points at the
        // caller's output slot.
        unsafe {
            let u_var1 = mech_trace::cr!("10066ec0", "10002570", FUN_10002570(dividend, divisor));
            mech_trace::wr!("10066ec0", quotient, write_unaligned, u_var1);
        }
    }
    mech_trace::rt!("10066ec0", failed)
}
