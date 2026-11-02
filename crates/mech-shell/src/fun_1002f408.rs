// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002f408(param_1: u32) -> i32 {
    mech_trace::enter!("1002f408", param_1);
    let mut param_1 = param_1;
    let mut neg = false;
    if (param_1 as i32) < 0 {
        neg = true;
        param_1 = 0u32.wrapping_sub(param_1);
    }

    if param_1 <= 0x4000 {
        // SAFETY: the OS routine is the game's own import; a null argument
        // array with zero arguments is valid.
        unsafe {
            RaiseException(0xC000_0094, 0, 0, core::ptr::null());
        }
    }

    let q: u32;
    let r: u32;
    // SAFETY: 64-bit by 32-bit unsigned divide of EDX:EAX; a fault is the
    // machine's own behavior, as in the original.
    unsafe {
        asm!(
            "div {d}",
            d = in(reg) param_1,
            inout("eax") 0u32 => q,
            inout("edx") 0x4000u32 => r,
            options(nomem, nostack)
        );
    }

    let half = (param_1 >> 1).wrapping_add(((param_1 & 1) != 0) as u32);
    let cmp = (half.wrapping_sub(1) < r) as u32;
    let mut i_var1 = q.wrapping_add(cmp) as i32;
    if neg {
        i_var1 = 0u32.wrapping_sub(i_var1 as u32) as i32;
    }
    mech_trace::rt!("1002f408", i_var1)
}
